# Session Summary — devplans.ts Code Exploration

## Topics Covered

### Architectural Overview (`devplans.ts`)

- Bun-native CLI tool (`#!/usr/bin/env bun`) for managing dev-plan artifacts.
- All commands emit JSON to stdout; errors go to stderr with `process.exit(1)`.
- Config loaded from `~/.config/opencode/scripts/config.json` on every invocation — provides `devPlansBase` (Windows/WSL paths) and `projectMapping` (cwd key → folder name).
- Project is resolved by matching `cwd` (lowercased) against `projectMapping` keys.
- External dependency: `./lib/opencode-db.ts` for reading OpenCode's SQLite DB (`bun:sqlite`).

### Command Tree

```
devplans
  info
  progress              → get/create today's file, write scaffold temp JSON
  progress list [n]
  progress append [file|stdin]
  handoff create/list/move/consume
  doc create/list
  session extract       → spawns extract-session-text.ts via spawnSync
  archive <type> [--all] [--keep-days n] [--dry-run]
```

### `cmdProgressAppend` — How It Works

1. Reads `ProgressAppendInput` JSON from a temp file (deleted after read) or stdin.
2. Required fields: `slug` (entry title), `content` (markdown body). Optional: `nextSteps[]`, `completed[]`.
3. Entry timestamp (`HH:MM`) = **wall-clock time at invocation** via `new Date()` → `formatTime()` → `toTimeString().slice(0, 5)`. Independent of the late-night date shift.
4. Late-night shift (`getProgressDate()`): if hour < 6, uses yesterday's date for the **filename** only.
5. New entry block: `---\n\n## HH:MM — <slug>\n\n<content>\n`
6. Inserted **above** `## Next Steps` if present, otherwise at EOF.
7. If `nextSteps` is provided, replaces the entire section; if omitted, existing section is preserved.
8. `rebuildSummary()` regenerates `## Summary` by scanning all `## HH:MM` headings — replaces content between `## Summary` and the first `---` separator.

### When Is a New File Created?

A new `YYYY-MM-DD-progress.md` is created when the dated file does not exist:

- **`cmdProgress`** — creates a bare scaffold (title + date heading + empty Summary) and also writes a `.tmp-entry-<ts>.json` scaffold for the agent to fill in.
- **`cmdProgressAppend`** — creates the file inline if it doesn't exist yet, then immediately appends the entry. Output includes `"status": "new"`.

### Progress File Format (after multiple appends)

```markdown
# My App Progress Update

## 2026-06-23

## Summary

- 09:15 — 4.5.1 Auth Setup
- 11:42 — 4.5.2 Database Schema

---

## 09:15 — 4.5.1 Auth Setup

Entry content...

---

## 11:42 — 4.5.2 Database Schema

Entry content...

## Next Steps

- Write unit tests
- Deploy to staging
```
