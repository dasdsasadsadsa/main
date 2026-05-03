//! Map Schema - Data structures for the project map

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Complete project map - the core data structure of SourceClass V4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMapIR {
    pub root_path: String,
    pub files: Vec<FileNode>,
    pub directories: Vec<DirectoryNode>,
    pub symbols: Vec<Symbol>,
    pub relationships: Vec<Relationship>,
    pub stats: MapStats,
}

/// A file in the project map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileNode {
    pub id: String,
    pub path: String,
    pub relative_path: String,
    pub extension: String,
    pub language: String,
    pub size_bytes: u64,
    pub line_count: usize,
    pub role: FileRole,
    pub centrality_score: f64,
    pub symbol_count: usize,
    pub hash: String,
}

/// A directory in the project map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryNode {
    pub id: String,
    pub path: String,
    pub relative_path: String,
    pub children_count: usize,
    pub file_count: usize,
    pub dir_count: usize,
    pub role_hint: Option<String>,
}

/// A symbol extracted from code (class, function, method, variable, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub id: String,
    pub name: String,
    pub kind: SymbolKind,
    pub file_id: String,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub signature: String,
    pub docstring: Option<String>,
    pub parent_id: Option<String>,
    pub visibility: Visibility,
    pub is_async: bool,
    pub is_static: bool,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub type_parameters: Vec<String>,
}

/// Parameter of a function/method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<String>,
    pub default_value: Option<String>,
}

/// Kind of symbol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SymbolKind {
    Class,
    Interface,
    Trait,
    Struct,
    Enum,
    Function,
    Method,
    Constructor,
    Property,
    Field,
    Variable,
    Constant,
    TypeAlias,
    Import,
    Module,
    Namespace,
    Package,
}

/// Visibility modifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

/// Role of a file in the project
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileRole {
    Entry,          // Main entry point
    Brain,          // Core logic/analysis
    Interface,      // API/routes/commands
    Connector,      // External service integration
    Config,         // Configuration
    Utility,        // Helper functions
    DataModel,      // Schema/types/models
    Output,         // Report/rendering
    Test,           // Test files
    Documentation,  // Docs
    Unknown,
}

/// Relationship between two elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub from_id: String,
    pub to_id: String,
    pub kind: RelationshipKind,
    pub confidence: f64,
}

/// Kind of relationship
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipKind {
    Imports,
    Extends,
    Implements,
    Calls,
    Uses,
    Contains,
    Exports,
    DependsOn,
    References,
}

/// Statistics about the map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapStats {
    pub total_files: usize,
    pub total_directories: usize,
    pub total_symbols: usize,
    pub total_lines: usize,
    pub total_size_bytes: u64,
    pub languages: HashMap<String, usize>,
    pub roles: HashMap<String, usize>,
    pub avg_centrality: f64,
}

/// Summary of a single file for quick display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSummary {
    pub file: FileNode,
    pub symbols: Vec<Symbol>,
    pub imports: Vec<ImportInfo>,
    pub dependencies: Vec<DependencyInfo>,
    pub dependents: Vec<DependencyInfo>,
    pub one_line_summary: String,
}

/// Import information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportInfo {
    pub source: String,
    pub names: Vec<String>,
    pub is_external: bool,
    pub is_relative: bool,
}

/// Dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub target_id: String,
    pub target_path: String,
    pub relationship: RelationshipKind,
}

/// Compact project skeleton for LLM context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSkeleton {
    pub root_name: String,
    pub top_level_dirs: Vec<String>,
    pub key_files: Vec<KeyFile>,
    pub symbol_index: Vec<SymbolIndexEntry>,
}

/// Key file in the project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyFile {
    pub path: String,
    pub role: FileRole,
    pub centrality: f64,
}

/// Compact symbol index entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolIndexEntry {
    pub id: String,
    pub name: String,
    pub kind: SymbolKind,
    pub file_path: String,
    pub centrality: f64,
}

/// Centrality measurement for an element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Centrality {
    pub score: f64,
    pub level: CentralityLevel,
    pub incoming_edges: usize,
    pub outgoing_edges: usize,
    pub description: String,
}

/// Centrality level classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CentralityLevel {
    High,    // Critical to system
    Medium,  // Moderate impact
    Low,     // Incidental
}

impl Default for FileRole {
    fn default() -> Self {
        FileRole::Unknown
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}

impl Default for CentralityLevel {
    fn default() -> Self {
        CentralityLevel::Low
    }
}
