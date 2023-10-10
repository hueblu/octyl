pub mod tree;

use std::collections::HashMap;
pub use tree::ComponentTree;

pub struct Components {
    components: HashMap<String, Box<dyn Component>>,
}

pub trait Component {
    fn init(self);
}

pub trait ComponentOptions {}

impl Components {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
}
