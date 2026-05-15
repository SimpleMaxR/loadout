import { writable } from 'svelte/store';
import type { McpServerDto } from '$lib/ipc';
import { listMcpServers } from '$lib/ipc';

export const mcpServers = writable<McpServerDto[]>([]);
export const mcpLoading = writable(false);
export const mcpError = writable<string | null>(null);

export async function refreshMcp() {
  mcpLoading.set(true);
  mcpError.set(null);
  try {
    const data = await listMcpServers();
    mcpServers.set(data);
  } catch (e) {
    mcpError.set(String(e));
  } finally {
    mcpLoading.set(false);
  }
}
