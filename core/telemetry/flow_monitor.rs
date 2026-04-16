use std::collections::HashMap;

pub struct FlowMonitor {
    pub flow_stats: HashMap<String, (u64, f64)>, // flow_id -> (packets, latency)
}

impl FlowMonitor {
    pub fn new() -> Self {
        Self {
            flow_stats: HashMap::new(),
        }
    }

    pub fn record_packet(&mut self, flow_id: &str, latency: f64) {
        let entry = self.flow_stats.entry(flow_id.to_string()).or_insert((0, 0.0));
        entry.0 += 1;
        entry.1 += latency;
    }

    pub fn avg_latency(&self, flow_id: &str) -> f64 {
        self.flow_stats
            .get(flow_id)
            .map(|(count, total)| if *count > 0 { total / *count as f64 } else { 0.0 })
            .unwrap_or(0.0)
    }

    pub fn packet_count(&self, flow_id: &str) -> u64 {
        self.flow_stats.get(flow_id).map(|(c, _)| *c).unwrap_or(0)
    }
}
