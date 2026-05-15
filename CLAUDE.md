# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Development
./dev.sh          # Start Tauri dev server (handles PATH for cargo automatically)

# Build
./build.sh                 # Full release build → .app + .dmg in src-tauri/target/release/bundle/
./build.sh --no-bundle     # Compile only, no packaging → src-tauri/target/release/loadout

# If running cargo or tauri directly, ensure cargo is in PATH first:
source "$HOME/.cargo/env"
node_modules/.bin/tauri dev
node_modules/.bin/tauri build

# Type checking (frontend)
npx svelte-check --tsconfig ./tsconfig.json
```

There are no automated tests currently.

## Architecture

Loadout is a **Tauri 2 + Svelte 5 + Rust** desktop app that syncs MCP server configs and Skills across multiple AI coding tools (Claude Code, Codebuddy, Workbuddy, Codex).

### Core concept: Driver-based extensibility

Each AI tool is described by a declarative TOML driver file in `drivers/`. Adding support for a new tool requires only a new TOML file — no Rust code changes — as long as its MCP format is already implemented.

Driver files declare:
- **Detection**: which paths prove the tool is installed
- **MCP format strategy**: `json-key` (most tools), `plugin-manifest` (Codex)
- **Skill sync strategy**: `symlink` (most tools), `copy-with-manifest` (Codex)

At startup, `lib.rs` loads drivers from the bundled `drivers/` dir (resource dir in prod, `CARGO_MANIFEST_DIR/../drivers` in dev), then from `~/.loadout/drivers/` for user-defined drivers.

### Data flow

```
drivers/*.toml
    └─ driver::loader → ToolDriver structs
           └─ SyncEngine (src-tauri/src/sync/engine.rs)
                  ├─ read_all_mcp()          → merge presence across all installed tools
                  ├─ sync_mcp_to_all()       → write to every installed tool
                  ├─ sync_mcp_to_tool()      → write to one specific tool
                  ├─ remove_mcp_from_tool()  → remove from one specific tool
                  ├─ remove_mcp_from_all()   → remove from all installed tools
                  ├─ read_all_skills()       → central store + tool dirs, presence detection
                  ├─ sync_skill_to_all()     → auto-import if needed, then sync to all
                  ├─ import_skill()          → copy from tool dir into central store
                  └─ remove_skill_from_all() → remove from tools (optionally from store)
```

**MCP**: Configs are read/written directly to each tool's config file (no central MCP store). `mcp/formats/json_key.rs` handles the standard `{"mcpServers": {...}}` format; `mcp/formats/plugin_manifest.rs` handles Codex's plugin folder format.

**Skills**: Canonical copies live in `~/.loadout/store/skills/<slug>/`. Tools get either a symlink pointing there (zero-copy, instant sync) or a full copy + generated manifest (Codex). `sync/hasher.rs` uses blake3 to detect content drift.

Skill discovery is **two-phase**: `read_all_skills()` first scans the central store, then scans each installed tool's skill directory. Skills that exist only in a tool (never imported) are included in the result with presence detection.

### Backend modules

| Module | Purpose |
|--------|---------|
| `driver/` | TOML loading, path expansion, driver registry |
| `mcp/formats/` | Read/write/remove MCP configs per format strategy |
| `skills/formats/` | Symlink, copy, copy-with-manifest strategies |
| `skills/scanner.rs` | Scan a directory for `SKILL.md` subdirectories |
| `skills/parser.rs` | Parse SKILL.md frontmatter (name, description, tags) |
| `sync/engine.rs` | Orchestrates all sync operations; the main entry point for commands |
| `sync/hasher.rs` | blake3-based content hashing for drift detection |
| `store/` | `~/.loadout/` directory layout |
| `commands/` | Tauri IPC handlers (thin wrappers over `SyncEngine`) |
| `watcher/` | File watching with debouncing |

### Frontend

Svelte 5 with `$state`/`$derived` runes (not legacy stores for component state). Writable stores in `src/lib/stores/` hold server-fetched data.

**Key pattern**: Pages call `refresh*()` from stores on `onMount`, then invoke IPC directly for mutations. All IPC calls go through `src/lib/ipc/index.ts` which wraps Tauri's `invoke()`.

**`SyncMatrix`** (`src/lib/components/matrix/SyncMatrix.svelte`) is the shared presence table used on Dashboard, MCP, and Skills pages. It accepts `rows` (name + presence map), `tools`, `onRowClick`, and `onCellClick` callbacks.

**Cell click behavior** (when `onCellClick` is provided):
- Badge is `absent` / `copied` / `drifted` → sync (add) to that tool
- Badge is `present` / `symlinked` → remove from that tool (toggle off)

**`ConfirmDialog`** (`src/lib/components/shared/ConfirmDialog.svelte`) is a modal used before destructive or sync operations. It supports a "15 分钟内不再提示" checkbox that suppresses confirmations for 15 minutes (in-memory, per-page).

**Presence states** for MCP: `present` / `absent`. For skills: `symlinked` / `copied` / `drifted` / `absent`.

### Tauri commands (IPC)

| Command | Frontend function | Behavior |
|---------|-------------------|----------|
| `list_tools` | `listTools()` | List all drivers + installed status |
| `list_mcp_servers` | `listMcpServers()` | Read MCP from all tools, merged with presence |
| `add_mcp_server` | `addMcpServer(req)` | Create new server and sync to all |
| `sync_mcp_to_tool` | `syncMcpToTool(name, toolId)` | Add server to one tool |
| `remove_mcp_from_tool` | `removeMcpFromTool(name, toolId)` | Remove server from one tool |
| `remove_mcp_server` | `removeMcpServer(name)` | Remove server from all tools |
| `sync_all_mcp` | `syncAllMcp()` | Full MCP sync across all tools |
| `list_skills` | `listSkills()` | Read skills from store + tool dirs |
| `sync_skill` | `syncSkill(slug)` | Auto-import then sync to all tools |
| `import_skill` | `importSkill(req)` | Import from tool into central store |
| `remove_skill` | `removeSkill(slug)` | Remove from all tools (+ store) |
| `sync_all_skills` | `syncAllSkills()` | Full skill sync across all tools |

### Adding a new tool (zero Rust code)

Create `drivers/<toolname>.toml` following the pattern of `drivers/claude-code.toml` (for json-key MCP) or `drivers/codex.toml` (for plugin-manifest MCP with copy-with-manifest skills). The app picks it up on next launch.

### Adding a new MCP format strategy

1. Add a variant to `McpFormat` enum in `src-tauri/src/driver/mod.rs`
2. Add parsing in `driver/loader.rs` → `resolve_mcp()`
3. Implement read/write/remove in a new file under `mcp/formats/`
4. Add dispatch cases in `mcp/formats/mod.rs`

## Central store layout

```
~/.loadout/
├── store/skills/<slug>/    ← canonical skill copies
├── drivers/                ← user-defined driver overrides
├── generators/             ← Lua manifest generators (for Codex-style tools)
├── db/                     ← future search index
└── config.toml
```

## Key design decisions

- **Skills auto-import**: When syncing a skill that only exists in a tool directory (not yet in central store), the engine automatically imports it from the first tool that has it before syncing to others.
- **Presence without store**: Skills discovered only in tool dirs (never imported to `~/.loadout/store/skills/`) are shown as `copied` — not `drifted` — because there's no central source to drift from.
- **Toggle semantics**: Clicking a presence badge toggles it — present items get removed, absent items get synced. This applies to both MCP and Skills matrices.
- **Confirm dialog with suppression**: All cell-click operations on the Dashboard show a confirmation dialog. Users can check "15 分钟内不再提示" to suppress confirmations for 15 minutes.
