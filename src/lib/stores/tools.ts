import { writable } from 'svelte/store';
import type { ToolInfo } from '$lib/ipc';
import { listTools } from '$lib/ipc';

export const tools = writable<ToolInfo[]>([]);
export const toolsLoading = writable(false);
export const toolsError = writable<string | null>(null);

export async function refreshTools() {
  toolsLoading.set(true);
  toolsError.set(null);
  try {
    const data = await listTools();
    tools.set(data);
  } catch (e) {
    toolsError.set(String(e));
  } finally {
    toolsLoading.set(false);
  }
}
