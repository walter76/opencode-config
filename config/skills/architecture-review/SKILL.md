---
name: architecture-review
description: Create architecture review documents to understand and document existing codebases. Use when onboarding to an unfamiliar codebase, conducting architecture audits, or creating reference documentation for myself or team members.
---

# Architecture Review Skill

Create architecture review documents that comprehensively document an existing codebase's structure, patterns, and systems.

## When to Use This Skill

- Onboarding to an unfamiliar codebase
- Starting significant work in a new area
- Creating reference documentation for team members or myself
- Architecture audits or technical reviews
- Documenting a system before major refactoring

**Don't use for:**
- Planning new features
- Tracking work progress
- Strategic project context (use `project-overview` skill)

## File Location

Architecture reviews live in the repository root under `doc`:

```
doc/
└── architecture/
    ├── 2025-12-29-backend-architecture.md
    └── 2025-12-15-api-layer-review.md
```

**Naming:** `YYYY-MM-DD-{system-or-area}-architecture.md`

## Document Template

### Title and Metadata

```markdown
# [System/Codebase Name] - Architecture Review

**Review Date:** [Date]
**Codebase:** [Path or repo name]
**Scope:** [What parts of the codebase this covers]

---
```

### Executive Summary

One paragraph summarizing what this system does and its key architectural characteristics.

```markdown
## Executive Summary

[Brief description of the system's purpose, its main components, and notable architectural patterns. Reader should understand what this codebase does and its general shape after this paragraph.]
```

### Tech Stack

Document the languages, frameworks, and key dependencies.

```markdown
## Tech Stack

**Language(s):** [Rust, C#, JavaScript, etc.]
**Framework(s):** [.NET, React, React MUI Joy, etc.]
**Runtime:** [Node.js, Bun, Deno, etc.]

### Key Dependencies

| Package | Purpose |
|---------|---------|
| `package-name` | [What it's used for] |
| `package-name` | [What it's used for] |

### Dev Dependencies

| Package | Purpose |
|---------|---------|
| `vitest` | Testing |
| `eslint` | Linting |
```

**Include:**
- Primary language(s) and version requirements
- Core framework(s)
- Key runtime dependencies (not every package, just important ones)
- Notable dev dependencies (testing, linting, building)

### Directory Structure

Show the high-level folder organization with brief descriptions.

```markdown
## Directory Structure

```
src/
├── api/                    # API routes and handlers
│   ├── routes/             # Route definitions
│   └── middleware/         # Request middleware
├── components/             # React components
│   ├── ui/                 # Reusable UI primitives
│   └── features/           # Feature-specific components
├── hooks/                  # Custom React hooks
├── store/                  # State management
│   ├── slices/             # Redux slices
│   └── api/                # RTK Query definitions
├── types/                  # TypeScript type definitions
└── utils/                  # Utility functions
```

### Key Directories

#### `src/api/`

[2-3 sentences describing what's in this directory and how it's organized]

#### `src/store/`

[Description]
```

**Guidelines:**
- Use tree diagrams for visual clarity
- Include inline comments in the tree for quick reference
- Expand on important directories in prose below

### Core Systems

Document the major systems/subsystems and how they work.

```markdown
## Core Systems

### [System Name]

**Purpose:** [What this system does]
**Key Files:** 
- `path/to/main-file.rs`
- `path/to/related-file.rs`

[2-4 paragraphs explaining how this system works, its responsibilities, and how it fits into the larger architecture]

#### How It Works

[Detailed explanation with code snippets if helpful]

```rust
// Example showing key pattern
```

#### Key Concepts

- **[Concept 1]:** [Definition]
- **[Concept 2]:** [Definition]
```

**Repeat for each major system.** Typical systems to document:
- Data fetching / API layer
- State management
- Routing
- Authentication
- Job/task systems
- Plugin/extension systems

### Data Flow

Describe how data moves through the application.

```markdown
## Data Flow

### [Flow Name] (e.g., "User Request Flow")

```
[User Action] → [Component] → [Hook/Action] → [API] → [Backend] → [Database]
                                   ↓
                              [State Update]
                                   ↓
                              [Re-render]
```

[Prose explanation of the flow]

### [Another Flow Name]

[Diagram and explanation]
```

**Include:**
- Request/response flows
- State update flows
- Data transformation pipelines
- Event flows (if event-driven)

### Key Data Structures

Document important types, interfaces, and data shapes.

