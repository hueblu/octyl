use std::collections::HashMap;

use anyhow::{bail, Result};
use crossterm::event::KeyEvent;

use crate::action::Action;

#[derive(Default)]
pub struct KeyTreeNode {
    pub desc: Option<String>,
    pub children: HashMap<KeyEvent, KeyTreeNode>,

    // if this is a leaf node, this is the action to be executed
    // along with the behavior for termination
    action: Option<(Box<dyn Action>, TerminationBehavior)>,
}

pub enum TerminationBehavior {
    Immediate,
    Delayed(),
}

impl KeyTreeNode {
    pub fn get_action(&self, keys: &mut Vec<KeyEvent>) -> Option<&KeyTreeNode> {
        match self.children.get(&keys.remove(0)) {
            Some(child) => child.get_action(keys),
            None => None,
        }
    }

    pub fn add_map(&mut self, key: Vec<KeyEvent>, value: Box<dyn Action>) -> Result<()> {
        if key.is_empty() {
            bail!("Cannot add empty key");
        }

        if let Some(first) = key.first() {
            if key.len() == 1 {
                self.children
                    .insert(*first, KeyTreeNode::from_action(value));
            } else {
                let next_node = self.children.entry(*first).or_default();
                let rest_of_path = key[1..].to_vec();

                next_node.add_map(rest_of_path, value)?;
            }
        }

        Ok(())
    }

    fn from_action(value: Box<dyn Action>) -> KeyTreeNode {
        Self {
            desc: None,
            children: HashMap::new(),
            action: Some((value, TerminationBehavior::Immediate)),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn keytreenode_simpl() {}
}
