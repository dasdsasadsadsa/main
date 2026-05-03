use std::fs;
use std::path::Path;
use crate::ir::schema::{FileObject, DependencyObject};

pub fn extract_dependencies(files: &[FileObject], base_path: &str) -> Vec<DependencyObject> {
    let mut dependencies = Vec::new();
    
    for file in files {
        if let Some(dep) = parse_dependency_file(file, base_path) {
            dependencies.push(dep);
        }
    }
    
    dependencies
}

fn parse_dependency_file(file: &FileObject, base_path: &str) -> Option<DependencyObject> {
    let filename = file.path.split('/').last().unwrap_or(&file.path);
    let path = Path::new(base_path).join(&file.path);
    
    let (manager, deps) = match filename {
        "Cargo.toml" => parse_cargo_toml(&path)?,
        "package.json" => parse_package_json(&path)?,
        "requirements.txt" => parse_requirements_txt(&path)?,
        "go.mod" => parse_go_mod(&path)?,
        _ => return None,
    };
    
    Some(DependencyObject {
        source: file.path.clone(),
        manager,
        dependencies: deps,
    })
}

fn parse_cargo_toml(path: &Path) -> Option<(String, Vec<crate::ir::schema::DependencyInfo>)> {
    let content = fs::read_to_string(path).ok()?;
    let mut deps = Vec::new();
    
    // Simple parsing - look for [dependencies] section
    let mut in_deps = false;
    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed == "[dependencies]" {
            in_deps = true;
            continue;
        } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_deps = false;
            continue;
        }
        
        if in_deps && trimmed.contains('=') {
            let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
            if parts.len() == 2 {
                let name = parts[0].trim().to_string();
                let version = extract_version(parts[1]);
                
                deps.push(crate::ir::schema::DependencyInfo {
                    name,
                    version,
                    kind: String::from("runtime"),
                });
            }
        }
    }
    
    Some((String::from("cargo"), deps))
}

fn parse_package_json(path: &Path) -> Option<(String, Vec<crate::ir::schema::DependencyInfo>)> {
    let content = fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    let mut deps = Vec::new();
    
    if let Some(dependencies) = json.get("dependencies").and_then(|v| v.as_object()) {
        for (name, version) in dependencies {
            deps.push(crate::ir::schema::DependencyInfo {
                name: name.clone(),
                version: version.as_str().unwrap_or("*").to_string(),
                kind: String::from("runtime"),
            });
        }
    }
    
    if let Some(dev_dependencies) = json.get("devDependencies").and_then(|v| v.as_object()) {
        for (name, version) in dev_dependencies {
            deps.push(crate::ir::schema::DependencyInfo {
                name: name.clone(),
                version: version.as_str().unwrap_or("*").to_string(),
                kind: String::from("dev"),
            });
        }
    }
    
    Some((String::from("npm"), deps))
}

fn parse_requirements_txt(path: &Path) -> Option<(String, Vec<crate::ir::schema::DependencyInfo>)> {
    let content = fs::read_to_string(path).ok()?;
    let mut deps = Vec::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        // Handle various formats: package==version, package>=version, package
        let name = if trimmed.contains("==") {
            trimmed.split("==").next()?.trim().to_string()
        } else if trimmed.contains(">=") {
            trimmed.split(">=").next()?.trim().to_string()
        } else if trimmed.contains("<=") {
            trimmed.split("<=").next()?.trim().to_string()
        } else {
            trimmed.to_string()
        };
        
        let version = if trimmed.contains("==") {
            trimmed.split("==").nth(1)?.trim().to_string()
        } else {
            String::from("*")
        };
        
        deps.push(crate::ir::schema::DependencyInfo {
            name,
            version,
            kind: String::from("runtime"),
        });
    }
    
    Some((String::from("pip"), deps))
}

fn parse_go_mod(path: &Path) -> Option<(String, Vec<crate::ir::schema::DependencyInfo>)> {
    let content = fs::read_to_string(path).ok()?;
    let mut deps = Vec::new();
    
    let mut in_require = false;
    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("require (") {
            in_require = true;
            continue;
        } else if trimmed == ")" {
            in_require = false;
            continue;
        }
        
        if in_require || trimmed.starts_with("require ") {
            let line = if trimmed.starts_with("require ") {
                trimmed.strip_prefix("require ")?.trim()
            } else {
                trimmed
            };
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                deps.push(crate::ir::schema::DependencyInfo {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    kind: String::from("runtime"),
                });
            }
        }
    }
    
    Some((String::from("go"), deps))
}

fn extract_version(value: &str) -> String {
    let value = value.trim();
    
    // Handle quoted strings
    if value.starts_with('"') && value.ends_with('"') {
        return value[1..value.len()-1].to_string();
    }
    
    // Handle inline table { version = "x.y" }
    if value.contains("version") {
        if let Some(start) = value.find('"') {
            if let Some(end) = value[start+1..].find('"') {
                return value[start+1..start+1+end].to_string();
            }
        }
    }
    
    value.to_string()
}
