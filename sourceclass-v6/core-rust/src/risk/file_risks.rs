use crate::ir::schema::{FileObject, RiskFlagObject};

pub fn detect_file_risks(file: &FileObject) -> Option<RiskFlagObject> {
    // Large file risk
    if file.size_bytes > 1_000_000 { // 1MB
        return Some(RiskFlagObject {
            file: file.path.clone(),
            risk_type: String::from("large_file"),
            severity: String::from("Low"),
            confidence: 0.99,
            reason: format!("Large file ({} bytes) - may waste tokens", file.size_bytes),
            safe_action: String::from("Consider excluding from LLM context unless necessary."),
        });
    }
    
    // Binary file risk
    if file.is_binary {
        return Some(RiskFlagObject {
            file: file.path.clone(),
            risk_type: String::from("binary_file"),
            severity: String::from("Low"),
            confidence: 0.95,
            reason: String::from("Binary file detected - not suitable for LLM analysis"),
            safe_action: String::from("Exclude from LLM context. Binary files cannot be analyzed as text."),
        });
    }
    
    // Generated file risk
    if file.is_generated {
        return Some(RiskFlagObject {
            file: file.path.clone(),
            risk_type: String::from("generated_noise"),
            severity: String::from("Low"),
            confidence: 0.85,
            reason: String::from("Generated file detected - likely not useful for understanding logic"),
            safe_action: String::from("Skip generated files when analyzing source code logic."),
        });
    }
    
    None
}
