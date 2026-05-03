//! SourceClass V4 Core Engine
//! 
//! A mapping-based search engine that turns codebases into explorable,
//! interactive maps with LLM-powered intelligence.

mod map;
mod symbol;
mod role;
mod context;

use std::env;
use std::process;
use std::path::PathBuf;

use map::builder::ProjectMap;
use symbol::extractor::SymbolExtractor;
use role::classifier::RoleClassifier;
use context::planner::ContextPlanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("SourceClass V4 - Mapping Search Engine");
        eprintln!();
        eprintln!("Usage: sourceclass-v4-core <command> <path> [options]");
        eprintln!();
        eprintln!("Commands:");
        eprintln!("  map      Build and output the project map");
        eprintln!("  summary  Get structural summary of a file");
        eprintln!("  symbols  Extract symbols from a file");
        eprintln!("  context  Prepare minimized context for LLM");
        eprintln!("  explain  Generate explanation context for an element");
        process::exit(1);
    }
    
    let command = &args[1];
    let path = &args[2];
    
    let result = match command.as_str() {
        "map" => cmd_map(path),
        "summary" => {
            let file = args.get(3).expect("File path required");
            cmd_summary(path, file)
        },
        "symbols" => {
            let file = args.get(3).expect("File path required");
            cmd_symbols(path, file)
        },
        "context" => cmd_context(path),
        "explain" => {
            let file = args.get(3).expect("File path required");
            let element = args.get(4).expect("Element identifier required");
            cmd_explain(path, file, element)
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            process::exit(1)
        }
    };
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn cmd_map(path: &str) -> Result<(), String> {
    let root_path = PathBuf::from(path);
    
    if !root_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    
    // Build the project map
    let mut map_builder = ProjectMap::new(root_path.clone());
    map_builder.build()?;
    
    // Output as JSON
    let json = serde_json::to_string_pretty(&map_builder.to_ir())
        .map_err(|e| format!("JSON serialization error: {}", e))?;
    
    println!("{}", json);
    Ok(())
}

fn cmd_summary(path: &str, file: &str) -> Result<(), String> {
    let root_path = PathBuf::from(path);
    let file_path = root_path.join(file);
    
    if !file_path.exists() {
        return Err(format!("File does not exist: {}", file));
    }
    
    // Build map to get file context
    let mut map_builder = ProjectMap::new(root_path.clone());
    map_builder.build()?;
    
    // Get file summary
    let summary = map_builder.get_file_summary(file)?;
    
    let json = serde_json::to_string_pretty(&summary)
        .map_err(|e| format!("JSON serialization error: {}", e))?;
    
    println!("{}", json);
    Ok(())
}

fn cmd_symbols(path: &str, file: &str) -> Result<(), String> {
    let root_path = PathBuf::from(path);
    let file_path = root_path.join(file);
    
    if !file_path.exists() {
        return Err(format!("File does not exist: {}", file));
    }
    
    // Extract symbols from file
    let extractor = SymbolExtractor::new();
    let symbols = extractor.extract_from_file(&file_path)?;
    
    let json = serde_json::to_string_pretty(&symbols)
        .map_err(|e| format!("JSON serialization error: {}", e))?;
    
    println!("{}", json);
    Ok(())
}

fn cmd_context(path: &str) -> Result<(), String> {
    let root_path = PathBuf::from(path);
    
    if !root_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    
    // Build map
    let mut map_builder = ProjectMap::new(root_path.clone());
    map_builder.build()?;
    
    // Plan context for LLM
    let planner = ContextPlanner::new();
    let context_plan = planner.plan(&map_builder)?;
    
    let json = serde_json::to_string_pretty(&context_plan)
        .map_err(|e| format!("JSON serialization error: {}", e))?;
    
    println!("{}", json);
    Ok(())
}

fn cmd_explain(path: &str, file: &str, element: &str) -> Result<(), String> {
    let root_path = PathBuf::from(path);
    let file_path = root_path.join(file);
    
    if !file_path.exists() {
        return Err(format!("File does not exist: {}", file));
    }
    
    // Build map
    let mut map_builder = ProjectMap::new(root_path.clone());
    map_builder.build()?;
    
    // Extract symbols to find the element
    let extractor = SymbolExtractor::new();
    let symbols = extractor.extract_from_file(&file_path)?;
    
    // Find the element in symbols
    let element_info = symbols.iter()
        .find(|s| s.name == element || s.id.contains(element))
        .ok_or_else(|| format!("Element not found: {}", element))?;
    
    // Prepare explanation context
    let explain_context = context::ExplainContext {
        file_path: file.to_string(),
        element: element_info.clone(),
        file_content: std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read file: {}", e))?,
        project_skeleton: map_builder.get_skeleton(),
        centrality: map_builder.calculate_centrality(element_info),
    };
    
    let json = serde_json::to_string_pretty(&explain_context)
        .map_err(|e| format!("JSON serialization error: {}", e))?;
    
    println!("{}", json);
    Ok(())
}
