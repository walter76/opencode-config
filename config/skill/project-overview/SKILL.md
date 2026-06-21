---
name: project-overview
description: Create comprehensive project overview documents for major features or projects. Use when starting significant work that needs context documentation for team members or future reference. Provides strategic context, goals, and architectural decisions.
---

# Project Overview Skill

Create comprehensive project overview documents that provide strategic context and understanding for major projects or features.

## When to Use This Skill

- Starting a major project (multi-week effort)
- Significant architectural changes
- Cross-team initiatives needing shared context
- Prototypes that will inform production work
- Dogfooding/validation projects

**Don't use for:**
- Small bug fixes
- Single-feature additions
- Routine maintenance work

## Document Template

### Title and Metadata

```markdown
# [Project Name]: Project Overview

**Project Name:** [Short identifier]
**Status:** [Planning / In Progress / Complete]
**Timeline:** [Estimated duration]
**Purpose:** [One-sentence summary]

---
```

### Section 1: Understanding the Context

Provide background for someone unfamiliar with the project. Answer:
- What is the system/codebase you're working in?
- What prior work led to this? (prototypes, experiments, decisions)
- What recent progress or decisions have been made?

**Length:** 3-5 paragraphs

**Example:**
```markdown
## Understanding the Context

### What is [System Name]?
[Brief description of the system, its purpose, what it generates/does]

### The [Prototype/Reference Project]
[Describe any reference implementation or prototype that validated the approach]

### Recent Progress
[Key decisions or progress that led to this work]
```

### Section 2: What is This Project?

Define the project scope clearly.

```markdown
## What is This Project?

### Current State
[Describe what exists today and why it's insufficient]

### Target State
[Describe the desired end state]

### Important Scope Decisions
**In Scope:**
- [What you ARE doing]

**Out of Scope:**
- [What you are NOT doing - be explicit]
```

**Critical:** Use blockquotes or callouts for major scope constraints:
```markdown
> **⚠️ SCOPE:** This focuses exclusively on [X]. We are NOT:
> - [Thing 1]
> - [Thing 2]
```

### Section 3: Why This Work Matters

Explain the value. Don't assume it's obvious.

**Categories:**
- **Architecture Improvement:** Technical debt reduction, modernization
- **Developer Experience:** Productivity, maintainability
- **Foundation for Future:** Enabling future work
- **Validation:** Proving patterns before wider adoption

**Length:** 2-4 paragraphs or bullet sections

### Section 4: Big Picture Goals

List 3-5 major goals. Be specific and measurable when possible.

```markdown
## Big Picture Goals

### 1. [Goal Name]
[1-2 sentences describing what success looks like]

### 2. [Goal Name]
[1-2 sentences]

...
```

### Section 5: Major Phases Overview

High-level phase breakdown. Keep it brief - detailed tasks belong in the development plan.

```markdown
## Major Phases Overview

### Phase 1: [Name]
**Goal:** [One sentence]
**Key Activities:** [3-5 bullet points of major work items]

### Phase 2: [Name]
...
```

**Length:** 1 paragraph per phase (2-3 sentences + bullets)

### Section 6: Key Architectural Decisions

Document important technical decisions with rationale.

```markdown
## Key Architectural Decisions

### 1. [Decision Name]
**Decision:** [What was chosen]
**Rationale:** [Why this choice over alternatives]
**Impact:** [What this means for implementation]

### 2. [Decision Name]
...
```

**Include:**
- Technology choices (libraries, patterns, tools)
- Data flow decisions
- Integration approaches
- Performance trade-offs

### Section 7: Key Constraints and Principles

Be explicit about what to do and what to avoid.

```markdown
## Key Constraints and Principles

### Do's
- [Guideline 1]
- [Guideline 2]

### Don'ts
- [Anti-pattern 1]
- [Anti-pattern 2]

### Commands
If relevant, document correct vs incorrect commands:

**Correct:**
- `pnpm typecheck`

**NEVER use:**
- `tsc --noEmit` (won't work with split config)
```

### Section 8: Success Metrics

Define what "done" looks like.

```markdown
## Success Metrics

### Functional Requirements
- [ ] [Requirement 1]
- [ ] [Requirement 2]

### Quality Indicators
- Code coverage: [target]
- Type safety: [target]
- Performance: [target]

### Validation Metrics
- [How you'll know it works]
```

### Section 9: Related Documentation

Link to other relevant docs and code.

```markdown
## Related Documentation

### This Project
- [Development Plan](./development-plan.md)
- [Architecture Doc](./architecture.md)

### Reference Implementation
- [Prototype repo/folder]

### Key Code Locations
- Backend: `path/to/code`
- Frontend: `path/to/code`
```

### Section 10: Risk Assessment

Identify risks by impact level.

```markdown
## Risk Assessment

### High Impact Risks
- **[Risk Name]:** [Description] | **Mitigation:** [Strategy]

### Medium Impact Risks
- **[Risk Name]:** [Description] | **Mitigation:** [Strategy]

### Hardest Parts
- [Challenge 1]
- [Challenge 2]
```

### Section 11: Conclusion

One paragraph wrap-up. Restate the core goal and why it matters.

---

## Writing Guidelines

1. **Audience:** Write for someone joining the project mid-stream or reviewing in 6 months
2. **Tone:** Direct, technical, no fluff
3. **Length:** 400-800 lines typical
4. **Living Document:** Update as decisions change, but keep historical context
5. **Link Heavily:** Reference other docs, code locations, prototypes
6. **Scope Clarity:** Be explicit about what's NOT being done
7. **Rationale:** Always explain "why", not just "what"

## Common Pitfalls

- ❌ Assuming context is obvious (it's not - spell it out)
- ❌ Vague goals ("improve architecture" → "migrate to X pattern for Y benefit")
- ❌ Skipping the "why" (future-you won't remember)
- ❌ Listing tasks here (those belong in development plan)
- ❌ No success criteria (how will you know you're done?)
