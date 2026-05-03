//! Project Map Builder
//! 
//! Builds the complete project map by scanning files, extracting symbols,
//! and computing relationships.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use rayon::prelude::*;

use crate::map::schema::*;
use crate::symbol::extractor::SymbolExtractor;
use crate::role::classifier::RoleClassifier;

/// Main project map builder
pub struct ProjectMap {
    root_path: PathBuf,
    files: Vec<FileNode>,
    directories: Vec<DirectoryNode>,
    symbols: Vec<Symbol>,
    relationships: Vec<Relationship>,
    file_index: HashMap<String, usize>,
    symbol_index: HashMap<String, usize>,
}

impl ProjectMap {
    /// Create a new project map builder
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            files: Vec::new(),
            directories: Vec::new(),
            symbols: Vec::new(),
            relationships: Vec::new(),
            file_index: HashMap::new(),
            symbol_index: HashMap::new(),
        }
    }
    
    /// Build the complete project map
    pub fn build(&mut self) -> Result<(), String> {
        // Scan and process files
        self.scan_files()?;
        
        // Extract symbols from all files
        self.extract_symbols()?;
        
        // Classify file roles
        self.classify_roles();
        
        // Build relationships
        self.build_relationships();
        
        // Calculate centrality scores
        self.compute_centrality();
        
        Ok(())
    }
    
    /// Scan directory for files
    fn scan_files(&mut self) -> Result<(), String> {
        let mut dir_map: HashMap<String, usize> = HashMap::new();
        
        // Use walkdir to traverse the directory tree
        let entries = walkdir::WalkDir::new(&self.root_path)
            .into_iter()
            .filter_entry(|e| !is_ignored(e.path()))
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>();
        
        // Process directories first
        for (idx, entry) in entries.iter().enumerate() {
            if entry.path().is_dir() {
                let rel_path = entry.path()
                    .strip_prefix(&self.root_path)
                    .unwrap_or(entry.path())
                    .to_string_lossy()
                    .to_string();
                
                let dir_node = DirectoryNode {
                    id: format!("dir_{}", idx),
                    path: entry.path().to_string_lossy().to_string(),
                    relative_path: rel_path.clone(),
                    children_count: 0,
                    file_count: 0,
                    dir_count: 0,
                    role_hint: None,
                };
                
                dir_map.insert(rel_path, self.directories.len());
                self.directories.push(dir_node);
            }
        }
        
        // Process files in parallel
        let file_results: Vec<_> = entries.par_iter()
            .filter(|e| e.path().is_file())
            .filter_map(|entry| {
                match self.process_file_entry(entry) {
                    Ok(file_node) => Some(file_node),
                    Err(_) => None,
                }
            })
            .collect();
        
        self.files = file_results;
        
        // Build file index
        for (idx, file) in self.files.iter().enumerate() {
            self.file_index.insert(file.id.clone(), idx);
        }
        
        // Update directory counts
        self.update_dir_counts(&dir_map);
        
        Ok(())
    }
    
    /// Process a single file entry
    fn process_file_entry(&self, entry: &walkdir::DirEntry) -> Result<FileNode, String> {
        let path = entry.path();
        let rel_path = path
            .strip_prefix(&self.root_path)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        
        let metadata = entry.metadata()
            .map_err(|e| format!("Failed to read metadata: {}", e))?;
        
        let extension = path.extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();
        
        let language = detect_language_from_extension(&extension);
        
        let content = std::fs::read_to_string(path).unwrap_or_default();
        let line_count = content.lines().count();
        
        let hash = compute_hash(&content);
        
        Ok(FileNode {
            id: format!("file_{}", rel_path.replace('/', "_").replace('\\', "_")),
            path: path.to_string_lossy().to_string(),
            relative_path: rel_path.clone(),
            extension,
            language,
            size_bytes: metadata.len(),
            line_count,
            role: FileRole::Unknown,
            centrality_score: 0.0,
            symbol_count: 0,
            hash,
        })
    }
    
    /// Extract symbols from all files
    fn extract_symbols(&mut self) -> Result<(), String> {
        let extractor = SymbolExtractor::new();
        
        // Extract symbols in parallel
        let all_symbols: Vec<Vec<Symbol>> = self.files.par_iter()
            .map(|file| {
                let path = Path::new(&file.path);
                extractor.extract_from_file(path).unwrap_or_default()
            })
            .collect();
        
        // Flatten and index symbols
        for symbols in all_symbols {
            for symbol in symbols {
                self.symbol_index.insert(symbol.id.clone(), self.symbols.len());
                self.symbols.push(symbol);
            }
        }
        
        // Update file symbol counts
        self.update_file_symbol_counts();
        
        Ok(())
    }
    
    /// Classify file roles
    fn classify_roles(&mut self) {
        let classifier = RoleClassifier::new();
        
        for file in &mut self.files {
            file.role = classifier.classify(file);
        }
    }
    
    /// Build relationships between elements
    fn build_relationships(&mut self) {
        // Import relationships
        self.build_import_relationships();
        
        // Inheritance/implementation relationships
        self.build_inheritance_relationships();
        
        // Call relationships (heuristic)
        self.build_call_relationships();
    }
    
    /// Build import-based relationships
    fn build_import_relationships(&mut self) {
        for symbol in &self.symbols {
            if symbol.kind == SymbolKind::Import {
                // Try to find the target file
                if let Some(target_id) = self.find_import_target(&symbol.name) {
                    self.relationships.push(Relationship {
                        from_id: symbol.file_id.clone(),
                        to_id: target_id,
                        kind: RelationshipKind::Imports,
                        confidence: 0.8,
                    });
                }
            }
        }
    }
    
    /// Build inheritance relationships
    fn build_inheritance_relationships(&mut self) {
        // Look for extends/implements patterns in symbol signatures
        for symbol in &self.symbols {
            if symbol.kind == SymbolKind::Class || symbol.kind == SymbolKind::Interface {
                // Parse signature for extends/implements
                if let Some(parent) = extract_parent_class(&symbol.signature) {
                    if let Some(parent_id) = self.find_symbol_by_name(&parent) {
                        self.relationships.push(Relationship {
                            from_id: symbol.id.clone(),
                            to_id: parent_id,
                            kind: RelationshipKind::Extends,
                            confidence: 0.9,
                        });
                    }
                }
            }
        }
    }
    
    /// Build call relationships (heuristic based on name matching)
    fn build_call_relationships(&mut self) {
        // Simple heuristic: if a function body contains another function name
        // This is a simplified version - real implementation would parse AST
        for symbol in &self.symbols {
            if symbol.kind == SymbolKind::Function || symbol.kind == SymbolKind::Method {
                // Count references to other functions
                let mut outgoing = 0;
                for other in &self.symbols {
                    if other.kind == SymbolKind::Function || other.kind == SymbolKind::Method {
                        if other.id != symbol.id && symbol.signature.contains(&other.name) {
                            outgoing += 1;
                        }
                    }
                }
                
                // Only record if there are significant calls
                if outgoing > 0 {
                    // Simplified - would need actual call graph analysis
                }
            }
        }
    }
    
    /// Calculate centrality scores for all files (internal method)
    fn compute_centrality(&mut self) {
        // Count incoming and outgoing edges for each file
        let mut file_incoming: HashMap<String, usize> = HashMap::new();
        let mut file_outgoing: HashMap<String, usize> = HashMap::new();
        
        for rel in &self.relationships {
            *file_outgoing.entry(rel.from_id.clone()).or_insert(0) += 1;
            *file_incoming.entry(rel.to_id.clone()).or_insert(0) += 1;
        }
        
        // Calculate centrality score for each file
        for file in &mut self.files {
            let incoming = file_incoming.get(&file.id).copied().unwrap_or(0);
            let outgoing = file_outgoing.get(&file.id).copied().unwrap_or(0);
            
            // Centrality = weighted combination of incoming (more important) and outgoing
            file.centrality_score = (incoming as f64 * 2.0 + outgoing as f64) / 3.0;
        }
        
        // Normalize scores to 0-1 range
        let max_score = self.files.iter()
            .map(|f| f.centrality_score)
            .fold(0.0_f64, |a, b| a.max(b));
        
        if max_score > 0.0 {
            for file in &mut self.files {
                file.centrality_score /= max_score;
            }
        }
    }
    
    /// Update symbol counts for files
    fn update_file_symbol_counts(&mut self) {
        let mut counts: HashMap<String, usize> = HashMap::new();
        
        for symbol in &self.symbols {
            *counts.entry(symbol.file_id.clone()).or_insert(0) += 1;
        }
        
        for file in &mut self.files {
            file.symbol_count = counts.get(&file.id).copied().unwrap_or(0);
        }
    }
    
    /// Update directory counts
    fn update_dir_counts(&mut self, dir_map: &HashMap<String, usize>) {
        for file in &self.files {
            if let Some(parent_dir) = get_parent_dir(&file.relative_path) {
                if let Some(&dir_idx) = dir_map.get(&parent_dir) {
                    if let Some(dir) = self.directories.get_mut(dir_idx) {
                        dir.file_count += 1;
                        dir.children_count += 1;
                    }
                }
            }
        }
        
        // Count subdirectories (fix borrow checker issue)
        let parent_dirs: Vec<_> = self.directories.iter()
            .filter_map(|dir| {
                get_parent_dir(&dir.relative_path).map(|parent| (dir.id.clone(), parent))
            })
            .collect();
        
        for (dir_id, parent_dir) in parent_dirs {
            if let Some(&dir_idx) = dir_map.get(&parent_dir) {
                if let Some(parent) = self.directories.get_mut(dir_idx) {
                    parent.dir_count += 1;
                    parent.children_count += 1;
                }
            }
        }
    }
    
    /// Find the target file for an import
    fn find_import_target(&self, import_path: &str) -> Option<String> {
        // Try to match import path to file paths
        for file in &self.files {
            let file_base = get_file_base(&file.relative_path);
            if import_path.ends_with(&file_base) || file.relative_path.contains(import_path) {
                return Some(file.id.clone());
            }
        }
        None
    }
    
    /// Find a symbol by name
    fn find_symbol_by_name(&self, name: &str) -> Option<String> {
        self.symbols.iter()
            .find(|s| s.name == name)
            .map(|s| s.id.clone())
    }
    
    /// Get file summary
    pub fn get_file_summary(&self, file_path: &str) -> Result<FileSummary, String> {
        let file = self.files.iter()
            .find(|f| f.relative_path == file_path || f.path.contains(file_path))
            .ok_or_else(|| format!("File not found: {}", file_path))?;
        
        let file_symbols: Vec<Symbol> = self.symbols.iter()
            .filter(|s| s.file_id == file.id)
            .cloned()
            .collect();
        
        let imports = extract_imports_from_symbols(&file_symbols);
        
        let dependencies = self.relationships.iter()
            .filter(|r| r.from_id == file.id)
            .map(|r| DependencyInfo {
                target_id: r.to_id.clone(),
                target_path: self.get_path_for_id(&r.to_id).unwrap_or_default(),
                relationship: r.kind.clone(),
            })
            .collect();
        
        let dependents = self.relationships.iter()
            .filter(|r| r.to_id == file.id)
            .map(|r| DependencyInfo {
                target_id: r.from_id.clone(),
                target_path: self.get_path_for_id(&r.from_id).unwrap_or_default(),
                relationship: r.kind.clone(),
            })
            .collect();
        
        let one_line_summary = generate_one_line_summary(file, &file_symbols);
        
        Ok(FileSummary {
            file: file.clone(),
            symbols: file_symbols,
            imports,
            dependencies,
            dependents,
            one_line_summary,
        })
    }
    
    /// Get path for an ID
    fn get_path_for_id(&self, id: &str) -> Option<String> {
        if id.starts_with("file_") {
            self.files.iter()
                .find(|f| f.id == id)
                .map(|f| f.relative_path.clone())
        } else {
            self.symbols.iter()
                .find(|s| s.id == id)
                .map(|s| s.file_path.clone())
        }
    }
    
    /// Get compact project skeleton for LLM context
    pub fn get_skeleton(&self) -> ProjectSkeleton {
        let root_name = self.root_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "project".to_string());
        
        let top_level_dirs: Vec<String> = self.directories.iter()
            .filter(|d| d.relative_path.split('/').count() == 1 && !d.relative_path.is_empty())
            .map(|d| d.relative_path.clone())
            .collect();
        
        let key_files: Vec<KeyFile> = self.files.iter()
            .filter(|f| f.centrality_score > 0.5)
            .map(|f| KeyFile {
                path: f.relative_path.clone(),
                role: f.role.clone(),
                centrality: f.centrality_score,
            })
            .take(20)
            .collect();
        
        let symbol_index: Vec<SymbolIndexEntry> = self.symbols.iter()
            .filter(|s| matches!(s.kind, SymbolKind::Class | SymbolKind::Function | SymbolKind::Interface))
            .map(|s| SymbolIndexEntry {
                id: s.id.clone(),
                name: s.name.clone(),
                kind: s.kind.clone(),
                file_path: s.file_path.clone(),
                centrality: self.files.iter()
                    .find(|f| f.id == s.file_id)
                    .map(|f| f.centrality_score)
                    .unwrap_or(0.0),
            })
            .take(100)
            .collect();
        
        ProjectSkeleton {
            root_name,
            top_level_dirs,
            key_files,
            symbol_index,
        }
    }
    
    /// Calculate centrality for a specific element (public method)
    pub fn calculate_centrality(&self, symbol: &Symbol) -> Centrality {
        let incoming = self.relationships.iter()
            .filter(|r| r.to_id == symbol.id)
            .count();
        
        let outgoing = self.relationships.iter()
            .filter(|r| r.from_id == symbol.id)
            .count();
        
        let score = (incoming as f64 * 2.0 + outgoing as f64) / 3.0;
        let max_possible = self.symbols.len().max(1);
        let normalized_score = score / max_possible as f64;
        
        let level = if normalized_score > 0.7 {
            CentralityLevel::High
        } else if normalized_score > 0.3 {
            CentralityLevel::Medium
        } else {
            CentralityLevel::Low
        };
        
        let description = match level {
            CentralityLevel::High => format!(
                "Critical element: referenced by {} other elements, calls {} others",
                incoming, outgoing
            ),
            CentralityLevel::Medium => format!(
                "Moderate impact: referenced by {} elements, calls {} others",
                incoming, outgoing
            ),
            CentralityLevel::Low => format!(
                "Incidental element: referenced by {} elements, calls {} others",
                incoming, outgoing
            ),
        };
        
        Centrality {
            score: normalized_score,
            level,
            incoming_edges: incoming,
            outgoing_edges: outgoing,
            description,
        }
    }
    
    /// Convert to IR for JSON output
    pub fn to_ir(&self) -> ProjectMapIR {
        let stats = self.compute_stats();
        
        ProjectMapIR {
            root_path: self.root_path.to_string_lossy().to_string(),
            files: self.files.clone(),
            directories: self.directories.clone(),
            symbols: self.symbols.clone(),
            relationships: self.relationships.clone(),
            stats,
        }
    }
    
    /// Compute map statistics
    fn compute_stats(&self) -> MapStats {
        let total_lines: usize = self.files.iter().map(|f| f.line_count).sum();
        let total_size: u64 = self.files.iter().map(|f| f.size_bytes).sum();
        
        let mut languages: HashMap<String, usize> = HashMap::new();
        let mut roles: HashMap<String, usize> = HashMap::new();
        
        for file in &self.files {
            *languages.entry(file.language.clone()).or_insert(0) += 1;
            let role_str = format!("{:?}", file.role);
            *roles.entry(role_str).or_insert(0) += 1;
        }
        
        let avg_centrality = if self.files.is_empty() {
            0.0
        } else {
            self.files.iter().map(|f| f.centrality_score).sum::<f64>() / self.files.len() as f64
        };
        
        MapStats {
            total_files: self.files.len(),
            total_directories: self.directories.len(),
            total_symbols: self.symbols.len(),
            total_lines,
            total_size_bytes: total_size,
            languages,
            roles,
            avg_centrality,
        }
    }
}

