use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "vars-1-concept",
        title: "1. Concept: Immutable by Default",
        module_type: ModuleType::Concept,
        content: r#"
# Variables Are Stable by Default

In Rust, `let` creates an **immutable binding** unless you opt into change with `mut`.

### Key Ideas
- `let score = 10;` cannot be reassigned later
- `let mut score = 10;` can be changed
- This default keeps accidental state changes out of your code

**Mental Hook**: Rust assumes values should stay put until you explicitly allow movement.
        "#,
        initial_code: "// Read the lesson on the left.\n// Click ACKNOWLEDGE to continue.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "vars-2-practice",
        title: "2. Practice: Mutable Counter",
        module_type: ModuleType::Practice,
        content: r#"
# Make a Binding Mutable

### Task:
1. Create a mutable variable named `counter`
2. Initialize it to `0`
3. Reassign it to `1`
        "#,
        initial_code: "// Create a mutable variable named counter\n// then reassign it to 1\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "declare `counter` with `let mut`",
                    matcher: RuleMatcher::Regex(r"let\s+mut\s+counter\s*=\s*0\s*;"),
                },
                ValidationRule {
                    label: "reassign `counter` to 1",
                    matcher: RuleMatcher::Contains("counter = 1;"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let mut counter = 0;\ncounter = 1;"),
            hints: &[
                "Use `let mut` in the declaration, not just `let`.",
                "The reassignment should happen on a second line after the initial binding.",
            ],
        },
        success_message: "Correct. The binding is now mutable and can be updated.",
    },
    TutorialModule {
        id: "vars-3-concept",
        title: "3. Concept: Shadowing",
        module_type: ModuleType::Concept,
        content: r#"
# Shadowing Rebinds a Name

Rust lets you reuse a variable name with a fresh `let`.

### Why It Matters
- You can transform a value step by step
- You can even change the type
- It is different from mutation because it creates a new binding

**Mental Hook**: Shadowing keeps the same label but swaps in a new value behind it.
        "#,
        initial_code: "// Study shadowing, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "vars-4-practice",
        title: "4. Practice: Shadow `spaces`",
        module_type: ModuleType::Practice,
        content: r#"
# Rebind a Name

### Task:
1. Create `spaces` bound to `"   "`
2. Shadow it so the new `spaces` is `spaces.len()`
        "#,
        initial_code: "// Bind `spaces` to three spaces\n// then shadow it with spaces.len()\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "bind `spaces` to a string of spaces",
                    matcher: RuleMatcher::Regex(r#"let\s+spaces\s*=\s*" +"\s*;"#),
                },
                ValidationRule {
                    label: "shadow `spaces` with `spaces.len()`",
                    matcher: RuleMatcher::Contains("let spaces = spaces.len();"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let spaces = \"   \";\nlet spaces = spaces.len();"),
            hints: &[
                "Use `let` twice; this lesson is about shadowing, not `mut`.",
                "The second line should reuse the same variable name `spaces`.",
            ],
        },
        success_message: "Nice. You re-bound the same name without mutating the original binding.",
    },
    TutorialModule {
        id: "vars-5-concept",
        title: "5. Concept: Constants",
        module_type: ModuleType::Concept,
        content: r#"
# Constants Never Change

Use `const` for values that are fixed for the entire program.

### Constant Rules
- They must include a type annotation
- Their names are usually uppercase with underscores
- They cannot use `mut`

**Mental Hook**: `const` is for values you want nailed to the wall.
        "#,
        initial_code: "// Constants are fixed for the lifetime of the program.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "vars-6-practice",
        title: "6. Practice: Add a Constant",
        module_type: ModuleType::Practice,
        content: r#"
# Define a Constant

### Task:
Create a constant named `MAX_POINTS` with type `u32` and value `100`.
        "#,
        initial_code: "// Write the MAX_POINTS constant here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[ValidationRule {
                label: "declare `const MAX_POINTS: u32 = 100;`",
                matcher: RuleMatcher::Regex(r"const\s+MAX_POINTS\s*:\s*u32\s*=\s*100\s*;"),
            }],
            forbidden: &[ValidationRule {
                label: "do not use `let` for a constant",
                matcher: RuleMatcher::Contains("let MAX_POINTS"),
            }],
            canonical_solution: Some("const MAX_POINTS: u32 = 100;"),
            hints: &[
                "Constants use the `const` keyword and always include a type annotation.",
                "Keep the name uppercase with an underscore: `MAX_POINTS`.",
            ],
        },
        success_message: "Exactly. This is now a compile-time constant binding.",
    },
    TutorialModule {
        id: "vars-7-concept",
        title: "7. Concept: Type Annotations",
        module_type: ModuleType::Concept,
        content: r#"
# Type Annotations Make Intent Explicit

Rust often infers types, but you can write them out when you want clarity.

### Common Pattern
```rust
let retries: u8 = 3;
```

**Mental Hook**: Type annotations are labels you pin directly onto the binding.
        "#,
        initial_code: "// Read the concept and ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "vars-8-practice",
        title: "8. Practice: Annotate a Variable",
        module_type: ModuleType::Practice,
        content: r#"
# Add an Explicit Type

### Task:
Create a variable named `retries` with:
- type `u8`
- value `3`
        "#,
        initial_code: "// Create retries with an explicit u8 type\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[ValidationRule {
                label: "write `let retries: u8 = 3;`",
                matcher: RuleMatcher::Regex(r"let\s+retries\s*:\s*u8\s*=\s*3\s*;"),
            }],
            forbidden: &[],
            canonical_solution: Some("let retries: u8 = 3;"),
            hints: &[
                "Place the type annotation after the variable name: `retries: u8`.",
                "This lesson wants a plain immutable binding, so no `mut` is needed.",
            ],
        },
        success_message: "Correct. The binding now carries an explicit numeric type.",
    },
    TutorialModule {
        id: "vars-9-concept",
        title: "9. Concept: Destructuring Bindings",
        module_type: ModuleType::Concept,
        content: r#"
# Pull Values Out of a Tuple

Rust lets you destructure values right inside a `let`.

### Example
```rust
let point = (3, 7);
let (x, y) = point;
```

This is still variable binding, just with a pattern on the left side.
        "#,
        initial_code: "// Patterns can appear on the left side of let bindings.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "vars-10-practice",
        title: "10. Practice: Destructure a Tuple",
        module_type: ModuleType::Practice,
        content: r#"
# Bind Both Tuple Values

### Task:
1. Create a tuple binding: `let point = (4, 9);`
2. Destructure it into `x` and `y`
        "#,
        initial_code: "// Create point = (4, 9)\n// then destructure it into x and y\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "create the tuple binding `point`",
                    matcher: RuleMatcher::Contains("let point = (4, 9);"),
                },
                ValidationRule {
                    label: "destructure the tuple into x and y",
                    matcher: RuleMatcher::Contains("let (x, y) = point;"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let point = (4, 9);\nlet (x, y) = point;"),
            hints: &[
                "The left side of the second binding should be the tuple pattern `(x, y)`.",
                "Keep `point` as the tuple variable name so the destructuring reads naturally.",
            ],
        },
        success_message: "Well done. You used a binding pattern to unpack the tuple.",
    },
];
