use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DistanceMetric {
    L2,
    EUCLIDEAN,
    COSINE,
    DOT,
}

impl From<String> for DistanceMetric {
    fn from(s: String) -> Self {
        match s.as_str() {
            "l2" => DistanceMetric::L2,
            "euclidean" => DistanceMetric::EUCLIDEAN,
            "cosine" => DistanceMetric::COSINE,
            "dot" => DistanceMetric::DOT,
            _ => DistanceMetric::L2,
        }
    }
}

impl std::fmt::Display for DistanceMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistanceMetric::L2 => write!(f, "l2"),
            DistanceMetric::EUCLIDEAN => write!(f, "euclidean"),
            DistanceMetric::COSINE => write!(f, "cosine"),
            DistanceMetric::DOT => write!(f, "dot"),
        }
    }
}
