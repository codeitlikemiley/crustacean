use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "cf-1-concept",
        title: "1. Concept: If Expressions",
        module_type: ModuleType::Concept,
        content: r#"
# Branching with `if`

In Rust, `if` is an **expression**, meaning it returns a value. This allows you to use it on the right side of a `let` statement.

### Key Rules
- The condition must be a `bool`. Rust does not have "truthy" or "falsy" values like JavaScript.
- Because it's an expression, all branches must return the same type if used in an assignment.

```rust
let number = 3;
if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```
        "#,
        initial_code: "// Read about if expressions, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "cf-2-practice",
        title: "2. Practice: Basic If/Else",
        module_type: ModuleType::Practice,
        content: r#"
# Write an If/Else Block

### Task:
1. Create a variable `number` set to `7`.
2. Write an `if` statement: if `number` is greater than `5`, print `"Big"`, otherwise print `"Small"`.
        "#,
        initial_code: "// Create number and write the if/else block\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "declare `number = 7`",
                    matcher: RuleMatcher::FlexContains("let number = 7;"),
                },
                ValidationRule {
                    label: "if number > 5",
                    matcher: RuleMatcher::FlexContains("if number > 5"),
                },
                ValidationRule {
                    label: "print \"Big\"",
                    matcher: RuleMatcher::Contains("println!(\"Big\")"),
                },
                ValidationRule {
                    label: "else branch",
                    matcher: RuleMatcher::Contains("else"),
                },
                ValidationRule {
                    label: "print \"Small\"",
                    matcher: RuleMatcher::Contains("println!(\"Small\")"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let number = 7;\nif number > 5 {\n    println!(\"Big\");\n} else {\n    println!(\"Small\");\n}"),
            hints: &[
                "Ensure your condition `number > 5` is correct.",
                "Remember the `else` keyword for the fallback branch.",
            ],
        },
        success_message: "Well done! You've mastered basic branching.",
    },
    TutorialModule {
        id: "cf-3-concept",
        title: "3. Concept: Repetition with loop",
        module_type: ModuleType::Concept,
        content: r#"
# Infinite Loops

The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

### Stopping a Loop
- `break`: Exits the loop immediately.
- `continue`: Skips the rest of the current iteration and starts the next one.

```rust
loop {
    println!("again!");
    break; // Stops the loop
}
```
        "#,
        initial_code: "// Study the loop keyword, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "cf-4-practice",
        title: "4. Practice: Breaking a Loop",
        module_type: ModuleType::Practice,
        content: r#"
# Use loop and break

### Task:
1. Create a `loop` block.
2. Inside, use `break;` to exit immediately.
        "#,
        initial_code: "// Write a loop that breaks immediately\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use the `loop` keyword",
                    matcher: RuleMatcher::Contains("loop {"),
                },
                ValidationRule {
                    label: "use `break;` inside the loop",
                    matcher: RuleMatcher::FlexContains("break;"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("loop {\n    break;\n}"),
            hints: &[
                "The `break` keyword must be inside the curly braces of the `loop`.",
            ],
        },
        success_message: "Exactly! `break` is essential for controlling infinite loops.",
    },
    TutorialModule {
        id: "cf-5-concept",
        title: "5. Concept: While Loops",
        module_type: ModuleType::Concept,
        content: r#"
# Conditional Loops

A `while` loop runs as long as a condition remains true.

```rust
let mut number = 3;
while number != 0 {
    println!("{number}!");
    number -= 1;
}
```
        "#,
        initial_code: "// Review while loops, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "cf-6-practice",
        title: "6. Practice: Counting Down",
        module_type: ModuleType::Practice,
        content: r#"
# Use a while loop

### Task:
1. Create a mutable `count` set to `3`.
2. While `count` is not `0`, decrement `count`.
        "#,
        initial_code: "// Implement the countdown loop\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "mutable count = 3",
                    matcher: RuleMatcher::FlexContains("let mut count = 3;"),
                },
                ValidationRule {
                    label: "while count != 0",
                    matcher: RuleMatcher::FlexContains("while count != 0"),
                },
                ValidationRule {
                    label: "decrement count",
                    matcher: RuleMatcher::AnyContains(&["count -= 1", "count = count - 1"]),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let mut count = 3;\nwhile count != 0 {\n    count -= 1;\n}"),
            hints: &[
                "Make sure `count` is `mut` so you can change it.",
                "The decrement `count -= 1;` should be inside the `while` block.",
            ],
        },
        success_message: "Nice! You used a conditional loop to manage state.",
    },
    TutorialModule {
        id: "cf-7-concept",
        title: "7. Concept: For Loops and Ranges",
        module_type: ModuleType::Concept,
        content: r#"
# Iterating with for

The `for` loop is the most common loop in Rust. It's used to iterate over elements of a collection or a range.

### Ranges
- `1..4`: From 1 up to (but not including) 4.
- `1..=4`: From 1 up to and including 4.

```rust
for number in 1..4 {
    println!("{number}");
}
```
        "#,
        initial_code: "// Study for loops, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "cf-8-practice",
        title: "8. Practice: Simple For Loop",
        module_type: ModuleType::Practice,
        content: r#"
# Iterate over a Range

### Task:
Write a `for` loop that iterates from `1` to `5` (inclusive) and prints each number.
        "#,
        initial_code: "// Write the for loop here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "for loop with range 1..=5",
                    matcher: RuleMatcher::Regex(r"for\s+\w+\s+in\s+1\.\.=5"),
                },
                ValidationRule {
                    label: "print the loop variable",
                    matcher: RuleMatcher::Contains("println!"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("for n in 1..=5 {\n    println!(\"{}\", n);\n}"),
            hints: &[
                "Use the `..=` syntax for an inclusive range.",
                "Choose any variable name for the loop iterator (e.g., `i` or `n`).",
            ],
        },
        success_message: "Excellent! You've completed the control flow fundamentals.",
    },
];
