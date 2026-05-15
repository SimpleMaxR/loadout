use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::driver::{detector, ToolDriver};
use crate::mcp::{formats as mcp_formats, McpServer, PresenceState};
use crate::skills::{formats as skill_formats, scanner, SkillPresence};
use crate::store::CentralStore;

use super::{
    hasher,
    report::SyncReport,
};

pub struct SyncEngine {
    pub store: CentralStore,
    pub drivers: Vec<ToolDriver>,
}

impl SyncEngine {
    pub fn new(store: CentralStore, drivers: Vec<ToolDriver>) -> Self {
        Self { store, drivers }
    }

    /// Installed (detected) drivers only
    pub fn installed_drivers(&self) -> Vec<&ToolDriver> {
        self.drivers.iter().filter(|d| detector::is_installed(d)).collect()
    }

    // ── MCP ────────────────────────────────────────────────────────────────

    /// Read all MCP servers across all installed tools
    pub fn read_all_mcp(&self) -> HashMap<String, McpServer> {
        let mut merged: HashMap<String, McpServer> = HashMap::new();

        for driver in self.installed_drivers() {
            match mcp_formats::read_servers(&driver.mcp.format) {
                Ok(servers) => {
                    for (name, mut server) in servers {
                        let entry = merged.entry(name.clone()).or_insert_with(|| {
                            server.presence.insert(driver.id.clone(), PresenceState::Present);
                            server.clone()
                        });
                        if entry.name != name {
                            // Shouldn't happen, but guard
                            continue;
                        }
                        entry
                            .presence
                            .insert(driver.id.clone(), PresenceState::Present);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: failed to read MCP for {}: {}", driver.id, e);
                }
            }
        }

        // Mark absent tools
        for server in merged.values_mut() {
            for driver in self.installed_drivers() {
                server
                    .presence
                    .entry(driver.id.clone())
                    .or_insert(PresenceState::Absent);
            }
        }

        merged
    }

    /// Sync an MCP server to all installed tools (or specified subset)
    pub fn sync_mcp_to_all(&self, server: &McpServer) -> SyncReport {
        let mut report = SyncReport::new();

        for driver in self.installed_drivers() {
            match mcp_formats::write_server(&driver.mcp.format, server) {
                Ok(()) => report.add(SyncReport::success(&driver.id, &server.name)),
                Err(e) => report.add(SyncReport::failed(&driver.id, &server.name, e.to_string())),
            }
        }

        report
    }

    /// Sync an MCP server to a single specific tool
    pub fn sync_mcp_to_tool(&self, server_name: &str, tool_id: &str) -> Result<SyncReport> {
        let driver = self
            .drivers
            .iter()
            .find(|d| d.id == tool_id)
            .ok_or_else(|| anyhow::anyhow!("driver not found: {}", tool_id))?;

        // Read the server config from any tool that has it
        let all = self.read_all_mcp();
        let server = all
            .get(server_name)
            .ok_or_else(|| anyhow::anyhow!("server not found: {}", server_name))?;

        let mut report = SyncReport::new();
        match mcp_formats::write_server(&driver.mcp.format, server) {
            Ok(()) => report.add(SyncReport::success(tool_id, server_name)),
            Err(e) => report.add(SyncReport::failed(tool_id, server_name, e.to_string())),
        }
        Ok(report)
    }

    /// Remove an MCP server from all installed tools
    pub fn remove_mcp_from_all(&self, name: &str) -> SyncReport {
        let mut report = SyncReport::new();

        for driver in self.installed_drivers() {
            match mcp_formats::remove_server(&driver.mcp.format, name) {
                Ok(()) => report.add(SyncReport::success(&driver.id, name)),
                Err(e) => report.add(SyncReport::failed(&driver.id, name, e.to_string())),
            }
        }

        report
    }

    /// Remove an MCP server from a single specific tool
    pub fn remove_mcp_from_tool(&self, name: &str, tool_id: &str) -> Result<SyncReport> {
        let driver = self
            .drivers
            .iter()
            .find(|d| d.id == tool_id)
            .ok_or_else(|| anyhow::anyhow!("driver not found: {}", tool_id))?;

        let mut report = SyncReport::new();
        match mcp_formats::remove_server(&driver.mcp.format, name) {
            Ok(()) => report.add(SyncReport::success(tool_id, name)),
            Err(e) => report.add(SyncReport::failed(tool_id, name, e.to_string())),
        }
        Ok(report)
    }

    // ── Skills ─────────────────────────────────────────────────────────────

    /// Read all skills from the central store + detect presence in each tool.
    /// Also discovers skills that live in tool directories but haven't been
    /// imported to the central store yet, showing them as absent from the store.
    pub fn read_all_skills(&self) -> Vec<crate::skills::Skill> {
        let store_dir = &self.store.layout.skills_dir;
        let mut skill_map: HashMap<String, crate::skills::Skill> = HashMap::new();

        // ── 1. Seed from central store ─────────────────────────────────────
        for skill in scanner::scan_dir(store_dir, "SKILL.md").unwrap_or_default() {
            skill_map.insert(skill.slug.clone(), skill);
        }

        // ── 2. Discover skills in each installed tool ──────────────────────
        for driver in self.installed_drivers() {
            let tool_skills =
                scanner::scan_dir(&driver.skills.path, &driver.skills.skill_entry)
                    .unwrap_or_default();

            for tool_skill in tool_skills {
                skill_map
                    .entry(tool_skill.slug.clone())
                    .or_insert(tool_skill);
            }
        }

        // ── 3. Annotate each skill with presence across all tools ──────────
        let mut skills: Vec<crate::skills::Skill> = skill_map.into_values().collect();

        for skill in &mut skills {
            let skill_source = store_dir.join(&skill.slug);

            for driver in self.installed_drivers() {
                let target = driver.skills.path.join(&skill.slug);
                let presence = detect_skill_presence(&skill_source, &target, &driver.skills.skill_entry);
                skill.presence.insert(driver.id.clone(), presence);
            }
        }

        skills.sort_by(|a, b| a.slug.cmp(&b.slug));
        skills
    }

    /// Import a skill from a tool into the central store
    pub fn import_skill(&self, tool_id: &str, slug: &str) -> Result<SyncReport> {
        let driver = self
            .drivers
            .iter()
            .find(|d| d.id == tool_id)
            .ok_or_else(|| anyhow::anyhow!("driver not found: {}", tool_id))?;

        let source = driver.skills.path.join(slug);
        if !source.exists() {
            anyhow::bail!("skill '{}' not found in tool '{}'", slug, tool_id);
        }

        let store_target = self.store.skill_path(slug);

        // Copy to central store
        skill_formats::copy::sync(&source, &store_target, None)?;

        // Now sync to all tools
        Ok(self.sync_skill_to_all(slug))
    }

    /// Sync a skill from the central store to all installed tools.
    /// If the skill isn't in the central store yet, auto-import it from the
    /// first tool that has it.
    pub fn sync_skill_to_all(&self, slug: &str) -> SyncReport {
        let mut report = SyncReport::new();
        let source = self.store.skill_path(slug);

        if !source.exists() {
            // Try to bootstrap the central store from a tool that has this skill
            let donor = self.installed_drivers().into_iter().find(|d| {
                d.skills.path.join(slug).join(&d.skills.skill_entry).exists()
            });
            if let Some(driver) = donor {
                let tool_source = driver.skills.path.join(slug);
                if let Err(e) = skill_formats::copy::sync(&tool_source, &source, None) {
                    report.add(SyncReport::failed("store", slug, format!("auto-import failed: {}", e)));
                    return report;
                }
            } else {
                // Skill doesn't exist anywhere
                return report;
            }
        }

        for driver in self.installed_drivers() {
            let result = skill_formats::sync_skill(
                &driver.skills.format,
                &source,
                &driver.skills.path,
                slug,
                driver.skills.file_filter.as_ref(),
            );

            match result {
                Ok(()) => report.add(SyncReport::success(&driver.id, slug)),
                Err(e) => report.add(SyncReport::failed(&driver.id, slug, e.to_string())),
            }
        }

        report
    }

    /// Remove a skill from all tools (and optionally the central store)
    pub fn remove_skill_from_all(&self, slug: &str, remove_from_store: bool) -> SyncReport {
        let mut report = SyncReport::new();

        for driver in self.installed_drivers() {
            let result = skill_formats::remove_skill(&driver.skills.format, &driver.skills.path, slug);
            match result {
                Ok(()) => report.add(SyncReport::success(&driver.id, slug)),
                Err(e) => report.add(SyncReport::failed(&driver.id, slug, e.to_string())),
            }
        }

        if remove_from_store {
            let store_path = self.store.skill_path(slug);
            if store_path.exists() {
                if let Err(e) = std::fs::remove_dir_all(&store_path) {
                    report.add(SyncReport::failed("store", slug, e.to_string()));
                } else {
                    report.add(SyncReport::success("store", slug));
                }
            }
        }

        report
    }
}

/// Detect how a skill is present in a tool's directory
fn detect_skill_presence(source: &PathBuf, target: &PathBuf, skill_entry: &str) -> SkillPresence {
    if !target.exists() && !target.is_symlink() {
        return SkillPresence::Absent;
    }

    if target.is_symlink() {
        // Check if symlink points to the canonical source
        if let Ok(link_target) = std::fs::read_link(target) {
            if link_target == *source {
                return SkillPresence::Symlinked;
            }
        }
        // If the central store doesn't have this skill, the symlink is still
        // valid (it just points somewhere else, e.g. another tool's copy).
        // Only flag as drifted if the central store *does* have a source.
        if source.exists() {
            return SkillPresence::Drifted;
        }
        return SkillPresence::Copied;
    }

    // If the central store doesn't have this skill yet, the tool's copy is
    // the only version — treat it as a valid copy (no drift is possible).
    if !source.exists() {
        return SkillPresence::Copied;
    }

    // Both exist — check for content drift
    if hasher::skills_match(source, target, skill_entry) {
        SkillPresence::Copied
    } else {
        SkillPresence::Drifted
    }
}
