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

## License

MIT
