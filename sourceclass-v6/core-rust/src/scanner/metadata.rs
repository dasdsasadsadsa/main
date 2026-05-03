use std::fs;
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};
use crate::ir::schema::FileObject;
use super::super::analyzer::language::detect_language_from_path;

pub fn collect_file_metadata(path: &Path, base: &Path) -> Option<FileObject> {
    let metadata = fs::metadata(path).ok()?;
    
    if metadata.is_dir() {
        return None;
    }
    
    let relative_path = path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();
    
    let absolute_path = path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .to_string();
    
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    
    let size_bytes = metadata.len();
    
    let content = fs::read_to_string(path).unwrap_or_default();
    let line_count = content.lines().count();
    let is_binary = content.contains('\0');
    
    let hash = compute_hash(path);
    
    let (language, lang_confidence) = detect_language_from_path(path);
    
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let is_generated = filename.contains(".generated") || 
                       filename.contains(".min.") ||
                       relative_path.contains("/generated/");
    
    let is_test = filename.contains(".test.") || 
                  filename.contains("_test.") ||
                  relative_path.contains("/test/") ||
                  relative_path.contains("/tests/") ||
                  relative_path.contains("__tests__");
    
    let is_config = filename == "Cargo.toml" ||
                    filename == "package.json" ||
                    filename == "requirements.txt" ||
                    filename == "pyproject.toml" ||
                    filename == "go.mod" ||
                    filename == "pom.xml" ||
                    filename.ends_with(".config.js") ||
                    filename.ends_with(".config.ts") ||
                    extension == "toml" ||
                    extension == "yaml" ||
                    extension == "yml";
    
    Some(FileObject {
        path: relative_path,
        absolute_path,
        extension,
        language,
        size_bytes,
        line_count,
        hash,
        is_binary,
        is_generated,
        is_test,
        is_config,
        ignored: false,
    })
}

fn compute_hash(path: &Path) -> String {
    if let Ok(content) = fs::read(path) {
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();
        format!("{:x}", result)
    } else {
        String::from("unknown")
    }
}
