#[derive(Debug, Clone)]
pub struct Segment {
    pub node: String,
}

pub struct SegmentRouting;

impl SegmentRouting {
    pub fn encode_path(path: Vec<String>) -> Vec<Segment> {
        path.into_iter().map(|n| Segment { node: n }).collect()
    }

    pub fn decode_path(segments: Vec<Segment>) -> Vec<String> {
        segments.into_iter().map(|s| s.node).collect()
    }
}
