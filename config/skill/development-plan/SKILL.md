---
name: development-plan
description: Create detailed development plans with task lists and progress tracking for major projects. Use for multi-phase work requiring detailed task breakdown, progress tracking, and living documentation of decisions and learnings.
---

# Development Plan Skill

Create detailed, actionable development plans that serve as living documents for tracking multi-phase project execution.

## When to Use This Skill

- Multi-week projects with multiple phases
- Work requiring detailed task breakdown
- Projects needing progress tracking over time
- Complex migrations or refactors
- When you need to track "what we learned" alongside tasks

**Don't use for:**

- Single-session work
- Small features (< 3 days)
- Exploratory work without clear tasks yet

## Document Template

### Title and Metadata

```markdown
# [Project Name] Development Plan & Progress Tracker

**Document Purpose:** This is a living document that tracks the overall development plan, task lists, and progress for [brief description].

**Last Update:** YYYY-MM-DD

---
```

### Section 1: Document Purpose

Explicitly state what this document tracks.

```markdown
## Document Purpose

This document serves as the central hub for:

- **Overall Development Plan:** Breaking [work] into logical phases
- **Task Lists:** Detailed, actionable tasks for each phase
- **Progress Tracking:** What's done, what's in progress, what's remaining
- **Notes & Learnings:** Capturing insights, decisions, and discoveries
- **Open Questions:** Items requiring further investigation

This is a **living document** that will be updated frequently as we
- Complete tasks and mark them done
- Discover new work that needs to be added
- Learn patterns and make design decisions
- Encounter and solve challenges
```

### Section 2: Key Constraints (Optional but Recommended)

Critical scope limitations that need constant visibility.

```markdown
## Key Constraints

> **⚠️ SCOPE:** [Critical constraint that affects all work]
>
> - NOT doing [thing]
> - NOT supporting [thing]
> - This is [approach/limitation]
```

### Section 3: Phase Overview

High-level summary of all phases (1-2 sentences each).

```markdown
## Phase Overview

The [work] is organized into [N] major phases:

### Phase 1: [Name]
**Goal:** [One sentence goal]
**Key Activities:** [3-5 bullets of major work]

### Phase 2: [Name]
...
```

**Purpose:** Quick reference to see full scope without scrolling through details.

### Section 4+: Detailed Phase Sections

One section per phase with full task breakdown.

```markdown
## Phase [N]: [Phase Name]

**Goal:** [Restate the phase goal in detail]

### Tasks

#### [Task Category 1]

- [ ] Task description
  - Sub-detail if needed
  - Notes about approach
- [x] Completed task ✅ COMPLETED (YYYY-MM-DD)
  - Notes about what was done
  - Learnings or gotchas
- [ ] Task with sub-tasks:
  - [ ] Sub-task 1
  - [x] Sub-task 2 ✅
  - [ ] Sub-task 3

#### [Task Category 2]

...

### Success Criteria

What "done" looks like for this phase:

- [ ] [Criterion 1]
- [ ] [Criterion 2]
- [ ] [Criterion 3]

### Notes

*This section captures learnings, decisions, and discoveries as work progresses.*

- **[Date or Topic]:** [Note]
- **Decision:** [What was decided and why]
- **Discovery:** [Thing we learned that affects approach]
```

---

## Task List Best Practices

### Checkbox Format

```markdown
- [ ] Pending task
- [x] Completed task
- [x] Completed with date ✅ COMPLETED (2025-12-15)
- [x] Completed with notes ✅ (Mostly Done - [detail about partial completion])
```

**Use:**
- `- [ ]` for pending
- `- [x]` for completed
- Add ✅ emoji for visual scanning
- Add dates when task completed (helps track velocity)
- Add notes inline for context

### Task Granularity

**Good task size:** Completable in 30 minutes - 2 hours

**Too big:**
- ❌ "Migrate all components"

**Right size:**
- ✅ "Migrate DataTable component"
- ✅ "Update FormDialog to use queryResult props"

**Break large tasks into categories:**

