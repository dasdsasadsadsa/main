mod scanner;
mod analyzer;
mod graph;
mod risk;
mod context;
mod ir;
mod utils;

use std::env;
use std::process;

use scanner::walk::scan_directory;
use analyzer::language::detect_languages;
use analyzer::entrypoint::detect_entrypoints;
use analyzer::file_role::classify_file_roles;
use analyzer::dependency::extract_dependencies;
use risk::detect_risks;
use graph::builder::build_structure_graph;
use context::planner::plan_context;
use ir::builder::build_project_ir;
use utils::json::print_json;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: sourceclass-core <command> <path>");
        eprintln!("Commands: scan, ir, context, risk");
        process::exit(1);
    }
    
    let command = &args[1];
    let path = &args[2];
    
    let result = match command.as_str() {
        "scan" => {
            let files = scan_directory(path);
            print_json(&files)
        },
        "ir" => {
            let files = scan_directory(path);
            let languages = detect_languages(&files);
            let entrypoints = detect_entrypoints(&files, path);
            let file_roles = classify_file_roles(&files);
            let dependencies = extract_dependencies(&files, path);
            let risks = detect_risks(&files, path);
            let graph = build_structure_graph(&files, &entrypoints);
            let token_estimate = context::token_estimator::estimate_total_tokens(&files);
            let context_plan = plan_context(&files, &entrypoints, &file_roles, &risks);
            
            let ir = build_project_ir(
                path,
                files,
                languages,
                entrypoints,
                file_roles,
                dependencies,
                risks,
                graph,
                token_estimate,
                context_plan,
            );
            print_json(&ir)
        },
        "context" => {
            let files = scan_directory(path);
            let entrypoints = detect_entrypoints(&files, path);
            let file_roles = classify_file_roles(&files);
            let risks = detect_risks(&files, path);
            let context_plan = plan_context(&files, &entrypoints, &file_roles, &risks);
            print_json(&context_plan)
        },
        "risk" => {
            let files = scan_directory(path);
            let risks = detect_risks(&files, path);
            print_json(&risks)
        },
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Valid commands: scan, ir, context, risk");
            process::exit(1);
        }
    };
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
