use std::fs;
use std::path::Path;
use crate::ir::schema::{FileObject, RiskFlagObject};

pub fn detect_secret_risks(file: &FileObject, base_path: &str) -> Option<RiskFlagObject> {
    let filename = file.path.split('/').last().unwrap_or(&file.path);
    let path = Path::new(base_path).join(&file.path);
    
    // Check for .env files
    if filename == ".env" || filename.ends_with(".env") || filename.starts_with(".env.") {
        return Some(RiskFlagObject {
            file: file.path.clone(),
            risk_type: String::from("env_file"),
            severity: String::from("High"),
            confidence: 0.95,
            reason: String::from("Environment variable file detected - may contain secrets"),
            safe_action: String::from("Do not send this file to LLM. Add to ignore rules."),
        });
    }
    
    // Check for key/certificate files
    if filename.ends_with(".pem") || filename.ends_with(".key") || filename.ends_with(".crt") {
        return Some(RiskFlagObject {
            file: file.path.clone(),
            risk_type: String::from("secret_leak"),
            severity: String::from("Critical"),
            confidence: 0.98,
            reason: String::from("Private key or certificate file detected"),
            safe_action: String::from("Never commit or send to LLM. Ensure proper access controls."),
        });
    }
    
    // Check for secret/credential patterns in code files
    if file.language == "Python" || file.language == "JavaScript" || 
       file.language == "TypeScript" || file.language == "Rust" ||
       file.language == "Go" || file.language == "Java" {
        
        if let Ok(content) = fs::read_to_string(&path) {
            let content_lower = content.to_lowercase();
            
            // Check for API key patterns
            if content_lower.contains("api_key") && content_lower.contains("=") {
                if is_likely_hardcoded_secret(&content) {
                    return Some(RiskFlagObject {
                        file: file.path.clone(),
                        risk_type: String::from("secret_leak"),
                        severity: String::from("High"),
                        confidence: 0.75,
                        reason: String::from("Potential hardcoded API key pattern detected"),
                        safe_action: String::from("Use environment variables instead of hardcoding secrets."),
                    });
                }
            }
            
            // Check for token patterns
            if (content_lower.contains("token") || content_lower.contains("secret")) && 
               content_lower.contains("=") {
                if is_likely_hardcoded_secret(&content) {
                    return Some(RiskFlagObject {
                        file: file.path.clone(),
                        risk_type: String::from("secret_leak"),
                        severity: String::from("Medium"),
                        confidence: 0.65,
                        reason: String::from("Potential hardcoded token/secret pattern detected"),
                        safe_action: String::from("Verify if this is a real secret. Use environment variables."),
                    });
                }
            }
            
            // Check for AWS-style keys
            if content.contains("AKIA") || content.contains("sk-") || 
               content.contains("ghp_") || content.contains("github_pat_") {
                return Some(RiskFlagObject {
                    file: file.path.clone(),
                    risk_type: String::from("secret_leak"),
                    severity: String::from("Critical"),
                    confidence: 0.90,
                    reason: String::from("Cloud provider or service API key pattern detected"),
                    safe_action: String::from("Immediately rotate this credential. Never commit secrets."),
                });
            }
        }
    }
    
    None
}

fn is_likely_hardcoded_secret(content: &str) -> bool {
    // Check if the assignment looks like a hardcoded value vs. environment variable lookup
    let lines: Vec<&str> = content.lines().collect();
    
    for line in lines.iter().take(20) { // Check first 20 lines
        let lower = line.to_lowercase();
        
        // Skip if it's using environment variables
        if lower.contains("os.environ") || lower.contains("process.env") || 
           lower.contains("getenv") || lower.contains("std::env") {
            continue;
        }
        
        // Check for string literals with significant length
        if lower.contains("\"") || lower.contains("'") {
            // Count quotes to see if there's a string assignment
            let quote_count = line.matches('"').count() + line.matches('\'').count();
            if quote_count >= 2 {
                return true;
            }
        }
    }
    
    false
}
