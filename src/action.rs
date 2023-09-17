use std::{any::Any, fmt::Debug};

use serde::Deserialize;
use serde_derive::Serialize;

pub trait Action: Send + Sync + Any + ActionTrait {
    fn noop() -> Self
    where
        Self: Sized;

    fn is_noop(&self) -> bool
    where
        Self: Sized + Clone;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_any_boxed(self: Box<Self>) -> Box<dyn Any>
    where
        Self: Sized;
}

pub trait ActionTrait {
    fn clone_box(&self) -> Box<dyn Action>;
    fn is_equal(&self, other: &dyn Action) -> bool;
}

impl<T> ActionTrait for T
where
    T: 'static + Action + Clone + PartialEq + Debug,
{
    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn is_equal(&self, other: &dyn Action) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            self == other
        } else {
            false
        }
    }
}

impl Clone for Box<dyn Action> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn Action> {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(&**other)
    }
}

impl Debug for Box<dyn Action> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Box<dyn Action>")
    }
}

impl<'de> Deserialize<'de> for Box<dyn Action> {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppAction {
    Quit,
    Tick,
    RenderTick,
    Resize(u16, u16),
    Update,
    Noop,
}

impl Action for AppAction {
    fn noop() -> Self {
        AppAction::Noop
    }

    fn is_noop(&self) -> bool {
        *self == AppAction::Noop
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any_boxed(self: Box<Self>) -> Box<dyn Any>
    where
        Self: Sized,
    {
        self
    }
}
