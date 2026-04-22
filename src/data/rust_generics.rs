use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "generics-1-concept",
        title: "1. Concept: Generic Functions",
        module_type: ModuleType::Concept,
        content: r#"
# Generics

Generics allow us to write code that works with multiple types rather than a specific one.

We place the generic type parameter (usually `T`) inside angle brackets `<T>` right after the function name.

```rust
// This function works for any type T
fn print_type<T>(item: T) {
    // ...
}
```

When the compiler builds your code, it performs **monomorphization** — it generates specific, fast copies of the generic function for every type you actually use in your program. There is zero runtime cost!
        "#,
        initial_code: "// Study generics, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "generics-2-practice",
        title: "2. Practice: Generic Function",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `make_pair`

### Task:
Write a generic function `make_pair<T>` that takes two parameters `a: T` and `b: T` and returns a tuple `(T, T)`.
        "#,
        initial_code: "// Write `fn make_pair<T>` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn make_pair<T>`",
                    matcher: RuleMatcher::Regex(r#"fn\s+make_pair\s*<\s*T\s*>"#),
                },
                ValidationRule {
                    label: "accept `a: T, b: T`",
                    matcher: RuleMatcher::Regex(r#"a\s*:\s*T\s*,\s*b\s*:\s*T"#),
                },
                ValidationRule {
                    label: "return `-> (T, T)`",
                    matcher: RuleMatcher::Regex(r#"->\s*\(\s*T\s*,\s*T\s*\)"#),
                },
                ValidationRule {
                    label: "return `(a, b)`",
                    matcher: RuleMatcher::Regex(r#"\(\s*a\s*,\s*b\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn make_pair<T>(a: T, b: T) -> (T, T) {\n    (a, b)\n}"),
            hints: &[
                "Signature: `fn make_pair<T>(a: T, b: T) -> (T, T)`",
                "Return the tuple `(a, b)`",
            ],
        },
        success_message: "Nice! This function can now pair integers, strings, or any custom struct.",
    },
    TutorialModule {
        id: "generics-3-concept",
        title: "3. Concept: Generic Structs",
        module_type: ModuleType::Concept,
        content: r#"
# Generic Structs

Structs can also use generics to hold data of any type.

```rust
struct Point<T> {
    x: T,
    y: T,
}

let int_point = Point { x: 5, y: 10 };
let float_point = Point { x: 1.0, y: 4.0 };
```

If you need `x` and `y` to potentially be *different* types, you use multiple generic parameters:
```rust
struct MixedPoint<T, U> {
    x: T,
    y: U,
}
```
        "#,
        initial_code: "// Read about generic structs, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "generics-4-practice",
        title: "4. Practice: Define `Result`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Custom Result Enum

### Task:
Just like structs, enums can be generic! Define an enum `MyResult<T, E>` with two variants:
- `Ok` holding `T`
- `Err` holding `E`
        "#,
        initial_code: "// Define `enum MyResult<T, E>` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `enum MyResult<T, E>`",
                    matcher: RuleMatcher::Regex(r#"enum\s+MyResult\s*<\s*T\s*,\s*E\s*>"#),
                },
                ValidationRule {
                    label: "variant `Ok(T)`",
                    matcher: RuleMatcher::Regex(r#"Ok\s*\(\s*T\s*\)"#),
                },
                ValidationRule {
                    label: "variant `Err(E)`",
                    matcher: RuleMatcher::Regex(r#"Err\s*\(\s*E\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("enum MyResult<T, E> {\n    Ok(T),\n    Err(E),\n}"),
            hints: &[
                "It looks exactly like the standard library `Result`: `enum MyResult<T, E> { Ok(T), Err(E) }`",
            ],
        },
        success_message: "Great! That's exactly how `std::result::Result` is defined.",
    },
    TutorialModule {
        id: "generics-5-concept",
        title: "5. Concept: Trait Bounds",
        module_type: ModuleType::Concept,
        content: r#"
# Constraining Generics

If you write a generic function, you might want to call methods on the generic value (like `.clone()` or printing it).

You can't do this with a completely raw `<T>`, because the compiler doesn't know if `T` supports those methods!

You fix this using **trait bounds**, restricting `T` to types that implement certain traits.

```rust
use std::fmt::Display;

// T must implement Display
fn print_it<T: Display>(item: T) {
    println!("{}", item);
}
```

You can require multiple traits using `+`: `<T: Display + Clone>`
        "#,
        initial_code: "// Study trait bounds, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "generics-6-practice",
        title: "6. Practice: Use a Bound",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `PartialEq` Bound

### Task:
Fix the `is_equal` function so it compiles. It compares `a == b`, so `T` must be constrained by the `PartialEq` trait.
        "#,
        initial_code: "fn is_equal<T>(a: T, b: T) -> bool {\n    a == b\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "add `PartialEq` bound to `T`",
                    matcher: RuleMatcher::Regex(r#"T\s*:\s*PartialEq"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn is_equal<T: PartialEq>(a: T, b: T) -> bool {\n    a == b\n}"),
            hints: &[
                "Change `<T>` to `<T: PartialEq>` in the function signature.",
            ],
        },
        success_message: "Perfect! Trait bounds are the cornerstone of Rust's generic system.",
    },
    TutorialModule {
        id: "generics-7-concept",
        title: "7. Concept: `where` Clauses",
        module_type: ModuleType::Concept,
        content: r#"
# Cleaner Trait Bounds

When you have multiple generic parameters and multiple trait bounds, the function signature can become very messy.

Rust provides the `where` clause to move the bounds to a cleaner location right before the opening brace.

```rust
// Messy:
fn some_fn<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 { ... }

// Clean:
fn some_fn<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```
        "#,
        initial_code: "// Study where clauses, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "generics-8-practice",
        title: "8. Practice: Write a `where` clause",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Refactor to `where`

### Task:
Refactor the function signature to use a `where` clause for the bounds on `T` and `U`.
        "#,
        initial_code: "fn combine<T: Clone, U: Clone>(t: T, u: U) {}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use `where`",
                    matcher: RuleMatcher::Contains("where"),
                },
                ValidationRule {
                    label: "bound `T: Clone` in where",
                    matcher: RuleMatcher::Regex(r#"where[\s\S]*T\s*:\s*Clone"#),
                },
                ValidationRule {
                    label: "bound `U: Clone` in where",
                    matcher: RuleMatcher::Regex(r#"where[\s\S]*U\s*:\s*Clone"#),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not put bounds in the angle brackets",
                    matcher: RuleMatcher::Regex(r#"<\s*T\s*:\s*Clone"#),
                },
            ],
            canonical_solution: Some("fn combine<T, U>(t: T, u: U)\nwhere\n    T: Clone,\n    U: Clone,\n{}"),
            hints: &[
                "Change `<T: Clone, U: Clone>` to just `<T, U>`.",
                "Add `where T: Clone, U: Clone` before the `{}`.",
            ],
        },
        success_message: "Awesome. The `where` clause is the idiomatic way to write complex generic bounds.",
    },
    TutorialModule {
        id: "generics-9-concept",
        title: "9. Concept: Lifetimes",
        module_type: ModuleType::Concept,
        content: r#"
# Lifetimes: `'a`

Every reference in Rust has a **lifetime** — the scope for which that reference is valid. The borrow checker ensures that references never outlive the data they point to (no dangling pointers!).

Usually, lifetimes are inferred. But when a function returns a reference, and it takes *multiple* references as arguments, the compiler needs your help.

Lifetimes are a kind of generic parameter. They start with an apostrophe: `'a`.

```rust
// This says: the returned reference will live at least as long
// as BOTH x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```
        "#,
        initial_code: "// Study lifetimes, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "generics-10-practice",
        title: "10. Practice: Annotate Lifetimes",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Return a Reference

### Task:
Fix the `first_word` function by adding a lifetime parameter `'a` so the compiler knows the returned reference is tied to the input string slice.
        "#,
        initial_code: "fn first_word(s: &str) -> &str {\n    s\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "declare `<'a>`",
                    matcher: RuleMatcher::Regex(r#"fn\s+first_word\s*<\s*'a\s*>"#),
                },
                ValidationRule {
                    label: "annotate input `s: &'a str`",
                    matcher: RuleMatcher::Regex(r#"s\s*:\s*&'a\s+str"#),
                },
                ValidationRule {
                    label: "annotate output `-> &'a str`",
                    matcher: RuleMatcher::Regex(r#"->\s*&'a\s+str"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn first_word<'a>(s: &'a str) -> &'a str {\n    s\n}"),
            hints: &[
                "Declare the lifetime: `fn first_word<'a>`",
                "Apply it to input and output: `(s: &'a str) -> &'a str`",
            ],
        },
        success_message: "Well done! (Note: modern Rust actually infers this specific case via 'lifetime elision', but it's crucial to know how to write it manually).",
    },
    TutorialModule {
        id: "generics-11-concept",
        title: "11. Concept: Structs with References",
        module_type: ModuleType::Concept,
        content: r#"
# Lifetimes in Structs

If a struct holds a reference, you **must** specify a lifetime annotation.

This tells the compiler: "An instance of this struct cannot outlive the reference it holds inside."

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

let text = String::from("Call me Ishmael. Some years ago...");
let first_sentence = text.split('.').next().unwrap();

// i holds a reference to `text`, so `i` cannot outlive `text`
let i = ImportantExcerpt { part: first_sentence };
```
        "#,
        initial_code: "// Read about structs with references, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "generics-12-practice",
        title: "12. Practice: Lifetime in a Struct",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `Config` Struct

### Task:
Define a struct named `Config` that has a generic lifetime parameter `'a`.
It should have a single field `path` of type `&'a str`.
        "#,
        initial_code: "// Define `struct Config` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `struct Config<'a>`",
                    matcher: RuleMatcher::Regex(r#"struct\s+Config\s*<\s*'a\s*>"#),
                },
                ValidationRule {
                    label: "field `path: &'a str`",
                    matcher: RuleMatcher::Regex(r#"path\s*:\s*&'a\s+str"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("struct Config<'a> {\n    path: &'a str,\n}"),
            hints: &[
                "Start with `struct Config<'a> {`",
                "The field is `path: &'a str,`",
            ],
        },
        success_message: "Congratulations! You've grasped Generics and Lifetimes, some of Rust's most advanced features.",
    },
];
