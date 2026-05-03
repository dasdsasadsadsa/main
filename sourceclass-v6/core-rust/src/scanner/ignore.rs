use std::path::Path;

const IGNORED_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    "dist",
    "build",
    ".venv",
    "venv",
    "__pycache__",
    ".cache",
    ".idea",
    ".vscode",
    ".DS_Store",
];

const IGNORED_FILES: &[&str] = &[
    "package-lock.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "*.png",
    "*.jpg",
    "*.jpeg",
    "*.gif",
    "*.mp4",
    "*.zip",
    "*.exe",
    "*.dll",
    "*.so",
];

pub fn should_ignore(path: &Path, base: &Path) -> bool {
    // Check if any component of the path matches ignored directories
    for component in path.components() {
        if let Some(name) = component.as_os_str().to_str() {
            if IGNORED_DIRS.contains(&name) {
                return true;
            }
        }
    }
    
    // Check filename patterns
    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
        for pattern in IGNORED_FILES {
            if pattern.starts_with("*.") {
                let ext = &pattern[1..];
                if filename.ends_with(ext) {
                    return true;
                }
            } else if filename == *pattern {
                return true;
            }
        }
    }
    
    false
}
