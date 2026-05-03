use crate::ir::schema::FileObject;

pub fn estimate_total_tokens(files: &[FileObject]) -> usize {
    files.iter().map(|f| estimate_file_tokens(f)).sum()
}

pub fn estimate_file_tokens(file: &FileObject) -> usize {
    // Simple formula: characters / 4 (approximate for English/code)
    // This is a rough estimate - actual tokenization varies by model
    let char_estimate = file.size_bytes as f64;
    (char_estimate / 4.0) as usize
}

pub fn estimate_tokens_for_content(content: &str) -> usize {
    let char_count = content.len();
    (char_count as f64 / 4.0) as usize
}
