//! Context Planner
//! 
//! Plans and prepares minimized context for LLM usage.

use crate::map::builder::ProjectMap;
use crate::map::schema::CentralityLevel;

/// Context planner for LLM token optimization
pub struct ContextPlanner {
    // Configuration for token limits, etc.
}

impl ContextPlanner {
    /// Create a new context planner
    pub fn new() -> Self {
        Self {}
    }
    
    /// Plan context for the entire project
    pub fn plan(&self, map: &ProjectMap) -> Result<Vec<crate::context::ContextPlanItem>, String> {
        let ir = map.to_ir();
        
        let mut items: Vec<crate::context::ContextPlanItem> = Vec::new();
        
        // Sort files by centrality and role importance
        let mut sorted_files: Vec<_> = ir.files.iter().collect();
        sorted_files.sort_by(|a, b| {
            // First by centrality (higher first)
            b.centrality_score.partial_cmp(&a.centrality_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        let mut rank = 0;
        for file in sorted_files {
            let priority = self.determine_priority(file);
            let reason = self.generate_priority_reason(file, &priority);
            
            // Estimate tokens (rough estimate: ~4 chars per token average)
            let estimated_tokens = (file.size_bytes as f64 / 4.0) as usize;
            
            items.push(crate::context::ContextPlanItem {
                rank,
                file: file.relative_path.clone(),
                priority,
                reason,
                estimated_tokens,
            });
            
            rank += 1;
        }
        
        Ok(items)
    }
    
    /// Determine priority level for a file
    fn determine_priority(&self, file: &crate::map::schema::FileNode) -> String {
        use crate::map::schema::FileRole;
        
        // High centrality files are must-send
        if file.centrality_score > 0.8 {
            return "MustSend".to_string();
        }
        
        match &file.role {
            FileRole::Entry | FileRole::Brain => "MustSend".to_string(),
            FileRole::Interface | FileRole::Connector => "ShouldSend".to_string(),
            FileRole::DataModel | FileRole::Config => "SendIfNeeded".to_string(),
            FileRole::Utility | FileRole::Output => "SendIfNeeded".to_string(),
            FileRole::Test | FileRole::Documentation => "IgnoreFirst".to_string(),
            FileRole::Unknown => {
                if file.centrality_score > 0.5 {
                    "ShouldSend".to_string()
                } else {
                    "IgnoreFirst".to_string()
                }
            }
        }
    }
    
    /// Generate human-readable reason for priority
    fn generate_priority_reason(&self, file: &crate::map::schema::FileNode, priority: &str) -> String {
        let role_str = format!("{:?}", file.role);
        
        match priority {
            "MustSend" => format!(
                "Critical {}: {:.0}% centrality score, essential for understanding project structure",
                role_str, file.centrality_score * 100.0
            ),
            "ShouldSend" => format!(
                "Important {}: {:.0}% centrality, provides key context for {}",
                role_str, file.centrality_score * 100.0, file.language
            ),
            "SendIfNeeded" => format!(
                "Reference {}: {:.0}% centrality, consult when needed for specific details",
                role_str, file.centrality_score * 100.0
            ),
            "IgnoreFirst" => format!(
                "Low priority {}: {:.0}% centrality, can be skipped for initial understanding",
                role_str, file.centrality_score * 100.0
            ),
            _ => format!("{} with {:.0}% centrality", role_str, file.centrality_score * 100.0),
        }
    }
}

impl Default for ContextPlanner {
    fn default() -> Self {
        Self::new()
    }
}
