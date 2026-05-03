//! Symbol Extractor
//! 
//! Extracts symbols from source files using tree-sitter for supported languages,
//! with regex fallbacks for others.

use std::path::Path;
use std::fs;

use crate::map::schema::{Symbol, SymbolKind, Visibility, Parameter};

/// Main symbol extractor supporting multiple languages
pub struct SymbolExtractor {
    // Tree-sitter parsers would be initialized here
}

impl SymbolExtractor {
    /// Create a new symbol extractor
    pub fn new() -> Self {
        Self {}
    }
    
    /// Extract symbols from a file
    pub fn extract_from_file(&self, path: &Path) -> Result<Vec<Symbol>, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let extension = path.extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();
        
        let language = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        
        let file_id = format!("file_{}", path.to_string_lossy().replace('/', "_"));
        
        // Use tree-sitter for supported languages, regex for others
        match extension.to_lowercase().as_str() {
            "rs" => self.extract_rust_symbols(&content, &file_id, path),
            "py" => self.extract_python_symbols(&content, &file_id, path),
            "js" | "mjs" => self.extract_javascript_symbols(&content, &file_id, path),
            "ts" | "tsx" => self.extract_typescript_symbols(&content, &file_id, path),
            "go" => self.extract_go_symbols(&content, &file_id, path),
            "java" => self.extract_java_symbols(&content, &file_id, path),
            _ => self.extract_regex_symbols(&content, &extension, &file_id, path),
        }
    }
    
    /// Extract Rust symbols using regex (tree-sitter would be better in production)
    fn extract_rust_symbols(&self, content: &str, file_id: &str, path: &Path) -> Result<Vec<Symbol>, String> {
        let mut symbols = Vec::new();
        let file_path = path.to_string_lossy().to_string();
        
        // Extract structs
        let struct_pattern = regex::Regex::new(r"(?m)^(pub\s+)?struct\s+(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in struct_pattern.captures_iter(content) {
            let visibility = if cap.get(1).is_some() { Visibility::Public } else { Visibility::Private };
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_struct_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Struct,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: visibility.clone(),
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract enums
        let enum_pattern = regex::Regex::new(r"(?m)^(pub\s+)?enum\s+(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in enum_pattern.captures_iter(content) {
            let visibility = if cap.get(1).is_some() { Visibility::Public } else { Visibility::Private };
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_enum_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Enum,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract traits
        let trait_pattern = regex::Regex::new(r"(?m)^(pub\s+)?trait\s+(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in trait_pattern.captures_iter(content) {
            let visibility = if cap.get(1).is_some() { Visibility::Public } else { Visibility::Private };
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_trait_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Trait,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract functions (including impl methods)
        let fn_pattern = regex::Regex::new(r"(?m)^(pub\s+)?(async\s+)?(fn)\s+(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in fn_pattern.captures_iter(content) {
            let visibility = if cap.get(1).is_some() { Visibility::Public } else { Visibility::Private };
            let is_async = cap.get(2).is_some();
            let name = cap.get(4).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            // Try to extract parameters and return type from signature
            let signature = extract_rust_fn_signature(content, cap.get(0).unwrap().start());
            let (params, return_type) = parse_rust_fn_params_return(&signature);
            
            symbols.push(Symbol {
                id: format!("{}_fn_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Function,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature,
                docstring: None,
                parent_id: None,
                visibility,
                is_async,
                is_static: false,
                parameters: params,
                return_type,
                type_parameters: vec![],
            });
        }
        
        // Extract impl blocks to identify methods
        self.extract_rust_impl_methods(content, file_id, &file_path, &mut symbols);
        
        // Extract use statements as imports
        let use_pattern = regex::Regex::new(r"(?m)^use\s+([^;]+);")
            .map_err(|e| e.to_string())?;
        
        for cap in use_pattern.captures_iter(content) {
            let import_path = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_use_{}", file_id, start_line),
                name: import_path.to_string(),
                kind: SymbolKind::Import,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: Visibility::Private,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        Ok(symbols)
    }
    
    /// Extract methods from Rust impl blocks
    fn extract_rust_impl_methods(&self, content: &str, file_id: &str, file_path: &str, symbols: &mut Vec<Symbol>) {
        // Find impl blocks and their methods
        let impl_pattern = regex::Regex::new(r"(?m)^impl(\s+<[^>]+>)?\s+(\w+)").unwrap();
        let impl_matches: Vec<_> = impl_pattern.captures_iter(content).collect();
        
        for impl_cap in impl_matches {
            let impl_name = impl_cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let impl_start = impl_cap.get(0).unwrap().end();
            
            // Find the closing brace of this impl block
            let impl_body = find_brace_block(content, impl_start);
            
            if let Some(body) = impl_body {
                // Extract methods from the impl body
                let method_pattern = regex::Regex::new(r"(?m)(pub\s+)?(async\s+)?(fn)\s+(\w+)").ok();
                
                if let Some(method_re) = method_pattern {
                    for method_cap in method_re.captures_iter(body) {
                        let visibility = if method_cap.get(1).is_some() { Visibility::Public } else { Visibility::Private };
                        let is_async = method_cap.get(2).is_some();
                        let method_name = method_cap.get(4).map(|m| m.as_str()).unwrap_or("");
                        
                        // Calculate actual line number
                        let offset = body.as_ptr() as usize - content.as_ptr() as usize;
                        let start_line = content[..offset].lines().count();
                        
                        let signature = extract_rust_fn_signature(body, method_cap.get(0).unwrap().start());
                        let (params, return_type) = parse_rust_fn_params_return(&signature);
                        
                        // Find parent struct/impl
                        let parent_id = symbols.iter()
                            .find(|s| s.name == impl_name && matches!(s.kind, SymbolKind::Struct | SymbolKind::Trait))
                            .map(|s| s.id.clone());
                        
                        symbols.push(Symbol {
                            id: format!("{}_method_{}_{}", file_id, impl_name, method_name),
                            name: method_name.to_string(),
                            kind: SymbolKind::Method,
                            file_id: file_id.to_string(),
                            file_path: file_path.to_string(),
                            start_line,
                            end_line: start_line + 1,
                            start_col: 0,
                            end_col: 0,
                            signature,
                            docstring: None,
                            parent_id,
                            visibility,
                            is_async,
                            is_static: method_name == "new",
                            parameters: params,
                            return_type,
                            type_parameters: vec![],
                        });
                    }
                }
            }
        }
    }
    
    /// Extract Python symbols
    fn extract_python_symbols(&self, content: &str, file_id: &str, path: &Path) -> Result<Vec<Symbol>, String> {
        let mut symbols = Vec::new();
        let file_path = path.to_string_lossy().to_string();
        
        // Extract classes
        let class_pattern = regex::Regex::new(r"(?m)^class\s+(\w+)(?:\(([^)]*)\))?")
            .map_err(|e| e.to_string())?;
        
        for cap in class_pattern.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let parents = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_class_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Class,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: format!("class {}({})", name, parents),
                docstring: None,
                parent_id: None,
                visibility: Visibility::Public,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract functions
        let fn_pattern = regex::Regex::new(r"(?m)^(async\s+)?def\s+(\w+)\s*\(")
            .map_err(|e| e.to_string())?;
        
        for cap in fn_pattern.captures_iter(content) {
            let is_async = cap.get(1).is_some();
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            let signature = extract_python_fn_signature(content, cap.get(0).unwrap().start());
            let params = parse_python_fn_params(&signature);
            
            // Check if it's a method (indented, inside a class)
            let kind = if content[..cap.get(0).unwrap().start()].ends_with("\n    ") || 
                          content[..cap.get(0).unwrap().start()].ends_with("\n\t") {
                SymbolKind::Method
            } else {
                SymbolKind::Function
            };
            
            symbols.push(Symbol {
                id: format!("{}_fn_{}", file_id, name),
                name: name.to_string(),
                kind,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature,
                docstring: None,
                parent_id: None,
                visibility: Visibility::Public,
                is_async,
                is_static: name.starts_with("__") && name.ends_with("__"),
                parameters: params,
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract imports
        let import_pattern = regex::Regex::new(r"(?m)^import\s+([^\n]+)|^from\s+(\S+)\s+import\s+([^\n]+)")
            .map_err(|e| e.to_string())?;
        
        for cap in import_pattern.captures_iter(content) {
            let import_path = cap.get(1)
                .or(cap.get(2))
                .map(|m| m.as_str())
                .unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_import_{}", file_id, start_line),
                name: import_path.to_string(),
                kind: SymbolKind::Import,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: Visibility::Private,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        Ok(symbols)
    }
    
    /// Extract JavaScript symbols
    fn extract_javascript_symbols(&self, content: &str, file_id: &str, path: &Path) -> Result<Vec<Symbol>, String> {
        let mut symbols = Vec::new();
        let file_path = path.to_string_lossy().to_string();
        
        // Extract classes
        let class_pattern = regex::Regex::new(r"(?m)(export\s+)?(default\s+)?class\s+(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in class_pattern.captures_iter(content) {
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let is_exported = cap.get(1).is_some() || cap.get(2).is_some();
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_class_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Class,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: if is_exported { Visibility::Public } else { Visibility::Private },
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract functions
        let fn_pattern = regex::Regex::new(r"(?m)(export\s+)?(async\s+)?function\s+(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in fn_pattern.captures_iter(content) {
            let is_exported = cap.get(1).is_some();
            let is_async = cap.get(2).is_some();
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_fn_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Function,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: if is_exported { Visibility::Public } else { Visibility::Private },
                is_async,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract arrow functions assigned to const/let/var
        let arrow_pattern = regex::Regex::new(r"(?m)(export\s+)?(const|let|var)\s+(\w+)\s*=\s*(async\s+)?\([^)]*\)\s*=>")
            .map_err(|e| e.to_string())?;
        
        for cap in arrow_pattern.captures_iter(content) {
            let is_exported = cap.get(1).is_some();
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let is_async = cap.get(4).is_some();
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_arrow_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Function,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: if is_exported { Visibility::Public } else { Visibility::Private },
                is_async,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract imports
        let import_pattern = regex::Regex::new(r#"(?m)^import\s+.*?from\s+['"]([^'"]+)['"]"#)
            .map_err(|e| e.to_string())?;
        
        for cap in import_pattern.captures_iter(content) {
            let import_source = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_import_{}", file_id, start_line),
                name: import_source.to_string(),
                kind: SymbolKind::Import,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: Visibility::Private,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        Ok(symbols)
    }
    
    /// Extract TypeScript symbols (extends JS with types/interfaces)
    fn extract_typescript_symbols(&self, content: &str, file_id: &str, path: &Path) -> Result<Vec<Symbol>, String> {
        let mut symbols = self.extract_javascript_symbols(content, file_id, path)?;
        let file_path = path.to_string_lossy().to_string();
        
        // Extract interfaces
        let interface_pattern = regex::Regex::new(r"(?m)(export\s+)?interface\s+(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in interface_pattern.captures_iter(content) {
            let is_exported = cap.get(1).is_some();
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_interface_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Interface,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: if is_exported { Visibility::Public } else { Visibility::Private },
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract type aliases
        let type_pattern = regex::Regex::new(r"(?m)(export\s+)?type\s+(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in type_pattern.captures_iter(content) {
            let is_exported = cap.get(1).is_some();
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_type_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::TypeAlias,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: if is_exported { Visibility::Public } else { Visibility::Private },
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        Ok(symbols)
    }
    
    /// Extract Go symbols
    fn extract_go_symbols(&self, content: &str, file_id: &str, path: &Path) -> Result<Vec<Symbol>, String> {
        let mut symbols = Vec::new();
        let file_path = path.to_string_lossy().to_string();
        
        // Extract structs
        let struct_pattern = regex::Regex::new(r"(?m)^type\s+(\w+)\s+struct")
            .map_err(|e| e.to_string())?;
        
        for cap in struct_pattern.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_struct_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Struct,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: Visibility::Public,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract interfaces
        let interface_pattern = regex::Regex::new(r"(?m)^type\s+(\w+)\s+interface")
            .map_err(|e| e.to_string())?;
        
        for cap in interface_pattern.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_interface_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Interface,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: Visibility::Public,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract functions
        let fn_pattern = regex::Regex::new(r"(?m)^func\s+(?:\([^)]+\)\s+)?(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in fn_pattern.captures_iter(content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            // Check if it's a method (has receiver)
            let is_method = cap.get(0).unwrap().as_str().starts_with("func (");
            
            symbols.push(Symbol {
                id: format!("{}_fn_{}", file_id, name),
                name: name.to_string(),
                kind: if is_method { SymbolKind::Method } else { SymbolKind::Function },
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: Visibility::Public,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        Ok(symbols)
    }
    
    /// Extract Java symbols
    fn extract_java_symbols(&self, content: &str, file_id: &str, path: &Path) -> Result<Vec<Symbol>, String> {
        let mut symbols = Vec::new();
        let file_path = path.to_string_lossy().to_string();
        
        // Extract classes
        let class_pattern = regex::Regex::new(r"(?m)(public\s+)?(abstract\s+)?class\s+(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in class_pattern.captures_iter(content) {
            let visibility = if cap.get(1).is_some() { Visibility::Public } else { Visibility::Internal };
            let name = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_class_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Class,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract interfaces
        let interface_pattern = regex::Regex::new(r"(?m)(public\s+)?interface\s+(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in interface_pattern.captures_iter(content) {
            let visibility = if cap.get(1).is_some() { Visibility::Public } else { Visibility::Internal };
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_interface_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Interface,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        // Extract methods
        let method_pattern = regex::Regex::new(r"(?m)(public|private|protected)?\s*(static\s+)?(\w+)\s+(\w+)\s*\(")
            .map_err(|e| e.to_string())?;
        
        for cap in method_pattern.captures_iter(content) {
            let visibility = match cap.get(1).map(|m| m.as_str()) {
                Some("public") => Visibility::Public,
                Some("private") => Visibility::Private,
                Some("protected") => Visibility::Protected,
                _ => Visibility::Internal,
            };
            let is_static = cap.get(2).is_some();
            let return_type = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let name = cap.get(4).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            // Skip if return type is 'class' or 'interface' (false positive)
            if ["class", "interface", "enum"].contains(&return_type) {
                continue;
            }
            
            symbols.push(Symbol {
                id: format!("{}_method_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Method,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility,
                is_async: false,
                is_static,
                parameters: vec![],
                return_type: Some(return_type.to_string()),
                type_parameters: vec![],
            });
        }
        
        Ok(symbols)
    }
    
    /// Generic regex-based symbol extraction for unsupported languages
    fn extract_regex_symbols(&self, content: &str, extension: &str, file_id: &str, path: &Path) -> Result<Vec<Symbol>, String> {
        // Basic pattern matching for common constructs
        let mut symbols = Vec::new();
        let file_path = path.to_string_lossy().to_string();
        
        // Try to find function-like patterns
        let fn_pattern = regex::Regex::new(r"(?m)(function|def|func|fn)\s+(\w+)")
            .map_err(|e| e.to_string())?;
        
        for cap in fn_pattern.captures_iter(content) {
            let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let start_line = content[..cap.get(0).unwrap().start()].lines().count();
            
            symbols.push(Symbol {
                id: format!("{}_fn_{}", file_id, name),
                name: name.to_string(),
                kind: SymbolKind::Function,
                file_id: file_id.to_string(),
                file_path: file_path.clone(),
                start_line,
                end_line: start_line + 1,
                start_col: 0,
                end_col: 0,
                signature: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                docstring: None,
                parent_id: None,
                visibility: Visibility::Public,
                is_async: false,
                is_static: false,
                parameters: vec![],
                return_type: None,
                type_parameters: vec![],
            });
        }
        
        Ok(symbols)
    }
}

impl Default for SymbolExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract Rust function signature from content starting at position
fn extract_rust_fn_signature(content: &str, start: usize) -> String {
    // Find the opening brace or semicolon
    let rest = &content[start..];
    let end = rest.find(|c| c == '{' || c == ';')
        .unwrap_or(rest.len().min(200));
    
    rest[..end].trim().to_string()
}

/// Parse Rust function parameters and return type
fn parse_rust_fn_params_return(signature: &str) -> (Vec<Parameter>, Option<String>) {
    let params_start = signature.find('(').unwrap_or(0);
    let params_end = signature.rfind(')').unwrap_or(signature.len());
    
    let params_str = &signature[params_start + 1..params_end.min(signature.len())];
    let return_type = signature[params_end..]
        .strip_prefix(")")
        .and_then(|s| s.split("->").nth(1))
        .map(|s| s.trim().to_string());
    
    let params = params_str.split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|p| {
            let parts: Vec<&str> = p.trim().split(':').collect();
            Parameter {
                name: parts.first().map(|s| s.trim().to_string()).unwrap_or_default(),
                type_annotation: parts.get(1).map(|s| s.trim().to_string()),
                default_value: None,
            }
        })
        .collect();
    
    (params, return_type)
}

/// Extract Python function signature
fn extract_python_fn_signature(content: &str, start: usize) -> String {
    let rest = &content[start..];
    let end = rest.find(':').unwrap_or(rest.len().min(200));
    rest[..end].trim().to_string()
}

/// Parse Python function parameters
fn parse_python_fn_params(signature: &str) -> Vec<Parameter> {
    let params_start = signature.find('(').unwrap_or(0);
    let params_end = signature.rfind(')').unwrap_or(signature.len());
    
    let params_str = &signature[params_start + 1..params_end.min(signature.len())];
    
    params_str.split(',')
        .filter(|s| !s.trim().is_empty() && s.trim() != "self")
        .map(|p| {
            let trimmed = p.trim();
            let parts: Vec<&str> = trimmed.split('=').collect();
            let name_and_type: Vec<&str> = parts.first().unwrap_or(&trimmed).split(':').collect();
            
            Parameter {
                name: name_and_type.first().map(|s| s.trim().to_string()).unwrap_or_default(),
                type_annotation: name_and_type.get(1).map(|s| s.trim().to_string()),
                default_value: parts.get(1).map(|s| s.trim().to_string()),
            }
        })
        .collect()
}

/// Find the content within braces starting from a position
fn find_brace_block(content: &str, start: usize) -> Option<&str> {
    let rest = &content[start..];
    let mut brace_count = 0;
    let mut block_start = None;
    
    for (i, c) in rest.char_indices() {
        if c == '{' {
            if brace_count == 0 {
                block_start = Some(i + 1);
            }
            brace_count += 1;
        } else if c == '}' {
            brace_count -= 1;
            if brace_count == 0 {
                if let Some(start_offset) = block_start {
                    return Some(&rest[start_offset..i]);
                }
            }
        }
    }
    
    None
}
