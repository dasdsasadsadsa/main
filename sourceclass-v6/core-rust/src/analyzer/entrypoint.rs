use std::fs;
use std::path::Path;
use crate::ir::schema::{FileObject, EntrypointObject};

pub fn detect_entrypoints(files: &[FileObject], base_path: &str) -> Vec<EntrypointObject> {
    let mut entrypoints = Vec::new();
    
    for file in files {
        if let Some(entrypoint) = check_file_for_entrypoint(file, base_path) {
            entrypoints.push(entrypoint);
        }
    }
    
    // Sort by confidence (highest first)
    entrypoints.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
    
    entrypoints
}

fn check_file_for_entrypoint(file: &FileObject, base_path: &str) -> Option<EntrypointObject> {
    let filename = file.path.split('/').last().unwrap_or(&file.path);
    let path = Path::new(base_path).join(&file.path);
    
    // Check by filename patterns
    match filename {
        "main.rs" => {
            return Some(EntrypointObject {
                file: file.path.clone(),
                kind: String::from("rust_main"),
                confidence: 0.95,
                reason: String::from("Rust main.rs - standard entry point"),
            });
        },
        "main.py" | "app.py" | "cli.py" => {
            return Some(EntrypointObject {
                file: file.path.clone(),
                kind: String::from("python_main"),
                confidence: 0.90,
                reason: String::from("Python main/app/cli file - common entry point pattern"),
            });
        },
        "index.js" | "server.js" | "app.js" => {
            return Some(EntrypointObject {
                file: file.path.clone(),
                kind: String::from("nodejs_main"),
                confidence: 0.88,
                reason: String::from("Node.js index/server/app file - common entry point"),
            });
        },
        "main.ts" | "index.ts" => {
            return Some(EntrypointObject {
                file: file.path.clone(),
                kind: String::from("typescript_main"),
                confidence: 0.85,
                reason: String::from("TypeScript main/index file - likely entry point"),
            });
        },
        "index.html" => {
            return Some(EntrypointObject {
                file: file.path.clone(),
                kind: String::from("web_entry"),
                confidence: 0.92,
                reason: String::from("HTML index - web application entry point"),
            });
        },
        _ => {}
    }
    
    // Check by content patterns (simple regex-style checks)
    if file.language == "Rust" && file.path.ends_with(".rs") {
        if let Ok(content) = fs::read_to_string(&path) {
            if content.contains("fn main()") {
                return Some(EntrypointObject {
                    file: file.path.clone(),
                    kind: String::from("rust_fn_main"),
                    confidence: 0.98,
                    reason: String::from("Contains fn main() - Rust execution entry point"),
                });
            }
        }
    }
    
    if file.language == "Python" && file.path.ends_with(".py") {
        if let Ok(content) = fs::read_to_string(&path) {
            if content.contains("if __name__ == \"__main__\"") || 
               content.contains("if __name__ == '__main__'") {
                return Some(EntrypointObject {
                    file: file.path.clone(),
                    kind: String::from("python_main_block"),
                    confidence: 0.95,
                    reason: String::from("Contains if __name__ == '__main__' - Python script entry"),
                });
            }
        }
    }
    
    if file.language == "Java" && file.path.ends_with(".java") {
        if let Ok(content) = fs::read_to_string(&path) {
            if content.contains("public static void main") {
                return Some(EntrypointObject {
                    file: file.path.clone(),
                    kind: String::from("java_main"),
                    confidence: 0.96,
                    reason: String::from("Contains public static void main - Java entry point"),
                });
            }
        }
    }
    
    if file.language == "Go" && file.path.ends_with(".go") {
        if let Ok(content) = fs::read_to_string(&path) {
            if content.contains("package main") && content.contains("func main()") {
                return Some(EntrypointObject {
                    file: file.path.clone(),
                    kind: String::from("go_main"),
                    confidence: 0.97,
                    reason: String::from("Go package main with func main() - Go entry point"),
                });
            }
        }
    }
    
    None
}
