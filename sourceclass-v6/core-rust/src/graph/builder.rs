use crate::ir::schema::{FileObject, EntrypointObject, StructureGraph, GraphNode, GraphEdge};

pub fn build_structure_graph(files: &[FileObject], entrypoints: &[EntrypointObject]) -> StructureGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    
    // Create nodes for all files
    for file in files {
        let role = get_role_for_file(file);
        nodes.push(GraphNode {
            id: file.path.clone(),
            label: file.path.split('/').last().unwrap_or(&file.path).to_string(),
            role,
            language: file.language.clone(),
        });
    }
    
    // Create edges from entrypoints to other files (simple heuristic)
    for entrypoint in entrypoints {
        let entry_path = &entrypoint.file;
        
        // Connect entrypoint to likely core files
        for file in files {
            if file.path == *entry_path {
                continue;
            }
            
            // Simple heuristics for relationships
            let relationship = determine_relationship(entry_path, &file.path);
            
            if relationship != "unknown" {
                edges.push(GraphEdge {
                    from: entry_path.clone(),
                    to: file.path.clone(),
                    relationship: String::from(relationship),
                    confidence: 0.6,
                });
            }
            
            // Limit edges to prevent explosion
            if edges.len() > 100 {
                break;
            }
        }
        
        if edges.len() > 100 {
            break;
        }
    }
    
    StructureGraph { nodes, edges }
}

fn get_role_for_file(file: &FileObject) -> String {
    if file.is_config {
        String::from("Config")
    } else if file.is_test {
        String::from("Test")
    } else {
        // Default role based on path
        let filename = file.path.split('/').last().unwrap_or(&file.path).to_lowercase();
        
        if filename.contains("main") || filename.contains("index") || filename.contains("app") {
            String::from("Entry")
        } else if filename.contains("util") || filename.contains("helper") {
            String::from("Utility")
        } else if filename.contains("test") {
            String::from("Test")
        } else {
            String::from("Source")
        }
    }
}

fn determine_relationship(from: &str, to: &str) -> &'static str {
    let from_lower = from.to_lowercase();
    let to_lower = to.to_lowercase();
    
    // Config relationships
    if from_lower.contains("config") || from_lower.contains("toml") || from_lower.contains("json") {
        return "configures";
    }
    
    // Test relationships
    if to_lower.contains("test") || to_lower.ends_with("_test") {
        return "tests";
    }
    
    // Module relationships (same directory)
    let from_dir = from.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
    let to_dir = to.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
    
    if from_dir == to_dir && from != to {
        return "imports";
    }
    
    // Main to module relationships
    if from_lower.contains("main") && !to_lower.contains("main") {
        return "calls_likely";
    }
    
    "unknown"
}
