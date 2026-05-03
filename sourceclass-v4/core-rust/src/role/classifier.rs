//! Role Classifier
//! 
//! Classifies files by their role in the project based on path, name, and content patterns.

use crate::map::schema::{FileNode, FileRole};

/// File role classifier
pub struct RoleClassifier {
    // Configuration could be added here for custom classification rules
}

impl RoleClassifier {
    /// Create a new role classifier
    pub fn new() -> Self {
        Self {}
    }
    
    /// Classify a file's role in the project
    pub fn classify(&self, file: &FileNode) -> FileRole {
        let filename = file.relative_path.to_lowercase();
        let basename = file.relative_path
            .rsplit('/')
            .next()
            .or_else(|| filename.rsplit('\\').next())
            .unwrap_or(&filename)
            .to_lowercase();
        
        // Check for test files first
        if self.is_test_file(&basename, &filename) {
            return FileRole::Test;
        }
        
        // Check for config files
        if self.is_config_file(&basename, &filename) {
            return FileRole::Config;
        }
        
        // Check for documentation
        if self.is_doc_file(&basename, &filename) {
            return FileRole::Documentation;
        }
        
        // Check for entry points
        if self.is_entry_file(&basename, &filename) {
            return FileRole::Entry;
        }
        
        // Check for brain/core files
        if self.is_brain_file(&basename, &filename) {
            return FileRole::Brain;
        }
        
        // Check for interface files
        if self.is_interface_file(&basename, &filename) {
            return FileRole::Interface;
        }
        
        // Check for connector files
        if self.is_connector_file(&basename, &filename) {
            return FileRole::Connector;
        }
        
        // Check for data model files
        if self.is_data_model_file(&basename, &filename) {
            return FileRole::DataModel;
        }
        
        // Check for utility files
        if self.is_utility_file(&basename, &filename) {
            return FileRole::Utility;
        }
        
        // Check for output/rendering files
        if self.is_output_file(&basename, &filename) {
            return FileRole::Output;
        }
        
        FileRole::Unknown
    }
    
    /// Check if file is a test file
    fn is_test_file(&self, basename: &str, filename: &str) -> bool {
        basename.contains("test") || 
        basename.contains("spec") ||
        basename.starts_with("test_") ||
        basename.ends_with("_test.rs") ||
        basename.ends_with("_test.go") ||
        filename.contains("/tests/") ||
        filename.contains("/test/") ||
        filename.contains("__tests__") ||
        filename.contains("specs")
    }
    
    /// Check if file is a configuration file
    fn is_config_file(&self, basename: &str, filename: &str) -> bool {
        // Config file names
        let config_names = [
            "config", "configuration", "settings", "options", "prefs",
            ".env", ".env.example", ".env.sample",
            "cargo.toml", "package.json", "pyproject.toml", "setup.py",
            "go.mod", "go.sum", "requirements.txt", "pipfile",
            "tsconfig.json", "jsconfig.json", "webpack.config",
            "vite.config", "rollup.config", "babel.config",
            ".gitignore", ".dockerignore", ".prettierignore",
            "dockerfile", "docker-compose",
            "makefile", "cmakelists.txt",
            ".eslintrc", ".stylelintrc", ".editorconfig",
        ];
        
        if config_names.iter().any(|n| basename.contains(n)) {
            return true;
        }
        
        // Config extensions
        let config_exts = [".toml", ".yaml", ".yml", ".ini", ".cfg", ".conf"];
        if config_exts.iter().any(|e| basename.ends_with(e)) {
            return true;
        }
        
        // Config directories
        filename.contains("/config/") || 
        filename.contains("/configs/") ||
        filename.contains("/configuration/") ||
        filename.contains("/.github/")
    }
    
    /// Check if file is documentation
    fn is_doc_file(&self, basename: &str, filename: &str) -> bool {
        basename.ends_with(".md") ||
        basename.ends_with(".rst") ||
        basename.ends_with(".txt") && (basename.contains("readme") || basename.contains("license") || basename.contains("changelog")) ||
        basename.contains("readme") ||
        basename.contains("license") ||
        basename.contains("changelog") ||
        basename.contains("contributing") ||
        basename.contains("authors") ||
        basename.contains("history") ||
        filename.contains("/docs/") ||
        filename.contains("/documentation/")
    }
    
