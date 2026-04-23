---
name: rust-tuts
category: software-development
description: "Create or update focus courses (atomic, bite-sized lesson modules) for the Rust + Leptos tutorial platform in /Volumes/goldcoders/tuts using the frontend-only validation model."
---

# Leptos Tutorial Platform — Focus Course Creator

Use this skill when **adding or updating atomic focus courses** for `/Volumes/goldcoders/tuts/`.

> **Important:** Focus courses are the *atoms* of the lesson system. They are self-contained, 2-6 lesson modules on a single topic. They live in `src/data/focus_*.rs`. Curriculum paths (guided learning journeys) are composed from these atoms — see the **rust-curriculum** skill.

## Architecture

The platform uses a **two-tier lesson architecture**:

```
┌─────────────────────────────────────────────────────┐
│  Lesson Pool (src/data/lesson_pool.rs)              │
│  Re-exports all focus_*.rs MODULES arrays           │
├─────────────────────────────────────────────────────┤
│  Focus Courses (src/data/focus_*.rs)                │
│  Atomic 2-6 lesson modules on a single topic        │
│  ─ This is what this skill creates ─                │
├─────────────────────────────────────────────────────┤
│  Curriculum Paths (src/data/curriculum_*.rs)         │
│  Composed from focus modules via module_groups       │
│  ─ See rust-curriculum skill ─                      │
├─────────────────────────────────────────────────────┤
│  Standalone Courses (src/data/rust_*.rs)             │
│  Legacy inline courses (pre-migration)              │
└─────────────────────────────────────────────────────┘
```

The platform is intentionally **frontend-only**:
- Rust + Leptos 0.8 CSR + Trunk
- localStorage progress under `tuts_platform_v1`
- no backend compiler, no Cargo execution, no rust-analyzer inside lessons

## Source of Truth

```
src/
  data/
    model.rs            # Course, TutorialModule, CourseKind, ModuleGroup
    courses.rs          # Active course registry (all courses)
    lesson_pool.rs      # Central re-exports of all focus module arrays
    focus_*.rs          # Atomic focus courses (THIS SKILL)
    curriculum_*.rs     # Composed learning paths (rust-curriculum skill)
    rust_*.rs           # Legacy standalone courses
    mod.rs              # Module declarations + re-exports
  validation/
    mod.rs              # Local validation engine
  state/
    mod.rs              # Run flow, persistence, terminal logging
```

## Authoring Contract

Every `TutorialModule` uses `validation`, not `solution`.

**Concept lessons:**
- `module_type: ModuleType::Concept`
- `validation: ValidationSpec::Acknowledge`

**Practice lessons:**
- `module_type: ModuleType::Practice`
- `validation: ValidationSpec::Rules { ... }`
- must include: `normalize`, `required`, `forbidden`, `canonical_solution`, at least 2 `hints`

## Allowed Exercise Shapes

Prefer narrow, structurally checkable tasks:
- define a trait, function, bound, or type alias
- add one associated type or constant
- convert a signature from one form to another
- fill in a specific trait bound or return type
- complete a short impl header
- edit a fixed starter template in a small, predictable way

Do not author exercises that require:
- real compilation
- dependency resolution
- macro expansion correctness
- borrow-check or lifetime soundness beyond narrow syntax checks
- "any semantically correct Rust answer" grading

If an exercise is too open-ended for local validation, redesign the exercise. Do not try to turn the validator into a compiler.

## Validation Rules

Default to the simplest reliable matcher:
- `Contains` for exact fragments
- `AnyContains` when 2-3 small alternatives are acceptable
- `OrderedContains` when sequence matters
- `Regex` for syntax-sensitive signatures (use raw strings: `r#""#`)
- `NormalizedExact` only for narrow template-style exercises

Read [validator recipes](references/validators.md) before writing a new practice module.

## Required Samples

For every new practice, the author must prepare:
- 1 passing sample
- 1 failing sample

When practical, add or update unit tests in `src/validation/mod.rs` that cover the new lesson shape.

## Workflow

1. Create `src/data/focus_<topic>.rs` with a `pub const MODULES: &[TutorialModule] = &[...];`
2. Encode concept steps with `ValidationSpec::Acknowledge`.
3. Encode practice steps with explicit rule labels, canonical solution, and two targeted hints.
4. Register in `src/data/mod.rs`:
   - Add `mod focus_<topic>;`
   - Add `pub use focus_<topic>::MODULES as FOCUS_<TOPIC>_MODULES;`
5. Register in `src/data/lesson_pool.rs`:
   - Add `pub use crate::data::focus_<topic>::MODULES as <TOPIC>;`
6. Register in `src/data/courses.rs`:
   - Add `use` import
   - Add `Course { ... kind: CourseKind::Focus, module_groups: &[], tags: &["deep-dive"], ... }`
7. Run:

```bash
cd /Volumes/goldcoders/tuts
cargo check
cargo test
```

## Quality Bar

- Rule labels must explain what is missing in plain English.
- Hints must point at likely mistakes, not generic "check syntax" advice.
- Canonical solutions should be short and reflect the lesson's intended answer.
- Prefer redesigning the lesson over stacking many brittle regexes.
- **Never duplicate content** — if a lesson already exists in another focus module, reference it via a curriculum instead.
