use std::collections::HashMap;

use crate::control_plane::intent_api::Intent;
use crate::control_plane::policy_dispatcher::PolicyDispatcher;

pub struct Controller {
    pub nodes: HashMap<String, NodeState>,
    pub dispatcher: PolicyDispatcher,
}

#[derive(Debug, Clone)]
pub struct NodeState {
    pub id: String,
    pub active: bool,
    pub load: f64,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            dispatcher: PolicyDispatcher::new(),
        }
    }

    pub fn register_node(&mut self, id: &str) {
        self.nodes.insert(
            id.to_string(),
            NodeState {
                id: id.to_string(),
                active: true,
                load: 0.0,
            },
        );
    }

    pub fn update_load(&mut self, id: &str, load: f64) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.load = load;
        }
    }

    pub fn apply_intent(&mut self, intent: Intent) {
        self.dispatcher.dispatch(&intent, &mut self.nodes);
    }
}
