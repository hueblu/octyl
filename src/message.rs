use std::any::Any;
use std::fmt::Debug;

use crossterm::event::Event;

pub trait Message: Send + Sync + Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any_boxed(self: Box<Self>) -> Box<dyn Any>;
    fn _clone_box(&self) -> Box<dyn Message>;
    fn _is_equal(&self, other: &dyn Message) -> bool;
}

impl<T> Message for T
where
    T: 'static + Any + PartialEq + Sized + Clone + Debug + Send + Sync,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any_boxed(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn _clone_box(&self) -> Box<dyn Message> {
        Box::new(self.clone())
    }

    fn _is_equal(&self, other: &dyn Message) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }
}

impl Clone for Box<dyn Message> {
    fn clone(&self) -> Self {
        self._clone_box()
    }
}

impl PartialEq for Box<dyn Message> {
    fn eq(&self, other: &Self) -> bool {
        self._is_equal(other)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum AppMessage {
    Quit,
    Event(Event),
    Suspend,
    Resume,
}
