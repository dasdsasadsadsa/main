use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectIR {
    pub project: ProjectObject,
    pub files: Vec<FileObject>,
    pub entrypoints: Vec<EntrypointObject>,
    pub dependencies: Vec<DependencyObject>,
    pub file_roles: Vec<FileRoleObject>,
    pub risk_flags: Vec<RiskFlagObject>,
    pub edit_zones: Vec<EditZoneObject>,
    pub structure_graph: StructureGraph,
    pub token_estimate: usize,
    pub context_plan: Vec<ContextPlanItem>,
    pub summary_stats: SummaryStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectObject {
    pub name: String,
    pub path: String,
    pub languages: Vec<String>,
    pub dominant_language: String,
    pub detected_frameworks: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileObject {
    pub path: String,
    pub absolute_path: String,
    pub extension: String,
    pub language: String,
    pub size_bytes: u64,
    pub line_count: usize,
    pub hash: String,
    pub is_binary: bool,
    pub is_generated: bool,
    pub is_test: bool,
    pub is_config: bool,
    pub ignored: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDetection {
    pub file: String,
    pub language: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntrypointObject {
    pub file: String,
    pub kind: String,
    pub confidence: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyObject {
    pub source: String,
    pub manager: String,
    pub dependencies: Vec<DependencyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub name: String,
    pub version: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRoleObject {
    pub file: String,
    pub role: String,
    pub importance: String,
    pub confidence: f64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFlagObject {
    pub file: String,
    pub risk_type: String,
    pub severity: String,
    pub confidence: f64,
    pub reason: String,
    pub safe_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditZoneObject {
    pub file: String,
    pub zone: String,
    pub reason: String,
    pub risk_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextPlanItem {
    pub rank: usize,
    pub file: String,
    pub priority: String,
    pub reason: String,
    pub estimated_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryStats {
    pub total_files: usize,
    pub analyzed_files: usize,
    pub ignored_files: usize,
    pub large_files: usize,
    pub risk_count: usize,
}
