---
name: feature-analysis
description: Analyze and document the features of a codebase. Use when onboarding to an unfamiliar codebase, conducting feature audits, or creating reference documentation for myself or team members.
---

# Feature Analysis Skill

Creates a comprehensive feature overview of a codebase, creating a detailed list of features, their summaries, and descriptions. Looks for feature gaps and documents them at the end.

## When to Use This Skill

- Onboarding to an unfamiliar codebase
- Starting significant work in a new area
- Creating reference documentation for team members or myself

## File Location

Feature analysis documents live in the repository root under `doc`:

```
doc/
└── feature-analysis/
    ├── 2025-12-29-backend-feature-analysis.md
    └── 2025-12-15-api-layer-review.md
```

**Naming:** `YYYY-MM-DD-{system-or-area}-feature-analysis.md`

## Document Template

### Title and Metadata

```markdown
# [System/Codebase Name] - Feature Analysis

**Review Date:** [Date]
**Codebase:** [Path or repo name]
**Scope:** [What parts of the codebase this covers]

---
```

### Purpose

One paragraph summarizing what this system does and its key features.

```markdown
## Purpose

[Brief description of the system's purpose, its main components, and notable features. Reader should understand what this system does after this paragraph.]
```

### Feature List

For each feature, provide a summary and description. Include any relevant details about how the feature works, its dependencies, and any known issues or limitations.

```markdown
## Feature List

### F-01: [Feature Name]

**Summary:** [One sentence summary of the feature]

**Description:** [Detailed description of the feature, including how it works, its dependencies, and any known issues or limitations.]

---

### F-02: [Feature Name]

**Summary:** [One sentence summary of the feature]

**Description:** [Detailed description of the feature, including how it works, its dependencies, and any known issues or limitations.]

---

...
```

### Feature Gaps

[Identify any missing features, limitations, or areas for improvement in the system. This section helps highlight potential enhancements or areas that require further development.]

```markdown
## Feature Gaps

### G-01: [Feature Gap Name]

**Description:** [Detailed description of the feature gap, including why it is a gap, its impact on the system, and any potential solutions or recommendations.]

---

### G-02: [Feature Gap Name]

**Description:** [Detailed description of the feature gap, including why it is a gap, its impact on the system, and any potential solutions or recommendations.]

---

...
```
