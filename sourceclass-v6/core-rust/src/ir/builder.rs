use crate::ir::schema::{ProjectIR, ProjectObject, FileObject, EntrypointObject, DependencyObject, FileRoleObject, RiskFlagObject, EditZoneObject, StructureGraph, ContextPlanItem, SummaryStats};
use super::super::analyzer::language::{get_dominant_language, detect_frameworks};

pub fn build_project_ir(
    base_path: &str,
    files: Vec<FileObject>,
    _languages: Vec<crate::ir::schema::LanguageDetection>,
    entrypoints: Vec<EntrypointObject>,
    file_roles: Vec<FileRoleObject>,
    dependencies: Vec<DependencyObject>,
    risk_flags: Vec<RiskFlagObject>,
    structure_graph: StructureGraph,
    token_estimate: usize,
    context_plan: Vec<ContextPlanItem>,
) -> ProjectIR {
    // Extract project name from path
    let name = base_path
        .rsplit('/')
        .next()
        .unwrap_or(base_path)
        .to_string();
    
    // Get unique languages
    let mut languages: Vec<String> = files
        .iter()
        .map(|f| f.language.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .filter(|l| l != "Unknown")
        .collect();
    languages.sort();
    
    let dominant_language = get_dominant_language(&files);
    let detected_frameworks = detect_frameworks(&files, base_path);
    
    // Calculate confidence based on analysis completeness
    let confidence = if files.is_empty() {
        0.0
    } else {
        let analyzed_ratio = files.len() as f64 / (files.len() + 10) as f64; // Mock ignored count
        (analyzed_ratio * 0.9).min(0.95)
    };
    
    // Build edit zones from file roles and risks
    let edit_zones = build_edit_zones(&files, &file_roles, &risk_flags);
    
    // Build summary stats
    let summary_stats = SummaryStats {
        total_files: files.len(),
        analyzed_files: files.iter().filter(|f| !f.ignored).count(),
        ignored_files: files.iter().filter(|f| f.ignored).count(),
        large_files: files.iter().filter(|f| f.size_bytes > 1_000_000).count(),
        risk_count: risk_flags.len(),
    };
    
    ProjectIR {
        project: ProjectObject {
            name,
            path: base_path.to_string(),
            languages,
            dominant_language,
            detected_frameworks,
            confidence,
        },
        files,
        entrypoints,
        dependencies,
        file_roles,
        risk_flags,
        edit_zones,
        structure_graph,
        token_estimate,
        context_plan,
        summary_stats,
    }
}

fn build_edit_zones(
    files: &[FileObject],
    file_roles: &[FileRoleObject],
    risks: &[RiskFlagObject],
) -> Vec<EditZoneObject> {
    let mut zones = Vec::new();
    
    let risky_files: std::collections::HashSet<&str> = risks
        .iter()
        .filter(|r| r.severity == "Critical" || r.severity == "High")
        .map(|r| r.file.as_str())
        .collect();
    
    for file in files {
        let (zone, risk_level) = if risky_files.contains(file.path.as_str()) {
            (String::from("DO_NOT_TOUCH_YET"), String::from("High"))
        } else if file.is_test {
            (String::from("SAFE_TO_MODIFY"), String::from("Low"))
        } else if file.is_config {
            (String::from("MODIFY_WITH_CAUTION"), String::from("Medium"))
        } else {
            (String::from("SAFE_TO_MODIFY"), String::from("Low"))
        };
        
        let reason = match zone.as_str() {
            "DO_NOT_TOUCH_YET" => String::from("High-risk file - review before modification"),
            "MODIFY_WITH_CAUTION" => String::from("Configuration changes may affect runtime behavior"),
            "SAFE_TO_MODIFY" => String::from("Standard source file - safe to modify with tests"),
            _ => String::from("Unknown risk level"),
        };
        
        zones.push(EditZoneObject {
            file: file.path.clone(),
            zone,
            reason,
            risk_level,
        });
    }
    
    zones
}
