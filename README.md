# OpenCode AI Development Workflow Config

Reference configuration for [OpenCode](https://opencode.ai/). Inspired by the incredible blog posts from
[Mark Erikson:](https://github.com/markerikson)

- [My Thoughts on AI, Part 1: Fears, Opinions, and Mental Journey](https://blog.isquaredsoftware.com/2026/04/ai-thoughts-part-1-fears-opinions-journey/)
- [My Thoughts on AI, Part 2: Agent Setup, Workflow, and Tools](https://blog.isquaredsoftware.com/2026/04/ai-thoughts-part-2-agent-workflow-tools/)

Big cudos to Mark and it is awesome that he put a sample config [here.](https://github.com/markerikson/opencode-config-example). I am using the contents of the repo as a starting point and developing my own workflow from
there.

## Structure

### `config/`

Drop-in contents for `~/.config/opencode/`. On Windows this is `%USERPROFILE%\.config\opencode`. Key pieces:

- `AGENTS.md` - Global behavioral rules: response style, thinking protocols, git policy, coding standards, documentation
  workflow

## Start Copilot CLI

On Windows, run `src\scripts\start-copilot.ps1` from PowerShell. The launcher starts Copilot in the directory from which it
is invoked and automatically allows local file edits and read-only Git inspection (`status`, `diff`, `log`, `show`, and
`ls-files`).
Other shell commands, network access, and paths outside the repository still require Copilot's normal approval.

```powershell
.\src\scripts\start-copilot.ps1
.\src\scripts\start-copilot.ps1 --continue
```

To install the launcher in a user tools directory and add it to the user `PATH`, run:

```powershell
.\scripts\install-start-copilot.ps1
```

Open a new PowerShell session afterward, then run `start-copilot.ps1` from any directory.

## License

MIT
