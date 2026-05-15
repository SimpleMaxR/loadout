import { writable, derived } from 'svelte/store';

export type ThemeMode = 'dark' | 'light' | 'system';

// Tell the native macOS title bar which theme to use
async function syncNativeTheme(dark: boolean) {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('set_window_theme', { dark });
  } catch {
    // Not in Tauri context (e.g. browser dev), ignore
  }
}

function createThemeStore() {
  const stored = (typeof localStorage !== 'undefined' && localStorage.getItem('theme') as ThemeMode) || 'system';
  const { subscribe, set } = writable<ThemeMode>(stored);

  return {
    subscribe,
    set(mode: ThemeMode) {
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('theme', mode);
      }
      set(mode);
    },
  };
}

export const themeMode = createThemeStore();

// Resolved: 'dark' | 'light' — after applying system preference
export const resolvedTheme = derived(themeMode, ($mode, setResolved) => {
  if ($mode !== 'system') {
    setResolved($mode);
    // Sync native title bar immediately
    syncNativeTheme($mode === 'dark');
    return;
  }
  // system: follow matchMedia
  const mq = typeof window !== 'undefined' ? window.matchMedia('(prefers-color-scheme: dark)') : null;
  const update = () => {
    const dark = mq?.matches ?? false;
    setResolved(dark ? 'dark' : 'light');
    syncNativeTheme(dark);
  };
  update();
  mq?.addEventListener('change', update);
  return () => mq?.removeEventListener('change', update);
});
