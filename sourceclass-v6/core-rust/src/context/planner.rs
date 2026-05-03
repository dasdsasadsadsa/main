use crate::ir::schema::{FileObject, EntrypointObject, FileRoleObject, RiskFlagObject, ContextPlanItem};
use super::token_estimator::estimate_file_tokens;
use super::priority::get_priority_for_role;

pub fn plan_context(
    files: &[FileObject],
    entrypoints: &[EntrypointObject],
    file_roles: &[FileRoleObject],
    risks: &[RiskFlagObject],
) -> Vec<ContextPlanItem> {
    let mut plan = Vec::new();
    
    // Create a map of risky files to exclude
    let risky_files: std::collections::HashSet<&str> = risks
        .iter()
        .filter(|r| r.severity == "Critical" || r.severity == "High")
        .map(|r| r.file.as_str())
        .collect();
    
    // Create a map of file roles for quick lookup
    let role_map: std::collections::HashMap<&str, &FileRoleObject> = file_roles
        .iter()
        .map(|r| (r.file.as_str(), r))
        .collect();
    
    // Create a map of entrypoint files
    let entrypoint_files: std::collections::HashSet<&str> = entrypoints
        .iter()
        .map(|e| e.file.as_str())
        .collect();
    
    for file in files {
        // Skip risky files
        if risky_files.contains(file.path.as_str()) {
            continue;
        }
        
        // Get role for this file
        let role = role_map.get(file.path.as_str());
        let role_name = role.map(|r| r.role.as_str()).unwrap_or("Utility");
        
        // Determine priority
        let (priority, reason) = if entrypoint_files.contains(file.path.as_str()) {
            (String::from("MustSend"), String::from("Execution entrypoint - essential for understanding"))
        } else {
            get_priority_for_role(role_name)
        };
        
        let estimated_tokens = estimate_file_tokens(file);
        
        plan.push(ContextPlanItem {
            rank: 0, // Will be set after sorting
            file: file.path.clone(),
            priority,
            reason,
            estimated_tokens,
        });
    }
    
    // Sort by priority order
    plan.sort_by(|a, b| {
        let priority_order = |p: &str| match p {
            "MustSend" => 1,
            "ShouldSend" => 2,
            "SendIfNeeded" => 3,
            "IgnoreFirst" => 4,
            "NeverSend" => 5,
            _ => 6,
        };
        priority_order(&a.priority).cmp(&priority_order(&b.priority))
    });
    
    // Assign ranks
    for (i, item) in plan.iter_mut().enumerate() {
        item.rank = i + 1;
    }
    
    plan
}
