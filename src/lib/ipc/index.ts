import { invoke } from '@tauri-apps/api/core';

export interface ToolInfo {
  id: string;
  display_name: string;
  icon: string;
  installed: boolean;
  mcp_path?: string;
  skills_path: string;
}

export interface McpServerDto {
  name: string;
  transport: string;
  config: Record<string, unknown>;
  enabled: boolean;
  presence: Record<string, string>;
}

export interface AddMcpServerRequest {
  name: string;
  config: Record<string, unknown>;
}

export interface SkillDto {
  slug: string;
  name: string;
  description: string;
  tags: string[];
  content_hash: string;
  presence: Record<string, string>;
}

export interface SyncResult {
  tool_id: string;
  item_name: string;
  outcome: 'success' | 'skipped' | 'failed';
  message?: string;
}

export interface SyncReport {
  results: SyncResult[];
}

// ── Tools ───────────────────────────────────────────────────────────────────

export async function listTools(): Promise<ToolInfo[]> {
  return invoke<ToolInfo[]>('list_tools');
}

export interface UpdateToolPathsRequest {
  tool_id: string;
  mcp_path?: string;
  skills_path?: string;
}

export async function updateToolPaths(req: UpdateToolPathsRequest): Promise<void> {
  return invoke<void>('update_tool_paths', { req });
}

// ── MCP ─────────────────────────────────────────────────────────────────────

export async function listMcpServers(): Promise<McpServerDto[]> {
  return invoke<McpServerDto[]>('list_mcp_servers');
}

export async function addMcpServer(req: AddMcpServerRequest): Promise<SyncReport> {
  return invoke<SyncReport>('add_mcp_server', { req });
}

export async function syncMcpToTool(name: string, toolId: string): Promise<SyncReport> {
  return invoke<SyncReport>('sync_mcp_to_tool', { name, toolId });
}

export async function removeMcpFromTool(name: string, toolId: string): Promise<SyncReport> {
  return invoke<SyncReport>('remove_mcp_from_tool', { name, toolId });
}

export async function removeMcpServer(name: string): Promise<SyncReport> {
  return invoke<SyncReport>('remove_mcp_server', { name });
}

export async function syncAllMcp(): Promise<SyncReport[]> {
  return invoke<SyncReport[]>('sync_all_mcp');
}

// ── Skills ───────────────────────────────────────────────────────────────────

export async function listSkills(): Promise<SkillDto[]> {
  return invoke<SkillDto[]>('list_skills');
}

export async function syncSkill(slug: string): Promise<SyncReport> {
  return invoke<SyncReport>('sync_skill', { slug });
}

export interface ImportSkillRequest {
  tool_id: string;
  slug: string;
}

export async function importSkill(req: ImportSkillRequest): Promise<SyncReport> {
  return invoke<SyncReport>('import_skill', { toolId: req.tool_id, slug: req.slug });
}

export async function removeSkill(slug: string): Promise<SyncReport> {
  return invoke<SyncReport>('remove_skill', { slug, removeFromStore: true });
}

export async function syncAllSkills(): Promise<SyncReport[]> {
  return invoke<SyncReport[]>('sync_all_skills');
}