    /// Check if file is an entry point
    fn is_entry_file(&self, basename: &str, filename: &str) -> bool {
        let entry_names = [
            "main", "index", "app", "cli", "server", "start",
            "entrypoint", "bootstrap", "launcher", "runner",
        ];
        
        if entry_names.iter().any(|n| basename.starts_with(n)) {
            return true;
        }
        
        // Entry point in specific directories
        (filename.contains("/bin/") || filename.contains("/cmd/")) && 
        !basename.contains("test")
    }
    
    /// Check if file is core/brain logic
    fn is_brain_file(&self, basename: &str, filename: &str) -> bool {
        let brain_keywords = [
            "engine", "core", "analyzer", "processor", "interpreter",
            "compiler", "executor", "scheduler", "orchestrator",
            "controller", "manager", "service", "handler",
            "logic", "business", "domain", "algorithm",
        ];
        
        brain_keywords.iter().any(|k| basename.contains(k)) ||
        filename.contains("/src/") && (basename.contains("lib") || basename.contains("mod"))
    }
    
    /// Check if file is an interface (API, routes, commands)
    fn is_interface_file(&self, basename: &str, filename: &str) -> bool {
        let interface_keywords = [
            "api", "route", "router", "endpoint", "controller",
            "command", "query", "request", "response",
            "graphql", "rest", "rpc", "grpc",
            "view", "template", "page",
        ];
        
        interface_keywords.iter().any(|k| basename.contains(k)) ||
        filename.contains("/api/") ||
        filename.contains("/routes/") ||
        filename.contains("/handlers/") ||
        filename.contains("/commands/") ||
        filename.contains("/views/") ||
        filename.contains("/templates/")
    }
    
    /// Check if file is a connector (external services)
    fn is_connector_file(&self, basename: &str, filename: &str) -> bool {
        let connector_keywords = [
            "client", "connector", "adapter", "integration",
            "database", "db", "repository", "dao",
            "cache", "queue", "broker", "publisher", "subscriber",
            "http", "grpc", "websocket", "socket",
            "aws", "gcp", "azure", "cloud",
            "stripe", "paypal", "sendgrid", "twilio",
        ];
        
        connector_keywords.iter().any(|k| basename.contains(k)) ||
        filename.contains("/clients/") ||
        filename.contains("/connectors/") ||
        filename.contains("/integrations/") ||
        filename.contains("/repositories/") ||
        filename.contains("/infrastructure/")
    }
    
    /// Check if file is a data model
    fn is_data_model_file(&self, basename: &str, filename: &str) -> bool {
        let model_keywords = [
            "model", "schema", "entity", "struct", "type",
            "record", "document", "table",
        ];
        
        model_keywords.iter().any(|k| basename.contains(k)) ||
        filename.contains("/models/") ||
        filename.contains("/schemas/") ||
        filename.contains("/entities/") ||
        filename.contains("/types/") ||
        filename.contains("/migrations/")
    }
    
    /// Check if file is a utility
    fn is_utility_file(&self, basename: &str, filename: &str) -> bool {
        let util_keywords = [
            "util", "helper", "common", "shared", "base",
            "extension", "mixin", "trait", "abstract",
            "constant", "enum", "flag",
        ];
        
        util_keywords.iter().any(|k| basename.contains(k)) ||
        filename.contains("/utils/") ||
        filename.contains("/helpers/") ||
        filename.contains("/common/") ||
        filename.contains("/shared/") ||
        filename.contains("/libs/") ||
        filename.contains("/internal/")
    }
    
    /// Check if file is for output/rendering
    fn is_output_file(&self, basename: &str, filename: &str) -> bool {
        let output_keywords = [
            "render", "report", "export", "print", "format",
            "serializer", "deserializer", "parser",
            "generator", "builder", "factory",
        ];
        
        output_keywords.iter().any(|k| basename.contains(k)) ||
        filename.contains("/render/") ||
        filename.contains("/reports/") ||
        filename.contains("/exports/") ||
        filename.contains("/output/")
    }
}

impl Default for RoleClassifier {
    fn default() -> Self {
        Self::new()
    }
}