```markdown
## Key Data Structures

### [Type/Interface Name]

**Location:** `path/to/types.ts`
**Purpose:** [What this represents]

```rust
struct AppState {
    user: Option<User>,
    settings: Settings,
    // ... key fields
}
```

**Usage:** [Where and how this is used]

### [Another Type]

[Same structure]
```

**Focus on:**
- Core domain types
- State shapes
- API request/response types
- Configuration objects

### Architectural Patterns

Document recurring patterns used throughout the codebase.

```markdown
## Architectural Patterns

### 1. [Pattern Name]

**What:** [Brief description]
**Where:** [Where this pattern is used]
**Why:** [Why this pattern was chosen]

```rust
// Example implementation
```

### 2. [Pattern Name]

[Same structure]
```

**Common patterns to look for:**
- Component composition patterns
- State management patterns
- Error handling patterns
- API integration patterns
- Code organization conventions

### File Index

Quick reference to important files.

```markdown
## File Index

### Entry Points
| File | Purpose |
|------|---------|
| `src/index.js` | Application entry point |
| `src/App.jsx` | Root component |

### Configuration
| File | Purpose |
|------|---------|
| `tsconfig.json` | TypeScript configuration |
| `vite.config.ts` | Build configuration |

### Core Logic
| File | Purpose |
|------|---------|
| `src/store/index.ts` | Store configuration |
| `src/api/client.ts` | API client setup |
```

### Configuration

Document how configuration is managed.

```markdown
## Configuration

### Config Files

| File | Purpose |
|------|---------|
| `tsconfig.json` | TypeScript compiler options |
| `.env.example` | Environment variable template |
| `vite.config.ts` | Build configuration |

### Environment Variables

| Variable | Purpose | Required |
|----------|---------|----------|
| `DATABASE_URL` | Database connection | Yes |
| `API_KEY` | External API access | Yes |
| `DEBUG` | Enable debug logging | No |

### Runtime Configuration

[How is configuration loaded? dotenv? Config files? Environment-specific?]
```

### Build & Deploy

Document how to build, test, and deploy.

```markdown
## Build & Deploy

### Common Commands

| Command | Purpose |
|---------|---------|
| `pnpm install` | Install dependencies |
| `pnpm dev` | Run development server |
| `pnpm build` | Production build |
| `pnpm test` | Run tests |
| `pnpm typecheck` | Type check without emit |

### Build Process

[How does the build work? What's the output?]

### Deployment

[How is this deployed? CI/CD? Manual? What environments exist?]
```

### External Integrations

Document external services and APIs.

```markdown
## External Integrations

### [Service Name]

**Purpose:** [What it's used for]
**Integration Point:** `path/to/integration.ts`
**Docs:** [Link to service docs]

### [Database/Storage]

**Type:** [PostgreSQL, MongoDB, S3, etc.]
**Connection:** [How is it connected?]
**Schema:** [Where are schemas/migrations defined?]
```

### Observations & Recommendations

Optional section for noting issues, tech debt, or improvement opportunities.

```markdown
## Observations

### Strengths
- [What's working well]
- [Good patterns in use]

### Areas of Concern
- [Potential issues or tech debt]
- [Inconsistencies noted]

### Recommendations
- [Suggested improvements]
- [Refactoring opportunities]
```

---

## Writing Guidelines

1. **Audience:** Engineers unfamiliar with the codebase
2. **Tone:** Descriptive, neutral, educational
3. **Depth:** Cover breadth first, depth where it matters
4. **Examples:** Include code snippets to make patterns concrete
5. **Visual:** Use diagrams, trees, and tables for clarity

## Section Priority

**Always include:**
- Executive Summary
- Tech Stack
- Directory Structure
- Core Systems (at least the most important ones)
- Build & Deploy (common commands)

**Include when helpful:**
- Data Flow (for complex applications)
- Key Data Structures (for data-heavy apps)
- Architectural Patterns (when patterns are consistent)
- Configuration (when non-trivial)
- External Integrations (when external services exist)
- File Index (for larger codebases)

**Optional:**
- Observations & Recommendations (when conducting an audit)

## Common Pitfalls

- ❌ Too shallow: Include enough detail to be useful, not just file names
- ❌ Too deep: This is an overview, not line-by-line documentation
- ❌ No examples: Code snippets make patterns concrete
- ❌ Missing "why": Explain rationale behind architectural choices when known
- ❌ Outdated quickly: Focus on stable architecture, not implementation details that change frequently
- ❌ No diagrams: Visual representations help understanding