/// Check if a path should be ignored
fn is_ignored(path: &Path) -> bool {
    let name = path.file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_default();
    
    // Common ignore patterns
    let ignore_patterns = [
        ".git", ".svn", ".hg",
        "node_modules", "__pycache__", ".venv", "venv",
        "target", "build", "dist", "out",
        "*.min.js", "*.bundle.js",
        ".DS_Store", "Thumbs.db",
    ];
    
    ignore_patterns.iter().any(|pattern| {
        if pattern.starts_with("*.") {
            name.ends_with(&pattern[1..])
        } else {
            name == *pattern
        }
    })
}

/// Detect language from file extension
fn detect_language_from_extension(ext: &str) -> String {
    match ext.to_lowercase().as_str() {
        "rs" => "Rust".to_string(),
        "py" => "Python".to_string(),
        "js" | "mjs" => "JavaScript".to_string(),
        "ts" | "tsx" => "TypeScript".to_string(),
        "jsx" => "JSX".to_string(),
        "go" => "Go".to_string(),
        "rb" => "Ruby".to_string(),
        "java" => "Java".to_string(),
        "c" | "h" => "C".to_string(),
        "cpp" | "cc" | "cxx" | "hpp" => "C++".to_string(),
        "cs" => "C#".to_string(),
        "php" => "PHP".to_string(),
        "swift" => "Swift".to_string(),
        "kt" | "kts" => "Kotlin".to_string(),
        "scala" => "Scala".to_string(),
        "ex" | "exs" => "Elixir".to_string(),
        "erl" | "hrl" => "Erlang".to_string(),
        "hs" => "Haskell".to_string(),
        "ml" | "mli" => "OCaml".to_string(),
        "sh" | "bash" | "zsh" => "Shell".to_string(),
        "md" => "Markdown".to_string(),
        "json" => "JSON".to_string(),
        "yaml" | "yml" => "YAML".to_string(),
        "toml" => "TOML".to_string(),
        "xml" => "XML".to_string(),
        "html" | "htm" => "HTML".to_string(),
        "css" | "scss" | "sass" | "less" => "CSS".to_string(),
        "sql" => "SQL".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// Compute hash of content
fn compute_hash(content: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Get parent directory from a path
fn get_parent_dir(path: &str) -> Option<String> {
    Path::new(path).parent()
        .map(|p| p.to_string_lossy().to_string())
        .filter(|s| !s.is_empty())
}

/// Get base name without extension
fn get_file_base(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string())
}

/// Extract imports from symbols
fn extract_imports_from_symbols(symbols: &[Symbol]) -> Vec<ImportInfo> {
    symbols.iter()
        .filter(|s| s.kind == SymbolKind::Import)
        .map(|s| {
            let is_relative = s.name.starts_with('.') || s.name.starts_with("..");
            let is_external = !is_relative && !s.name.starts_with('/');
            
            ImportInfo {
                source: s.name.clone(),
                names: vec![s.name.split('/').last().unwrap_or(&s.name).to_string()],
                is_external,
                is_relative,
            }
        })
        .collect()
}

/// Extract parent class from signature
fn extract_parent_class(signature: &str) -> Option<String> {
    // Look for "extends ClassName" or "implements InterfaceName"
    let patterns = [
        r"extends\s+(\w+)",
        r"implements\s+(\w+)",
        r":\s*(\w+)",
        r"inherit\s+(\w+)",
    ];
    
    for pattern in patterns {
        if let Some(captures) = regex::Regex::new(pattern).ok()?.captures(signature) {
            if let Some(parent) = captures.get(1) {
                return Some(parent.as_str().to_string());
            }
        }
    }
    
    None
}

/// Generate a one-line summary of a file
fn generate_one_line_summary(file: &FileNode, symbols: &[Symbol]) -> String {
    let class_count = symbols.iter().filter(|s| s.kind == SymbolKind::Class).count();
    let func_count = symbols.iter().filter(|s| matches!(s.kind, SymbolKind::Function | SymbolKind::Method)).count();
    
    let role_str = format!("{:?}", file.role);
    
    if class_count > 0 && func_count > 0 {
        format!(
            "{} file with {} classes and {} functions ({}, {:.0}%)",
            role_str, class_count, func_count, file.language, file.centrality_score * 100.0
        )
    } else if func_count > 0 {
        format!(
            "{} file with {} functions ({}, {:.0}%)",
            role_str, func_count, file.language, file.centrality_score * 100.0
        )
    } else {
        format!(
            "{} {} file ({:.0}% central)",
            file.language, role_str, file.centrality_score * 100.0
        )
    }
}
