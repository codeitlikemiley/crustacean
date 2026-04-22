use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "macros-1-concept",
        title: "1. Concept: Declarative Macros",
        module_type: ModuleType::Concept,
        content: r#"
# Metaprogramming

Macros allow you to write code that writes other code. This metaprogramming happens before the compiler checks types or borrows.

The most common macros are **declarative macros**, defined using `macro_rules!`.

```rust
// Define a macro named `say_hello`
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!(); // This gets replaced by the block above
}
```

Macros are matched against patterns, and they expand into Rust code.
        "#,
        initial_code: "// Study declarative macros, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "macros-2-practice",
        title: "2. Practice: Create a Macro",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `say_rust!`

### Task:
Define a declarative macro named `say_rust` that expands to `println!("Rust!");` when called with no arguments.
        "#,
        initial_code: "// Define `macro_rules! say_rust` here\n\nfn main() {\n    say_rust!();\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use `macro_rules! say_rust`",
                    matcher: RuleMatcher::Regex(r#"macro_rules!\s+say_rust"#),
                },
                ValidationRule {
                    label: "match no arguments `() => {`",
                    matcher: RuleMatcher::Regex(r#"\(\s*\)\s*=>\s*\{"#),
                },
                ValidationRule {
                    label: "expand to `println!`",
                    matcher: RuleMatcher::Contains("println!(\"Rust!\");"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("macro_rules! say_rust {\n    () => {\n        println!(\"Rust!\");\n    };\n}\n\nfn main() {\n    say_rust!();\n}"),
            hints: &[
                "Start with `macro_rules! say_rust {`",
                "Add the pattern `() => { println!(\"Rust!\"); };`",
            ],
        },
        success_message: "Great! You wrote code that generates code.",
    },
    TutorialModule {
        id: "macros-3-concept",
        title: "3. Concept: Matching Tokens",
        module_type: ModuleType::Concept,
        content: r#"
# Macro Arguments

Macros don't take values like `i32`; they take syntax tree tokens.

You capture them using `$` followed by a name, a colon, and a designator (like `expr` for expression, `ident` for identifier, `ty` for type).

```rust
macro_rules! print_result {
    // Capture any expression as $e
    ($e:expr) => {
        println!("Result: {}", $e);
    };
}

fn main() {
    print_result!(5 + 6); 
    // Expands to: println!("Result: {}", 5 + 6);
}
```
        "#,
        initial_code: "// Study token matching, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "macros-4-practice",
        title: "4. Practice: Match Expressions",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `multiply!`

### Task:
Write a macro `multiply!` that takes two expressions `$a:expr` and `$b:expr` separated by a comma, and expands to `$a * $b`.
        "#,
        initial_code: "// Define `macro_rules! multiply` here\n\nfn main() {\n    let result = multiply!(2 + 3, 4);\n    // Expands to: (2 + 3) * 4\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `macro_rules! multiply`",
                    matcher: RuleMatcher::Regex(r#"macro_rules!\s+multiply"#),
                },
                ValidationRule {
                    label: "match `($a:expr, $b:expr)`",
                    matcher: RuleMatcher::Regex(r#"\(\s*\$a\s*:\s*expr\s*,\s*\$b\s*:\s*expr\s*\)"#),
                },
                ValidationRule {
                    label: "expand to `$a * $b`",
                    matcher: RuleMatcher::Regex(r#"=>\s*\{\s*\$a\s*\*\s*\$b\s*\}"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("macro_rules! multiply {\n    ($a:expr, $b:expr) => {\n        $a * $b\n    };\n}\n\nfn main() {\n    let result = multiply!(2 + 3, 4);\n}"),
            hints: &[
                "Pattern: `($a:expr, $b:expr) => { $a * $b };`",
            ],
        },
        success_message: "Awesome! Your macro now accepts parameters.",
    },
    TutorialModule {
        id: "macros-5-concept",
        title: "5. Concept: Repetition",
        module_type: ModuleType::Concept,
        content: r#"
# Variadic Arguments

Functions in Rust can't take an arbitrary number of arguments, but macros can!

We use `$()` followed by a separator (like `,`) and a repetition character (like `*` for zero or more, or `+` for one or more).

```rust
macro_rules! sum {
    // Match one or more expressions separated by commas
    ( $( $x:expr ),* ) => {
        {
            let mut temp = 0;
            // Repeat this block for every $x
            $(
                temp += $x;
            )*
            temp // Return the total
        }
    };
}

let total = sum!(1, 2, 3);
```
        "#,
        initial_code: "// Study repetition, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "macros-6-practice",
        title: "6. Practice: `my_vec!`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Custom `vec!`

### Task:
Complete the `my_vec!` macro. It should take zero or more expressions separated by commas.
Inside the expansion, loop over `$x` and `.push($x)` onto `temp_vec`.
        "#,
        initial_code: "macro_rules! my_vec {\n    ( $( $x:expr ),* ) => {\n        {\n            let mut temp_vec = Vec::new();\n            // Loop over $x and push it to temp_vec\n            \n            temp_vec\n        }\n    };\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "write the repetition block `$( ... )*`",
                    matcher: RuleMatcher::Regex(r#"\$\(\s*temp_vec\.push\(\s*\$x\s*\)\s*;\s*\)\*"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("macro_rules! my_vec {\n    ( $( $x:expr ),* ) => {\n        {\n            let mut temp_vec = Vec::new();\n            $(\n                temp_vec.push($x);\n            )*\n            temp_vec\n        }\n    };\n}"),
            hints: &[
                "Write `$(` on one line.",
                "Then `temp_vec.push($x);`",
                "Then `)*` to close the repetition block.",
            ],
        },
        success_message: "Incredible! You just built a clone of one of Rust's most famous macros.",
    },
    TutorialModule {
        id: "macros-7-concept",
        title: "7. Concept: Procedural Macros",
        module_type: ModuleType::Concept,
        content: r#"
# Procedural Macros

Declarative macros are powerful but limited to matching patterns.

**Procedural Macros** run arbitrary Rust code at compile time! They take an Abstract Syntax Tree (AST) as input, manipulate it, and return a new AST.

There are three kinds:
1. **Custom `#[derive]`**: E.g., `#[derive(Serialize)]` automatically generates serialization code for your struct.
2. **Attribute-like**: E.g., `#[tokio::main]` modifies the entire `main` function to setup an async runtime.
3. **Function-like**: E.g., `sql!("SELECT * FROM users")` parses the SQL string at compile time.

*Writing procedural macros requires a separate crate of type `proc-macro`, which is an advanced topic beyond this course.*
        "#,
        initial_code: "// Marvel at procedural macros, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
];
