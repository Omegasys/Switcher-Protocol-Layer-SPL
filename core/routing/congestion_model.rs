use std::collections::HashMap;

pub struct CongestionModel {
    pub link_load: HashMap<String, f64>,
}

impl CongestionModel {
    pub fn new() -> Self {
        Self {
            link_load: HashMap::new(),
        }
    }

    pub fn update_load(&mut self, link: &str, load: f64) {
        self.link_load.insert(link.to_string(), load);
    }

    pub fn cost(&self, link: &str) -> f64 {
        self.link_load.get(link).cloned().unwrap_or(1.0)
    }

    pub fn is_congested(&self, link: &str) -> bool {
        self.cost(link) > 0.8
    }
}
