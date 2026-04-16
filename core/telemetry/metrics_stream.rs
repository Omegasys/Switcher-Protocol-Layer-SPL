use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct MetricSample {
    pub timestamp: u64,
    pub value: f64,
    pub metric_type: String,
}

pub struct MetricsStream {
    pub buffer: VecDeque<MetricSample>,
    pub max_size: usize,
}

impl MetricsStream {
    pub fn new(max_size: usize) -> Self {
        Self {
            buffer: VecDeque::new(),
            max_size,
        }
    }

    pub fn push(&mut self, sample: MetricSample) {
        if self.buffer.len() == self.max_size {
            self.buffer.pop_front();
        }
        self.buffer.push_back(sample);
    }

    pub fn latest(&self) -> Option<&MetricSample> {
        self.buffer.back()
    }

    pub fn average(&self) -> f64 {
        if self.buffer.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.buffer.iter().map(|s| s.value).sum();
        sum / self.buffer.len() as f64
    }
}
