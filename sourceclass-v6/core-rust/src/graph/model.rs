use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub role: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub relationship: String,
    pub confidence: f64,
}
