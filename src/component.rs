use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::atomic::AtomicU64,
};

use anyhow::Result;
use ratatui::{prelude::Rect, Frame};
use tokio::sync::mpsc;

use crate::{
    action::{Action, Tags},
    event::Event,
    identifier::Id,
    view::View,
    Context,
};

static COMPONENT_ID: AtomicU64 = AtomicU64::new(0);

pub struct Component {
    pub register: Box<dyn FnOnce(Context) -> Result<()>>,

    pub init: Box<dyn Fn() -> HashMap<String, Box<dyn Any>>>,
    pub subscriptions: Vec<Tags>,

    pub dispatch: Box<dyn Fn(Event) -> Option<Action>>,
    pub update: Box<dyn FnMut(&mut ComponentState, Action, Context) -> Option<Action>>,
    pub draw: Box<dyn FnMut(&mut ComponentState, &mut Frame, Rect)>,
}

pub struct ComponentState {
    pub component_id: Id,

    pub fields: Option<HashMap<String, Box<dyn Any>>>,
}

pub struct ComponentCollection {
    components: HashMap<Id, Box<Component>>,
}

impl Component {
    pub fn new(
        register: Box<dyn FnOnce(Context) -> Result<()>>,

        init: Box<dyn Fn() -> HashMap<String, Box<dyn Any>>>,
        subscriptions: Vec<Tags>,

        dispatch: Box<dyn Fn(Event) -> Option<Action>>,
        update: Box<dyn FnMut(&mut ComponentState, Action, Context) -> Option<Action>>,
        draw: Box<dyn FnMut(&mut ComponentState, &mut Frame, Rect)>,
    ) -> Self {
        Component {
            register,
            init,
            subscriptions,
            dispatch,
            update,
            draw,
        }
    }
}

impl ComponentState {
    pub fn empty(component_id: Id) -> Self {
        Self {
            component_id,
            fields: None,
        }
    }

    pub fn new(component_id: Id, fields: HashMap<String, Box<dyn Any>>) -> Self {
        Self {
            component_id,
            fields: Some(fields),
        }
    }
}

impl ComponentCollection {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    // registers the component, returning the new ID if successful
    pub fn register_component(&mut self, component: Component) -> Result<Id> {
        let id = Id::new(&COMPONENT_ID);
        self.components.insert(id.clone(), Box::new(component));

        Ok(id)
    }

    pub fn dispatch_event(
        &self,
        event: Event,
        id: &Id,
        action_tx: &mpsc::UnboundedSender<Action>,
    ) -> Result<()> {
        if let Some(component) = self.components.get(id)
            && let Some(action) = (component.dispatch)(event)
        {
            action_tx.send(action);
        }

        Ok(())
    }

    // create a new mounted component from the id of a registered component
    pub fn new_mounted(&self, id: Id) -> Result<ComponentState> {
        if let Some(component) = self.components.get(&id) {
            Ok(ComponentState::new(id, (component.init)()))
        } else {
            Err(anyhow::anyhow!("failed to find component from id"))
        }
    }
}
