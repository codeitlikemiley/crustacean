use crate::data::model::{TutorialModule, ModuleType};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "vars-concept-1",
        title: "1. Variables and Mutability",
        module_type: ModuleType::Concept,
        content: r#"
# Variables and Mutability

In Rust, variables are **immutable by default**.

When a variable is immutable, once a value is bound to a name, you can't change that value.

### Key Points:
- `let x = 5;` — x cannot be changed
- `let mut x = 5;` — x can be modified with `x = 6;`
- Immutability prevents entire classes of bugs
- The Rust compiler catches accidental mutations at compile time

**Mental Hook**: Rust is paranoid by default — it assumes you don't want things to change unless you explicitly say so.
        "#,
        initial_code: "// Concept: Study the theory on the left.\n// Click \"ACKNOWLEDGE\" to continue!",
        solution: None,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "vars-practice-1",
        title: "2. Practice: Mutable Variables",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Make It Mutable

### Task:
Create a mutable variable called `counter` initialized to `0`, then reassign it to `1`.
        "#,
        initial_code: "// Create a mutable variable named `counter`\n// and reassign it to 1\n",
        solution: Some(r"let\s+mut\s+counter\s*=\s*0\s*;\s*counter\s*=\s*1"),
        success_message: "Perfect! You've mastered mutable variables.",
    },
    TutorialModule {
        id: "vars-concept-2",
        title: "3. Shadowing",
        module_type: ModuleType::Concept,
        content: r#"
# Shadowing

Rust lets you **shadow** a variable by re-declaring it with `let`.

Unlike mutation, shadowing:
- Creates a new variable in the same scope
- Lets you change the type
- Lets you perform transformations without `mut`

### Example:
```rust
let x = 5;
let x = x + 1; // x is now 6
let x = \"six\"; // x is now a string!
```

**Mental Hook**: Shadowing is like wearing a new mask — same name, different face.
        "#,
        initial_code: "// Concept: Study the theory on the left.\n// Click \"ACKNOWLEDGE\" to continue!",
        solution: None,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "vars-practice-2",
        title: "4. Practice: Shadowing",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Shadowing

### Task:
Create a variable `spaces` bound to `"   "`, then shadow it to hold `spaces.len()`.
        "#,
        initial_code: "// Shadow the variable `spaces`\n// from a string to its length\n",
        solution: Some(r"let\s+spaces\s*=\s*\".*?\"\s*;.*?let\s+spaces\s*=\s*spaces\.len\(\)"),
        success_message: "Shadowing mastered!",
    },
    TutorialModule {
        id: "vars-final",
        title: "5. Summary",
        module_type: ModuleType::Concept,
        content: r#"
# Variables Summary

You've learned:
- Variables are **immutable by default**
- Use `mut` for mutable bindings
- Use **shadowing** to re-bind with new types

You're ready for the next level!
        "#,
        initial_code: "// Concept: Study the theory on the left.\n// Click \"ACKNOWLEDGE\" to finish!",
        solution: None,
        success_message: "Variables mastery achieved!",
    },
];
