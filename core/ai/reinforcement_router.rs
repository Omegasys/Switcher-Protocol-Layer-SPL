use crate::ai::policy_engine::{PolicyEngine, NetworkState, RoutingDecision};
use std::collections::HashMap;

pub struct ReinforcementRouter {
    pub q_table: HashMap<String, f64>,
    pub policy: PolicyEngine,
    pub learning_rate: f64,
    pub discount: f64,
}

impl ReinforcementRouter {
    pub fn new() -> Self {
        Self {
            q_table: HashMap::new(),
            policy: PolicyEngine::new(),
            learning_rate: 0.1,
            discount: 0.9,
        }
    }

    pub fn reward(&self, state: &NetworkState) -> f64 {
        self.policy.evaluate(state)
    }

    pub fn update_q(&mut self, state_key: String, reward: f64) {
        let entry = self.q_table.entry(state_key).or_insert(0.0);
        *entry = *entry + self.learning_rate * (reward - *entry);
    }

    pub fn select_action(&self, candidates: Vec<RoutingDecision>) -> Option<RoutingDecision> {
        self.policy.choose_best(candidates)
    }
}
