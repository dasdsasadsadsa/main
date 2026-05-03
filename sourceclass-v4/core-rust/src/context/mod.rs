//! Context Module
//! 
//! Prepares minimized context for LLM explanations.

pub mod planner;

pub use planner::ContextPlanner;

/// Context prepared for explaining a specific element
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExplainContext {
    pub file_path: String,
    pub element: crate::map::schema::Symbol,
    pub file_content: String,
    pub project_skeleton: crate::map::schema::ProjectSkeleton,
    pub centrality: crate::map::schema::Centrality,
}
