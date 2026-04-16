#[derive(Debug, Clone)]
pub enum IntentType {
    OptimizeLatency,
    ReduceCongestion,
    IncreaseReliability,
    MinimizeCost,
}

#[derive(Debug, Clone)]
pub struct Intent {
    pub id: String,
    pub intent_type: IntentType,
    pub target: Option<String>,
    pub priority: f64,
}

pub struct IntentBuilder;

impl IntentBuilder {
    pub fn new(id: &str, intent_type: IntentType) -> Intent {
        Intent {
            id: id.to_string(),
            intent_type,
            target: None,
            priority: 1.0,
        }
    }

    pub fn with_target(mut intent: Intent, target: &str) -> Intent {
        intent.target = Some(target.to_string());
        intent
    }

    pub fn with_priority(mut intent: Intent, priority: f64) -> Intent {
        intent.priority = priority;
        intent
    }
}
