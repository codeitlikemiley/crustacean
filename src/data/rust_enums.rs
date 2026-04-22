use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "enums-1-concept",
        title: "1. Concept: Enums",
        module_type: ModuleType::Concept,
        content: r#"
# Enumerations (Enums)

An enum allows you to define a type by enumerating its possible variants.

Unlike structs (which have *all* of their fields at the same time), an enum value can only be *one* of its variants at a time.

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

let current_light = TrafficLight::Green;
```
        "#,
        initial_code: "// Learn about enums, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "enums-2-practice",
        title: "2. Practice: Define an Enum",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `Direction`

### Task:
Define an enum named `Direction` with four variants: `Up`, `Down`, `Left`, and `Right`.
        "#,
        initial_code: "// Define `enum Direction` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `enum Direction`",
                    matcher: RuleMatcher::Regex(r#"enum\s+Direction"#),
                },
                ValidationRule {
                    label: "variant `Up`",
                    matcher: RuleMatcher::Contains("Up,"),
                },
                ValidationRule {
                    label: "variant `Down`",
                    matcher: RuleMatcher::Contains("Down,"),
                },
                ValidationRule {
                    label: "variant `Left`",
                    matcher: RuleMatcher::Contains("Left,"),
                },
                ValidationRule {
                    label: "variant `Right`",
                    matcher: RuleMatcher::Contains("Right"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("enum Direction {\n    Up,\n    Down,\n    Left,\n    Right,\n}"),
            hints: &[
                "Start with `enum Direction {`",
                "List the variants inside, separated by commas.",
            ],
        },
        success_message: "Nice! You defined a custom enumeration.",
    },
    TutorialModule {
        id: "enums-3-concept",
        title: "3. Concept: Enums with Data",
        module_type: ModuleType::Concept,
        content: r#"
# Enums Can Hold Data!

This is where Rust's enums shine. Each variant can hold different types and amounts of associated data.

```rust
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Struct-like data
    Write(String),              // Tuple-like data
    ChangeColor(i32, i32, i32), // Tuple with 3 elements
}

