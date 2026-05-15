import { writable } from 'svelte/store';
import type { SkillDto } from '$lib/ipc';
import { listSkills } from '$lib/ipc';

export const skills = writable<SkillDto[]>([]);
export const skillsLoading = writable(false);
export const skillsError = writable<string | null>(null);

export async function refreshSkills() {
  skillsLoading.set(true);
  skillsError.set(null);
  try {
    const data = await listSkills();
    skills.set(data);
  } catch (e) {
    skillsError.set(String(e));
  } finally {
    skillsLoading.set(false);
  }
}
