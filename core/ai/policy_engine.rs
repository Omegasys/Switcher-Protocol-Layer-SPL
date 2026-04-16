use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NetworkState {
    pub congestion: f64,
    pub latency: f64,
    pub packet_loss: f64,
}

#[derive(Debug, Clone)]
pub struct RoutingDecision {
    pub path_id: String,
    pub score: f64,
}

pub struct PolicyEngine {
    pub weights: HashMap<String, f64>,
}

impl PolicyEngine {
    pub fn new() -> Self {
        let mut weights = HashMap::new();
        weights.insert("latency".into(), 0.5);
        weights.insert("congestion".into(), 0.3);
        weights.insert("loss".into(), 0.2);

        Self { weights }
    }

    pub fn evaluate(&self, state: &NetworkState) -> f64 {
        let l = self.weights["latency"] * state.latency;
        let c = self.weights["congestion"] * state.congestion;
        let p = self.weights["loss"] * state.packet_loss;

        1.0 - (l + c + p)
    }

    pub fn choose_best(&self, candidates: Vec<RoutingDecision>) -> Option<RoutingDecision> {
        candidates.into_iter().max_by(|a, b| {
            a.score
                .partial_cmp(&b.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}
