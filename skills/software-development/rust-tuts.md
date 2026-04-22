---
name: rust-tuts
category: software-development
description: "Create or update courses, lessons, and practice exercises for the Rust + Leptos tutorial platform in /Volumes/goldcoders/tuts using the frontend-only validation model."
---

# Leptos Tutorial Platform - Course Creator

Use this skill when adding or updating course content for `/Volumes/goldcoders/tuts/`.

The platform is intentionally **frontend-only**:
- Rust + Leptos 0.8 CSR + Trunk
- localStorage progress under `tuts_platform_v1`
- no backend compiler
- no Cargo execution inside lessons
- no rust-analyzer integration inside lessons

## Source of Truth

```
src/
  data/
    model.rs        # Shared lesson/course types
    courses.rs      # Active course registry
    *.rs            # Per-course lesson arrays
    mod.rs          # Re-exports
  validation/
    mod.rs          # Local validation engine
  state/
    mod.rs          # Run flow, persistence, terminal logging
```

## Authoring Contract

Every `TutorialModule` now uses `validation`, not `solution`.

Concept lessons:
- `module_type: ModuleType::Concept`
- `validation: ValidationSpec::Acknowledge`

Practice lessons:
- `module_type: ModuleType::Practice`
- `validation: ValidationSpec::Rules { ... }`
- must include:
  - `normalize`
  - `required`
  - `forbidden`
  - `canonical_solution`
  - at least 2 `hints`

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
- “any semantically correct Rust answer” grading

If an exercise is too open-ended for local validation, redesign the exercise. Do not try to turn the validator into a compiler.

## Validation Rules

Default to the simplest reliable matcher:
- `Contains` for exact fragments
- `AnyContains` when 2-3 small alternatives are acceptable
- `OrderedContains` when sequence matters
- `Regex` for syntax-sensitive signatures
- `NormalizedExact` only for narrow template-style exercises

Read [validator recipes](references/validators.md) before writing a new practice module.

## Required Samples

For every new practice, the author must prepare:
- 1 passing sample
- 1 failing sample

When practical, add or update unit tests in `src/validation/mod.rs` that cover the new lesson shape.

## Workflow

1. Create or update the course module in `src/data/`.
2. Encode concept steps with `ValidationSpec::Acknowledge`.
3. Encode practice steps with explicit rule labels, canonical solution, and two targeted hints.
4. Register new courses in `src/data/courses.rs` and `src/data/mod.rs` if needed.
5. Run:

```bash
cd /Volumes/goldcoders/tuts
cargo check
cargo test
cargo check --target wasm32-unknown-unknown
trunk build
```

## Quality Bar

- Rule labels must explain what is missing in plain English.
- Hints must point at likely mistakes, not generic “check syntax” advice.
- Canonical solutions should be short and reflect the lesson’s intended answer.
- Prefer redesigning the lesson over stacking many brittle regexes.
