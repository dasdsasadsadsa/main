pub mod secrets;
pub mod unsafe_patterns;
pub mod file_risks;

use std::path::Path;
use crate::ir::schema::{FileObject, RiskFlagObject};
use secrets::detect_secret_risks;
use file_risks::detect_file_risks;

pub fn detect_risks(files: &[FileObject], base_path: &str) -> Vec<RiskFlagObject> {
    let mut risks = Vec::new();
    
    for file in files {
        // Detect secret-related risks
        if let Some(risk) = detect_secret_risks(file, base_path) {
            risks.push(risk);
        }
        
        // Detect file-based risks
        if let Some(risk) = detect_file_risks(file) {
            risks.push(risk);
        }
    }
    
    // Sort by severity (Critical > High > Medium > Low)
    risks.sort_by(|a, b| {
        let severity_order = |s: &str| match s {
            "Critical" => 4,
            "High" => 3,
            "Medium" => 2,
            "Low" => 1,
            _ => 0,
        };
        severity_order(&b.severity).cmp(&severity_order(&a.severity))
    });
    
    risks
}