```markdown
#### Block Migration (by Category)

- [ ] Migrate Data Display blocks (DataTable, DataSummary, ListView)
- [ ] Migrate Form blocks (FormDialog, InlineEdit)
- [ ] Migrate Selection blocks (InlineSelect, MultiSelect)
```

### Task Organization

Group tasks by logical categories, not just flat lists:

```markdown
#### Infrastructure Setup
- [x] Add utility files
- [x] Create placeholder files
- [x] Update config files

#### Component Migration
- [ ] User components
- [ ] Project components
- [ ] Issue components
```

**Why:** Makes it easy to see progress in each area, easier to find tasks.

### Progress Tracking

Add inline notes to completed tasks:

```markdown
- [x] Run schema generation ✅ (2025-12-10)
  - Generated 18 table schemas
  - Found 3 duplicate fields (documented in notes)
  - Performance: 85ms total
```

**Captures:** Not just "it's done", but what happened and what we learned.

---

## Phase Structure Guidelines

### Tasks Section

**Format:**
- Use `####` (h4) for task categories
- Checkboxes for all actionable items
- Sub-bullets for details, context, or approach notes
- Group related tasks together

### Success Criteria Section

**Purpose:** Define "done" for the phase (not for each task).

**Format:**
```markdown
### Success Criteria

- [ ] All blocks migrated and versioned to v2.0.0
- [ ] No `useDatabase*` hooks remain in codebase
- [ ] All blocks tested with generated RTKQ hooks
- [ ] Migration patterns documented
```

**Review:** Check these when you think the phase is complete.

### Notes Section

**Purpose:** Living capture of learnings and decisions.

**What goes here:**
- Architectural decisions made during implementation
- Patterns discovered that work well (or don't)
- Gotchas encountered and how you solved them
- Open questions or items for future investigation
- Performance observations

**Format:**
```markdown
### Notes

- **2025-12-10:** Discovered that queryResult.data can be undefined during loading. Need null checks in all formatters.
- **Pattern:** Callback mutations work better than inline when multiple blocks need same mutation.
- **Decision:** Using v2.0.0 for all migrated blocks to clearly distinguish from legacy.
- **TODO:** Investigate caching behavior with array filter params - seeing extra refetches.
```

**Update:** Add notes as you work, don't wait until end of phase.

---

## Document Maintenance

### When to Update

**Daily/per-session:**
- Mark tasks complete as you finish them
- Add notes about discoveries or decisions
- Add new tasks if scope expands

**End of phase:**
- Check success criteria
- Write summary in notes section
- Review if any tasks can be moved to future phases

**Don't:**
- Delete completed tasks (they're the history)
- Remove old notes (they provide context)

### Completed Task Format

```markdown
- [x] Task description ✅ COMPLETED (2025-12-15)
  - Outcome notes
  - Gotchas or learnings
```

**Why dates matter:** Helps you estimate future phases and see velocity.

---

## Writing Guidelines

1. **Audience:** Yourself during daily work, future-you reviewing progress
2. **Tone:** Action-oriented, technical, detailed
3. **Length:** 1200-2000 lines typical (grows over time)
4. **Living Document:** Update frequently, don't wait
5. **Task Granularity:** 30 minutes to 2 hour chunks
6. **Notes Matter:** Capture learnings inline, not separately
7. **Success Criteria:** Per-phase, not per-task

## Common Pitfalls

- ❌ Tasks too big ("Migrate everything")
- ❌ No task categories (flat 100-item list)
- ❌ Deleting completed tasks (loses history)
- ❌ Not dating completions (can't track velocity)
- ❌ Not capturing learnings in Notes (lose context)
- ❌ Waiting to update until phase is done (doc gets stale)
- ❌ No success criteria (don't know when phase is really done)

## Integration with Other Docs

**Project Overview:** High-level WHY and WHAT
**Development Plan:** Detailed HOW and task-by-task execution

**Reference pattern:**
```markdown
See [Project Overview](./project-overview.md) for context and architectural decisions.
```

Both docs should link to each other.
