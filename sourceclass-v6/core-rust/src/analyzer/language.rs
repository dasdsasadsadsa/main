use std::path::Path;
use crate::ir::schema::{FileObject, LanguageDetection};

pub fn detect_languages(files: &[FileObject]) -> Vec<LanguageDetection> {
    let mut detections = Vec::new();
    
    for file in files {
        detections.push(LanguageDetection {
            file: file.path.clone(),
            language: file.language.clone(),
            confidence: 0.95,
        });
    }
    
    detections
}

pub fn detect_language_from_path(path: &Path) -> (String, f64) {
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    
    // Check by filename first (higher confidence)
    match filename {
        "Cargo.toml" => return (String::from("Rust"), 0.98),
        "package.json" => return (String::from("JavaScript"), 0.95),
        "requirements.txt" => return (String::from("Python"), 0.95),
        "pyproject.toml" => return (String::from("Python"), 0.95),
        "go.mod" => return (String::from("Go"), 0.98),
        "pom.xml" => return (String::from("Java"), 0.95),
        "Gemfile" => return (String::from("Ruby"), 0.95),
        "composer.json" => return (String::from("PHP"), 0.95),
        _ => {}
    }
    
    // Check by extension
    match extension {
        "rs" => (String::from("Rust"), 0.98),
        "py" => (String::from("Python"), 0.98),
        "js" => (String::from("JavaScript"), 0.95),
        "ts" => (String::from("TypeScript"), 0.95),
        "jsx" => (String::from("JavaScript React"), 0.92),
        "tsx" => (String::from("TypeScript React"), 0.92),
        "go" => (String::from("Go"), 0.98),
        "java" => (String::from("Java"), 0.98),
        "rb" => (String::from("Ruby"), 0.98),
        "php" => (String::from("PHP"), 0.98),
        "c" | "h" => (String::from("C"), 0.98),
        "cpp" | "hpp" | "cc" => (String::from("C++"), 0.98),
        "cs" => (String::from("C#"), 0.98),
        "swift" => (String::from("Swift"), 0.98),
        "kt" => (String::from("Kotlin"), 0.98),
        "scala" => (String::from("Scala"), 0.98),
        "html" => (String::from("HTML"), 0.99),
        "css" => (String::from("CSS"), 0.99),
        "scss" | "sass" => (String::from("SCSS"), 0.98),
        "json" => (String::from("JSON"), 0.99),
        "xml" => (String::from("XML"), 0.99),
        "yaml" | "yml" => (String::from("YAML"), 0.99),
        "toml" => (String::from("TOML"), 0.99),
        "md" => (String::from("Markdown"), 0.99),
        "sh" | "bash" => (String::from("Shell"), 0.95),
        "sql" => (String::from("SQL"), 0.98),
        "graphql" => (String::from("GraphQL"), 0.98),
        "vue" => (String::from("Vue"), 0.95),
        "svelte" => (String::from("Svelte"), 0.95),
        "ex" | "exs" => (String::from("Elixir"), 0.98),
        "elm" => (String::from("Elm"), 0.98),
        "hs" => (String::from("Haskell"), 0.98),
        "clj" | "cljs" => (String::from("Clojure"), 0.98),
        "erl" => (String::from("Erlang"), 0.98),
        "lua" => (String::from("Lua"), 0.98),
        "r" => (String::from("R"), 0.98),
        "dart" => (String::from("Dart"), 0.98),
        "fs" => (String::from("F#"), 0.98),
        "vb" => (String::from("Visual Basic"), 0.98),
        "pl" | "pm" => (String::from("Perl"), 0.98),
        _ => (String::from("Unknown"), 0.5),
    }
}

pub fn get_dominant_language(files: &[FileObject]) -> String {
    let mut lang_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    
    for file in files {
        if file.language != "Unknown" && file.language != "JSON" && file.language != "Markdown" {
            *lang_counts.entry(file.language.clone()).or_insert(0) += 1;
        }
    }
    
    lang_counts.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(lang, _)| lang)
        .unwrap_or_else(|| String::from("Unknown"))
}

pub fn detect_frameworks(files: &[FileObject], base_path: &str) -> Vec<String> {
    let mut frameworks = Vec::new();
    
    for file in files {
        let filename = file.path.split('/').last().unwrap_or(&file.path);
        
        match filename {
            "Cargo.toml" => {
                if !frameworks.contains(&String::from("Cargo")) {
                    frameworks.push(String::from("Cargo"));
                }
            },
            "package.json" => {
                // Could check content for React, Vue, etc. but keeping simple for V6
                if !frameworks.contains(&String::from("Node.js")) {
                    frameworks.push(String::from("Node.js"));
                }
            },
            "requirements.txt" | "pyproject.toml" => {
                if !frameworks.contains(&String::from("Python")) {
                    frameworks.push(String::from("Python"));
                }
            },
            "pom.xml" => {
                if !frameworks.contains(&String::from("Maven")) {
                    frameworks.push(String::from("Maven"));
                }
            },
            "build.gradle" => {
                if !frameworks.contains(&String::from("Gradle")) {
                    frameworks.push(String::from("Gradle"));
                }
            },
            "Gemfile" => {
                if !frameworks.contains(&String::from("Ruby on Rails")) {
                    frameworks.push(String::from("Ruby on Rails"));
                }
            },
            _ => {}
        }
    }
    
    frameworks
}
