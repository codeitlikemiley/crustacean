use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "strings-1-concept",
        title: "1. Concept: `String` vs `&str`",
        module_type: ModuleType::Concept,
        content: r#"
# `String` vs `&str`

Rust has two main types of strings: `String` and `&str` (string slice).

### `&str` (String Slice)
- Borrowed, read-only data.
- Often points to data compiled into the executable (string literals) or a slice of a `String`.
- Fixed size, cannot be mutated or grown.

### `String`
- Owned, heap-allocated data.
- Can be mutated, grown, and shrunk.
- Created from a literal using `String::from("hello")` or `"hello".to_string()`.

**Mental Hook**: `&str` is a view into text. `String` is a bucket that holds text.
        "#,
        initial_code: "// Study String vs &str, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "strings-2-practice",
        title: "2. Practice: Create a `String`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Heap-allocated String

### Task:
Create a variable `greeting` of type `String` with the value `"Hello, Rust!"`.
        "#,
        initial_code: "// Create a `String` named `greeting`\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let greeting`",
                    matcher: RuleMatcher::Regex(r#"let\s+greeting\s*="#),
                },
                ValidationRule {
                    label: "create a String",
                    matcher: RuleMatcher::AnyContains(&["String::from(\"Hello, Rust!\")", "\"Hello, Rust!\".to_string()", "\"Hello, Rust!\".to_owned()"]),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not use just a string literal",
                    matcher: RuleMatcher::Regex(r#"let\s+greeting\s*=\s*&?"Hello,\s*Rust!"\s*;"#),
                },
            ],
            canonical_solution: Some("let greeting = String::from(\"Hello, Rust!\");"),
            hints: &[
                "Use `String::from()` or `.to_string()` on the literal.",
                "Ensure the text exactly matches `\"Hello, Rust!\"`.",
            ],
        },
        success_message: "Nice! You've created an owned String.",
    },
    TutorialModule {
        id: "strings-3-concept",
        title: "3. Concept: String Methods",
        module_type: ModuleType::Concept,
        content: r#"
# Modifying Strings

Because `String` is heap-allocated and growable, you can modify it (if it's `mut`).

### Common Methods:
- `.push_str(&str)`: Appends a string slice.
- `.push(char)`: Appends a single character.
- `.len()`: Returns the length in bytes (not characters!).
- `.contains(&str)`: Returns `true` if the substring is found.

```rust
let mut s = String::from("foo");
s.push_str("bar"); // s is now "foobar"
s.push('!');       // s is now "foobar!"
```
        "#,
        initial_code: "// Review String methods, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "strings-4-practice",
        title: "4. Practice: The `format!` macro",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: String Formatting

Often, you want to build a `String` by combining variables, rather than pushing pieces one by one. The `format!` macro does exactly this. It works like `println!`, but returns a `String`.

### Task:
Use `format!` to create a `String` combining `first` and `last` into `full_name`. The result should be `"John Doe"`.
        "#,
        initial_code: "let first = \"John\";\nlet last = \"Doe\";\n// Create `full_name` using format!\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let full_name`",
                    matcher: RuleMatcher::Regex(r#"let\s+full_name\s*="#),
                },
                ValidationRule {
                    label: "use `format!`",
                    matcher: RuleMatcher::Regex(r#"format!\s*\(\s*"\{\}\s\{\}""#),
                },
                ValidationRule {
                    label: "pass `first` and `last`",
                    matcher: RuleMatcher::Contains("first, last"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let first = \"John\";\nlet last = \"Doe\";\nlet full_name = format!(\"{} {}\", first, last);"),
            hints: &[
                "The format string should be `\"{} {}\"`.",
                "Pass `first` and `last` as the arguments to `format!`.",
            ],
        },
        success_message: "Great! `format!` is incredibly useful for building strings.",
    },
    TutorialModule {
        id: "strings-5-concept",
        title: "5. Concept: String Slicing",
        module_type: ModuleType::Concept,
        content: r#"
# Slicing Strings

You can take a slice of a `String` to get an `&str` pointing to a portion of it.

You do this using a range: `&s[start..end]`

```rust
let s = String::from("hello world");
let hello = &s[0..5]; // "hello"
let world = &s[6..11]; // "world"
```

**Warning:** String indices are byte offsets. Slicing in the middle of a multi-byte UTF-8 character will cause a panic!
        "#,
        initial_code: "// Study string slices, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "strings-6-practice",
        title: "6. Practice: Extract a Substring",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Slicing

### Task:
Given the string `sentence`, create a variable `rust` that is a slice (`&str`) containing just the word `"Rust"`.
        "#,
        initial_code: "let sentence = String::from(\"I love Rust programming\");\n// Extract just \"Rust\" into a variable named `rust`\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let rust`",
                    matcher: RuleMatcher::Regex(r#"let\s+rust\s*="#),
                },
                ValidationRule {
                    label: "slice the string",
                    matcher: RuleMatcher::Regex(r#"&sentence\[7\.\.11\]"#),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not create a new string literal",
                    matcher: RuleMatcher::Regex(r#"let\s+rust\s*=\s*"Rust"\s*;"#),
                },
            ],
            canonical_solution: Some("let sentence = String::from(\"I love Rust programming\");\nlet rust = &sentence[7..11];"),
            hints: &[
                "The word \"Rust\" starts at index 7.",
                "The word \"Rust\" ends at index 11 (exclusive).",
            ],
        },
        success_message: "Well done! Slicing is zero-cost because it just creates a view.",
    },
    TutorialModule {
        id: "strings-7-concept",
        title: "7. Concept: UTF-8 and Iteration",
        module_type: ModuleType::Concept,
        content: r#"
# UTF-8 and Iteration

Rust strings are UTF-8 encoded. This means a single "character" can be 1 to 4 bytes long!

Because of this, you cannot index into a string like an array (e.g., `s[0]` is an error).

Instead, you iterate over a string using:
- `.chars()` to get Unicode scalar values (`char`).
- `.bytes()` to get raw bytes (`u8`).

```rust
for c in "🦀".chars() {
    println!("{}", c); // Prints the crab
}
for b in "🦀".bytes() {
    println!("{}", b); // Prints 4 separate byte values!
}
```
        "#,
        initial_code: "// Study UTF-8 iteration, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "strings-8-practice",
        title: "8. Practice: Count Vowels",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Iterating Characters

### Task:
Complete the `count_vowels` function. It takes an `&str` and returns the number of lowercase vowels ('a', 'e', 'i', 'o', 'u').
Use `.chars()` to iterate over the string.
        "#,
        initial_code: "fn count_vowels(text: &str) -> i32 {\n    let mut count = 0;\n    // Iterate over characters and increment count\n    \n    count\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "iterate using `.chars()`",
                    matcher: RuleMatcher::Regex(r#"for\s+\w+\s+in\s+text\.chars\(\)"#),
                },
                ValidationRule {
                    label: "match on the character",
                    matcher: RuleMatcher::AnyContains(&["match", "if"]),
                },
                ValidationRule {
                    label: "check for vowels",
                    matcher: RuleMatcher::Contains("'a' | 'e' | 'i' | 'o' | 'u'"),
                },
                ValidationRule {
                    label: "increment count",
                    matcher: RuleMatcher::Contains("count += 1"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn count_vowels(text: &str) -> i32 {\n    let mut count = 0;\n    for c in text.chars() {\n        match c {\n            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,\n            _ => (),\n        }\n    }\n    count\n}"),
            hints: &[
                "Use a `for c in text.chars() { ... }` loop.",
                "Inside the loop, use a `match c` block with the pattern `'a' | 'e' | 'i' | 'o' | 'u' => count += 1`.",
            ],
        },
        success_message: "Fantastic! You're safely iterating over UTF-8 text.",
    },
    TutorialModule {
        id: "strings-9-concept",
        title: "9. Concept: Ownership and Borrowing Strings",
        module_type: ModuleType::Concept,
        content: r#"
# Passing Strings to Functions

When you pass a `String` to a function, you transfer ownership. If you pass an `&str`, you just borrow it.

```rust
fn print_owned(s: String) {
    println!("{}", s);
} // s is dropped here!

fn print_borrowed(s: &str) {
    println!("{}", s);
} // s is not dropped, we just looked at it

let my_string = String::from("hello");

// We can pass a reference to a String where an &str is expected!
print_borrowed(&my_string); 

// But if we pass it by value, we lose ownership.
print_owned(my_string);
// println!("{}", my_string); // Error!
```
        "#,
        initial_code: "// Study string ownership, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "strings-10-practice",
        title: "10. Practice: Return a new String",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `&str` to `String`

### Task:
Write a function `make_loud` that takes a borrowed string slice `msg: &str` and returns an owned `String` that contains the same message, but with `"!!!"` appended to the end.
        "#,
        initial_code: "// Write `make_loud` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn make_loud`",
                    matcher: RuleMatcher::Regex(r#"fn\s+make_loud"#),
                },
                ValidationRule {
                    label: "accept `msg: &str`",
                    matcher: RuleMatcher::Regex(r#"msg\s*:\s*&str"#),
                },
                ValidationRule {
                    label: "return `-> String`",
                    matcher: RuleMatcher::Regex(r#"->\s*String"#),
                },
                ValidationRule {
                    label: "create format or push_str",
                    matcher: RuleMatcher::AnyContains(&["format!(", "push_str"]),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn make_loud(msg: &str) -> String {\n    format!(\"{}!!!\", msg)\n}"),
            hints: &[
                "The signature is `fn make_loud(msg: &str) -> String`.",
                "You can return `format!(\"{}!!!\", msg)`.",
            ],
        },
        success_message: "Amazing! You've completed the Strings & Slices course.",
    },
];
