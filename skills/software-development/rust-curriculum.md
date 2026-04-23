---
name: rust-curriculum
category: software-development
description: "Compose guided learning paths (curriculums) from existing focus-module lesson pools for the Rust tutorial platform. Never creates inline lessons — only references existing focus modules."
---

# Leptos Tutorial Platform — Curriculum Composer

Use this skill when **composing guided learning paths** from existing focus modules for `/Volumes/goldcoders/tuts/`.

> **Critical Rule:** Curriculum files **never** contain inline `TutorialModule` definitions. They **only** reference existing focus module arrays from the lesson pool. If a lesson doesn't exist, create the focus module first using the **rust-tuts** skill.

## Architecture

```
Focus Modules (atoms)          Lesson Pool (index)           Curriculums (paths)
┌─────────────────────┐        ┌──────────────────┐        ┌──────────────────────┐
│ focus_trait_objects  │───────►│ TRAIT_OBJECTS     │───────►│ curriculum_trait_     │
│ focus_newtype        │───────►│ NEWTYPE           │───────►│   mastery.rs         │
│ focus_builder        │───────►│ BUILDER           │───────►│                      │
│ focus_typestate      │───────►│ TYPESTATE         │───────►│ curriculum_design_   │
│ focus_sealed_traits  │───────►│ SEALED_TRAITS     │───────►│   patterns.rs        │
│ ...                  │        │ ...               │        │ ...                  │
└─────────────────────┘        └──────────────────┘        └──────────────────────┘
                                lesson_pool.rs               (this skill creates these)
```

## The `ModuleGroup` Contract

A curriculum is a `&'static [ModuleGroup]` where each group references a focus module's `MODULES` array:

```rust
use crate::data::model::ModuleGroup;
use crate::data::lesson_pool;

pub const MODULE_GROUPS: &[ModuleGroup] = &[
    ModuleGroup {
        label: "Trait Objects & Dynamic Dispatch",  // Section label in UI
        modules: lesson_pool::TRAIT_OBJECTS,         // Reference, not copy
    },
    ModuleGroup {
        label: "The Newtype Pattern",
        modules: lesson_pool::NEWTYPE,
    },
    // ... more groups
];
```

Each `ModuleGroup` has:
- `label: &'static str` — displayed as a section divider in the course detail UI
- `modules: &'static [TutorialModule]` — a reference to a focus module's lesson array

## File Naming Convention

```
src/data/curriculum_<name>.rs
```

Examples:
- `curriculum_trait_mastery.rs`
- `curriculum_ownership_advanced.rs`
- `curriculum_design_patterns.rs`

## Workflow

### 1. Plan the Curriculum

List which focus modules to include and in what order. Consider prerequisite flow:

```
1. TRAIT_OBJECTS       (5 lessons) — start with basics
2. OBJECT_SAFETY       (3 lessons) — builds on #1
3. NEWTYPE             (4 lessons) — needs trait context
4. BLANKET_IMPLS       (3 lessons) — advanced trait usage
5. HRTB               (3 lessons) — capstone topic
```

### 2. Check the Lesson Pool

Open `src/data/lesson_pool.rs` and verify all required focus modules are exported. If a topic is missing:
- **Stop and use the `rust-tuts` skill** to create the focus module first.
- Then add its export to `lesson_pool.rs`.
- Then return here to compose the curriculum.

### 3. Create the Curriculum File

```rust
// src/data/curriculum_<name>.rs
use crate::data::model::ModuleGroup;
use crate::data::lesson_pool;

pub const MODULE_GROUPS: &[ModuleGroup] = &[
    ModuleGroup { label: "...", modules: lesson_pool::SOME_TOPIC },
    // ...
];
```

### 4. Register in `mod.rs`

```rust
mod curriculum_<name>;
```

### 5. Register in `courses.rs`

```rust
Course {
    id: "curriculum-<name>",
    title: "\u{1F4D6} <Title>",
    subtitle: "<description>",
    icon: "<emoji>",
    accent: "<color>",
    modules: &[],                                             // Empty — uses module_groups
    module_groups: crate::data::curriculum_<name>::MODULE_GROUPS,
    difficulty: crate::data::model::Difficulty::Intermediate,
    estimated_time: "<time>",
    kind: CourseKind::Curriculum,
    tags: &["guided-path", "<topic>"],
},
```

Key fields:
- `modules: &[]` — always empty for curriculums
- `module_groups:` — references the curriculum's MODULE_GROUPS constant
- `kind: CourseKind::Curriculum` — enables "Guided Path" badge in UI
- `tags:` — always include `"guided-path"` for filtering

### 6. Verify

```bash
cd /Volumes/goldcoders/tuts
cargo check
```

## Design Guidelines

1. **Prerequisite ordering**: Place foundational topics before advanced ones. Example: Trait Objects before Object Safety.
2. **Reasonable length**: 15-40 total lessons per curriculum. Too short? Users should just take the individual focus courses. Too long? Split into multiple paths.
3. **No duplication**: A focus module can appear in **multiple** curriculums (e.g., `NEWTYPE` in both "Trait Mastery" and "Design Patterns"). That's the whole point — shared atoms, composed paths.
4. **Section labels**: Each `ModuleGroup.label` becomes a visual section divider. Make them descriptive but concise.
5. **Tag conventions**: Use `"guided-path"` plus topic tags like `"traits"`, `"ownership"`, `"patterns"`, `"async"`.

## Quality Bar

- Every curriculum must be a coherent learning path, not a random bag of topics.
- The ordering must make sense: no topic should reference concepts not yet covered.
- Estimated time should be the sum of the individual focus module times.
- The subtitle should describe what the learner gains from completing the full path.