let msg = Message::Write(String::from("hello"));
```
        "#,
        initial_code: "// Study enums with data, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "enums-4-practice",
        title: "4. Practice: Shape Enum",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Data Variants

### Task:
Define an enum named `Shape` with two variants:
- `Circle` containing a single `f64` (radius)
- `Rect` containing two `f64`s (width and height)
        "#,
        initial_code: "// Define `enum Shape` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `enum Shape`",
                    matcher: RuleMatcher::Regex(r#"enum\s+Shape"#),
                },
                ValidationRule {
                    label: "variant `Circle(f64)`",
                    matcher: RuleMatcher::Regex(r#"Circle\s*\(\s*f64\s*\)"#),
                },
                ValidationRule {
                    label: "variant `Rect(f64, f64)`",
                    matcher: RuleMatcher::Regex(r#"Rect\s*\(\s*f64\s*,\s*f64\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("enum Shape {\n    Circle(f64),\n    Rect(f64, f64),\n}"),
            hints: &[
                "Variants with data look like tuple structs: `VariantName(Type, Type)`",
                "Don't forget the commas between variants.",
            ],
        },
        success_message: "Great! This is incredibly powerful for representing state.",
    },
    TutorialModule {
        id: "enums-5-concept",
        title: "5. Concept: Pattern Matching",
        module_type: ModuleType::Concept,
        content: r#"
# The `match` Control Flow

The `match` operator allows you to compare a value against a series of patterns and execute code.

It is **exhaustive**, meaning you must cover every possible case!

```rust
enum Coin {
    Penny,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter => 25,
    }
}
```

If you don't care about all variants, use the `_` placeholder to catch the rest.
        "#,
        initial_code: "// Read about match, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "enums-6-practice",
        title: "6. Practice: Match on Direction",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Matching Enums

### Task:
Complete the `as_str` method. Use a `match` expression on `self` to return the corresponding string slice (`"Up"`, `"Down"`, `"Left"`, `"Right"`).
        "#,
        initial_code: "enum Direction {\n    Up,\n    Down,\n    Left,\n    Right,\n}\n\nimpl Direction {\n    fn as_str(&self) -> &str {\n        // match on self here\n    }\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use `match self`",
                    matcher: RuleMatcher::Regex(r#"match\s+self\s*\{"#),
                },
                ValidationRule {
                    label: "match `Direction::Up` or `Self::Up`",
                    matcher: RuleMatcher::AnyContains(&["Direction::Up", "Self::Up"]),
                },
                ValidationRule {
                    label: "return \"Up\"",
                    matcher: RuleMatcher::Regex(r#"=>\s*"Up""#),
                },
                ValidationRule {
                    label: "cover all 4 variants",
                    matcher: RuleMatcher::AnyContains(&["\"Down\"", "\"Left\"", "\"Right\""]),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not use return statements in the match arms",
                    matcher: RuleMatcher::Regex(r#"=>\s*return"#),
                },
            ],
            canonical_solution: Some("impl Direction {\n    fn as_str(&self) -> &str {\n        match self {\n            Self::Up => \"Up\",\n            Self::Down => \"Down\",\n            Self::Left => \"Left\",\n            Self::Right => \"Right\",\n        }\n    }\n}"),
            hints: &[
                "Inside the function, write `match self {`",
                "Each arm looks like `Self::Up => \"Up\",`",
            ],
        },
        success_message: "Perfect! You've safely exhausted all possibilities.",
    },
    TutorialModule {
        id: "enums-7-concept",
        title: "7. Concept: `Option<T>`",
        module_type: ModuleType::Concept,
        content: r#"
# Rust's Null Replacement

Rust does not have a `null` keyword. Instead, it uses an enum called `Option<T>` to represent the concept of "a value may or may not be present".

It is so common that it is built into the standard library prelude.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Because `Option` is an enum, the compiler forces you to use `match` (or similar tools) to extract the `T` from `Some(T)`, completely eliminating Null Pointer Exceptions!
        "#,
        initial_code: "// Study Option<T>, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "enums-8-practice",
        title: "8. Practice: Safe Division",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Returning `Option`

### Task:
Write a function `divide` that takes `numerator: f64` and `denominator: f64`.
If the denominator is `0.0`, return `None`.
Otherwise, return `Some(numerator / denominator)`.
        "#,
        initial_code: "// Write `fn divide` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn divide`",
                    matcher: RuleMatcher::Regex(r#"fn\s+divide"#),
                },
                ValidationRule {
                    label: "return type `-> Option<f64>`",
                    matcher: RuleMatcher::Regex(r#"->\s*Option<f64>"#),
                },
                ValidationRule {
                    label: "check for 0.0",
                    matcher: RuleMatcher::Regex(r#"denominator\s*==\s*0\.0"#),
                },
                ValidationRule {
                    label: "return None",
                    matcher: RuleMatcher::Regex(r#"\{\s*None\s*\}|=>\s*None"#),
                },
                ValidationRule {
                    label: "return Some(...)",
                    matcher: RuleMatcher::Regex(r#"Some\(\s*numerator\s*/\s*denominator\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn divide(numerator: f64, denominator: f64) -> Option<f64> {\n    if denominator == 0.0 {\n        None\n    } else {\n        Some(numerator / denominator)\n    }\n}"),
            hints: &[
                "Signature: `fn divide(numerator: f64, denominator: f64) -> Option<f64>`",
                "Use an `if denominator == 0.0 { None } else { Some(...) }` expression.",
            ],
        },
        success_message: "Excellent! You've made division completely safe.",
    },
    TutorialModule {
        id: "enums-9-concept",
        title: "9. Concept: `if let`",
        module_type: ModuleType::Concept,
        content: r#"
# Concise Control Flow with `if let`

Sometimes `match` is too wordy when you only care about *one* variant and want to ignore the rest.

`if let` allows you to pattern match and execute code in one step.

```rust
let some_value = Some(3);

// Verbose match:
match some_value {
    Some(max) => println!("The max is {}", max),
    _ => (), // Ignore None
}

// Concise if let:
if let Some(max) = some_value {
    println!("The max is {}", max);
}
```
        "#,
        initial_code: "// Study if let, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "enums-10-practice",
        title: "10. Practice: Using `if let`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Unwrap safely

### Task:
Use `if let` to check if `user_age` is `Some(age)`. If it is, print `"Age: {}"` with the age value. Do nothing if it's `None`.
        "#,
        initial_code: "fn print_age(user_age: Option<u8>) {\n    // Use `if let` here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use `if let Some(age) = user_age`",
                    matcher: RuleMatcher::Regex(r#"if\s+let\s+Some\s*\(\s*\w+\s*\)\s*=\s*user_age"#),
                },
                ValidationRule {
                    label: "use println!",
                    matcher: RuleMatcher::Contains("println!("),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not use match",
                    matcher: RuleMatcher::Contains("match "),
                },
            ],
            canonical_solution: Some("fn print_age(user_age: Option<u8>) {\n    if let Some(age) = user_age {\n        println!(\"Age: {}\", age);\n    }\n}"),
            hints: &[
                "The syntax is `if let Some(age) = user_age { ... }`",
                "Inside the block, `println!(\"Age: {}\", age);`",
            ],
        },
        success_message: "Great! `if let` makes working with Option very clean.",
    },
    TutorialModule {
        id: "enums-11-concept",
        title: "11. Concept: `Result<T, E>`",
        module_type: ModuleType::Concept,
        content: r#"
# Errors as Values

While `Option` represents "something or nothing", `Result` represents "success or failure".

It's another built-in enum used everywhere for recoverable errors:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

File I/O, network requests, and parsing all return `Result`. You match on `Ok(value)` or `Err(error)`.
        "#,
        initial_code: "// Study Result<T, E>, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "enums-12-practice",
        title: "12. Practice: Handle a `Result`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Parsing Strings

The `.parse::<i32>()` method on a string returns a `Result<i32, ParseIntError>`.

### Task:
Complete the `parse_and_print` function. Use a `match` on `text.parse::<i32>()`.
- If `Ok(num)`, print `"Parsed: {}"`
- If `Err(_)`, print `"Failed to parse"`
        "#,
        initial_code: "fn parse_and_print(text: &str) {\n    match text.parse::<i32>() {\n        // Add Ok and Err arms here\n    }\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "match `Ok(num)`",
                    matcher: RuleMatcher::Regex(r#"Ok\s*\(\s*\w+\s*\)\s*=>"#),
                },
                ValidationRule {
                    label: "print parsed number",
                    matcher: RuleMatcher::Contains("\"Parsed:"),
                },
                ValidationRule {
                    label: "match `Err(_)`",
                    matcher: RuleMatcher::Regex(r#"Err\s*\(\s*_\s*\)\s*=>|Err\s*\(\s*\w+\s*\)\s*=>"#),
                },
                ValidationRule {
                    label: "print failure",
                    matcher: RuleMatcher::Contains("\"Failed to parse\""),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn parse_and_print(text: &str) {\n    match text.parse::<i32>() {\n        Ok(num) => println!(\"Parsed: {}\", num),\n        Err(_) => println!(\"Failed to parse\"),\n    }\n}"),
            hints: &[
                "The Ok arm should look like `Ok(num) => println!(\"Parsed: {}\", num),`",
                "The Err arm can ignore the error value: `Err(_) => println!(\"Failed to parse\"),`",
            ],
        },
        success_message: "Outstanding! You've mastered Rust's powerful enum-based control flow.",
    },
];
