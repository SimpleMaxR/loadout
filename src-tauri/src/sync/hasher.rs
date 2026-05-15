use std::path::Path;

/// Compute blake3 hash of a file's content
pub fn hash_file(path: &Path) -> Option<String> {
    let content = std::fs::read(path).ok()?;
    Some(blake3::hash(&content).to_hex().to_string())
}

/// Compute blake3 hash of a string
pub fn hash_str(s: &str) -> String {
    blake3::hash(s.as_bytes()).to_hex().to_string()
}

/// Check if two skill directories have identical SKILL.md content
pub fn skills_match(source: &Path, target: &Path, skill_entry: &str) -> bool {
    let source_file = source.join(skill_entry);
    let target_file = target.join(skill_entry);

    match (hash_file(&source_file), hash_file(&target_file)) {
        (Some(h1), Some(h2)) => h1 == h2,
        _ => false,
    }
}
