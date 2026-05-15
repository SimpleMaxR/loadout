use anyhow::Result;
use std::path::Path;

use super::Skill;

/// Parse a SKILL.md file and extract metadata from YAML frontmatter
/// Frontmatter format:
/// ---
/// name: "My Skill"
/// description: "What this skill does"
/// tags: [tag1, tag2]
/// ---
pub fn parse_skill_file(skill_file: &Path, slug: &str) -> Result<Skill> {
    let content = std::fs::read_to_string(skill_file)?;
    let mut skill = Skill::new(slug);

    // Compute content hash
    skill.content_hash = blake3::hash(content.as_bytes()).to_hex().to_string();

    // Extract YAML frontmatter manually (gray_matter crate wraps differently in Rust)
    if let Some(frontmatter) = extract_frontmatter(&content) {
        parse_frontmatter(&frontmatter, &mut skill);
    }

    // Use slug as fallback name
    if skill.name == slug || skill.name.is_empty() {
        skill.name = slug_to_name(slug);
    }

    Ok(skill)
}

/// Extract YAML frontmatter between `---` delimiters
fn extract_frontmatter(content: &str) -> Option<String> {
    let content = content.trim_start();
    if !content.starts_with("---") {
        return None;
    }

    let after_first = &content[3..];
    // Find the closing ---
    let end = after_first.find("\n---")?;
    Some(after_first[..end].trim().to_string())
}

/// Parse YAML frontmatter into skill metadata
/// Uses simple line-by-line parsing to avoid heavy YAML dependency
fn parse_frontmatter(frontmatter: &str, skill: &mut Skill) {
    // We use serde_json for JSON-compatible YAML subset
    // For robustness, use the gray_matter crate's parsed output if available
    // Simple implementation: parse key: value lines
    for line in frontmatter.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim().trim_matches('"').trim_matches('\'');

            match key.as_str() {
                "name" => skill.name = value.to_string(),
                "description" | "desc" => skill.description = value.to_string(),
                "tags" => {
                    // Parse inline array: [tag1, tag2] or tag1, tag2
                    let cleaned = value.trim_matches('[').trim_matches(']');
                    skill.tags = cleaned
                        .split(',')
                        .map(|t| t.trim().trim_matches('"').trim_matches('\'').to_string())
                        .filter(|t| !t.is_empty())
                        .collect();
                }
                _ => {}
            }
        }
    }
}

/// Convert a slug like "git-workflow" to "Git Workflow"
fn slug_to_name(slug: &str) -> String {
    slug.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
