use std::fs;
use std::path::{Path, PathBuf};
use walkdir::{WalkDir, DirEntry};
use super::ignore::should_ignore;
use super::metadata::collect_file_metadata;
use crate::ir::schema::FileObject;

pub fn scan_directory(base_path: &str) -> Vec<FileObject> {
    let base = Path::new(base_path);
    let mut files = Vec::new();
    
    if !base.exists() || !base.is_dir() {
        return files;
    }
    
    for entry in WalkDir::new(base)
        .into_iter()
        .filter_entry(|e| !should_ignore(e.path(), base))
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            if let Some(file_obj) = collect_file_metadata(entry.path(), base) {
                files.push(file_obj);
            }
        }
    }
    
    files
}
