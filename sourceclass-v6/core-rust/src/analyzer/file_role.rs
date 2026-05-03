use crate::ir::schema::{FileObject, FileRoleObject};

pub fn classify_file_roles(files: &[FileObject]) -> Vec<FileRoleObject> {
    let mut roles = Vec::new();
    
    for file in files {
        if let Some(role) = determine_file_role(file) {
            roles.push(role);
        }
    }
    
    roles
}

fn determine_file_role(file: &FileObject) -> Option<FileRoleObject> {
    let filename = file.path.split('/').last().unwrap_or(&file.path).to_lowercase();
    let path_lower = file.path.to_lowercase();
    
    // Skip certain files
    if file.is_test || file.is_generated {
        return None;
    }
    
    // Determine role based on patterns
    let (role, importance, confidence, reason) = if is_brain_file(&filename, &path_lower) {
        (String::from("Brain"), String::from("High"), 0.85, String::from("Core logic/analyzer/engine file"))
    } else if is_entry_file(&filename, &path_lower) {
        (String::from("Entry"), String::from("High"), 0.90, String::from("Execution entry point or index"))
    } else if is_interface_file(&filename, &path_lower) {
        (String::from("Interface"), String::from("Medium"), 0.80, String::from("API/routes/commands interface"))
    } else if is_connector_file(&filename, &path_lower) {
        (String::from("Connector"), String::from("Medium"), 0.78, String::from("External service integration"))
    } else if file.is_config || is_config_file(&filename, &path_lower) {
        (String::from("Config"), String::from("Medium"), 0.92, String::from("Configuration file"))
    } else if is_utility_file(&filename, &path_lower) {
        (String::from("Utility"), String::from("Low"), 0.75, String::from("Helper/utility function"))
    } else if is_output_file(&filename, &path_lower) {
        (String::from("Output"), String::from("Medium"), 0.80, String::from("Report/render/export logic"))
    } else if is_dangerous_file(&filename, &path_lower, file) {
        (String::from("Dangerous"), String::from("High"), 0.88, String::from("Security-sensitive file"))
    } else {
        // Default to utility for unclassified files
        (String::from("Utility"), String::from("Low"), 0.50, String::from("General purpose file"))
    };
    
    Some(FileRoleObject {
        file: file.path.clone(),
        role,
        importance,
        confidence,
        reason,
    })
}

fn is_brain_file(filename: &str, path: &str) -> bool {
    let brain_keywords = ["analyzer", "engine", "core", "service", "planner", "parser", "processor", "compiler", "interpreter"];
    brain_keywords.iter().any(|k| filename.contains(k) || path.contains(k))
}

fn is_entry_file(filename: &str, path: &str) -> bool {
    let entry_keywords = ["main", "index", "app", "cli", "entry", "bootstrap", "init"];
    entry_keywords.iter().any(|k| filename.contains(k) || path.contains(k))
}

fn is_interface_file(filename: &str, path: &str) -> bool {
    let interface_keywords = ["routes", "api", "ui", "commands", "controller", "handlers", "views", "pages"];
    interface_keywords.iter().any(|k| filename.contains(k) || path.contains(k))
}

fn is_connector_file(filename: &str, path: &str) -> bool {
    let connector_keywords = ["provider", "client", "adapter", "db", "database", "http", "integration", "repository"];
    connector_keywords.iter().any(|k| filename.contains(k) || path.contains(k))
}

fn is_config_file(filename: &str, path: &str) -> bool {
    let config_keywords = ["config", "settings", "env", "constant", "defaults"];
    config_keywords.iter().any(|k| filename.contains(k) || path.contains(k))
}

fn is_utility_file(filename: &str, path: &str) -> bool {
    let utility_keywords = ["utils", "helpers", "common", "shared", "tools", "functions", "lib"];
    utility_keywords.iter().any(|k| filename.contains(k) || path.contains(k))
}

fn is_output_file(filename: &str, path: &str) -> bool {
    let output_keywords = ["report", "exporter", "markdown", "renderer", "generator", "builder", "printer"];
    output_keywords.iter().any(|k| filename.contains(k) || path.contains(k))
}

fn is_dangerous_file(filename: &str, path: &str, file: &FileObject) -> bool {
    let dangerous_keywords = ["auth", "secret", "credential", "token", "permission", "deploy", "key"];
    let is_keyword = dangerous_keywords.iter().any(|k| filename.contains(k) || path.contains(k));
    
    is_keyword || 
    filename == ".env" || 
    filename.ends_with(".pem") || 
    filename.ends_with(".key") ||
    file.extension == "env"
}
