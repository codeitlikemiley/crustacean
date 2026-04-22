use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "collections-1-concept",
        title: "1. Concept: `Vec<T>`",
        module_type: ModuleType::Concept,
        content: r#"
# Vectors (`Vec<T>`)

A vector allows you to store a variable number of values of the same type next to each other in memory. It can grow and shrink dynamically.

```rust
// Create an empty, mutable vector
let mut v: Vec<i32> = Vec::new();
v.push(5);
v.push(6);

// Use the vec! macro to create one with initial values
let v2 = vec![1, 2, 3];
```

You can read elements using indexing (`&v[0]`) or the `.get(0)` method, which returns an `Option<&T>` in case the index is out of bounds.
        "#,
        initial_code: "// Study Vec<T>, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "collections-2-practice",
        title: "2. Practice: Build a Vector",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Create and push

### Task:
Create a mutable vector named `numbers` using `Vec::new()`. Then push the number `42` into it.
        "#,
        initial_code: "// Create `numbers` and push 42\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let mut numbers`",
                    matcher: RuleMatcher::Regex(r#"let\s+mut\s+numbers\s*="#),
                },
                ValidationRule {
                    label: "use `Vec::new()`",
                    matcher: RuleMatcher::Contains("Vec::new()"),
                },
                ValidationRule {
                    label: "push `42`",
                    matcher: RuleMatcher::Regex(r#"numbers\.push\(\s*42\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let mut numbers = Vec::new();\nnumbers.push(42);"),
            hints: &[
                "Start with `let mut numbers = Vec::new();`",
                "Then call `numbers.push(42);`",
            ],
        },
        success_message: "Great! You've dynamically allocated memory on the heap.",
    },
    TutorialModule {
        id: "collections-3-concept",
        title: "3. Concept: Iterating over Vectors",
        module_type: ModuleType::Concept,
        content: r#"
# Iterating

You can use a `for` loop to iterate over the elements of a vector.

```rust
let v = vec![100, 32, 57];

// Iterate over immutable references
for i in &v {
    println!("{}", i);
}

// Iterate over mutable references
let mut v_mut = vec![1, 2, 3];
for i in &mut v_mut {
    *i += 50; // Dereference to modify
}
```

If you don't use `&`, the `for` loop will take ownership of the vector and you won't be able to use it afterward!
        "#,
        initial_code: "// Read about iteration, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "collections-4-practice",
        title: "4. Practice: Summing Elements",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: For Loops on Vectors

### Task:
Write a `for` loop that iterates over `&scores` and adds each score to `total`.
        "#,
        initial_code: "fn sum_scores(scores: Vec<i32>) -> i32 {\n    let mut total = 0;\n    // Write the for loop here\n    \n    total\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "loop over `&scores`",
                    matcher: RuleMatcher::Regex(r#"for\s+\w+\s+in\s+&scores"#),
                },
                ValidationRule {
                    label: "add to `total`",
                    matcher: RuleMatcher::Contains("total +="),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not consume scores without `&`",
                    matcher: RuleMatcher::Regex(r#"for\s+\w+\s+in\s+scores\s*\{"#),
                },
            ],
            canonical_solution: Some("fn sum_scores(scores: Vec<i32>) -> i32 {\n    let mut total = 0;\n    for s in &scores {\n        total += s;\n    }\n    total\n}"),
            hints: &[
                "Start the loop with `for s in &scores {`",
                "Inside the loop, do `total += s;`",
            ],
        },
        success_message: "Excellent! You iterated safely without consuming the vector.",
    },
    TutorialModule {
        id: "collections-5-concept",
        title: "5. Concept: `HashMap<K, V>`",
        module_type: ModuleType::Concept,
        content: r#"
# Hash Maps

A `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a hashing function.

It's perfect when you want to look up data not by using an index, but by using an identifying key.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Reading a value (returns Option<&V>)
let team_name = String::from("Blue");
let score = scores.get(&team_name); // Some(&10)
```
        "#,
        initial_code: "// Study HashMap, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "collections-6-practice",
        title: "6. Practice: Word Count Map",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Create a Map

### Task:
Create a mutable `HashMap` named `words`. Insert the key `"hello"` with the value `1`, and the key `"world"` with the value `2`.

*Note: You must bring `HashMap` into scope first.*
        "#,
        initial_code: "// Bring HashMap into scope\n\n// Create `words` and insert the values\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "import `HashMap`",
                    matcher: RuleMatcher::Regex(r#"use\s+std::collections::HashMap;"#),
                },
                ValidationRule {
                    label: "define `let mut words`",
                    matcher: RuleMatcher::Regex(r#"let\s+mut\s+words\s*="#),
                },
                ValidationRule {
                    label: "insert hello",
                    matcher: RuleMatcher::Regex(r#"words\.insert\(\s*"hello"\s*,\s*1\s*\)"#),
                },
                ValidationRule {
                    label: "insert world",
                    matcher: RuleMatcher::Regex(r#"words\.insert\(\s*"world"\s*,\s*2\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::collections::HashMap;\n\nlet mut words = HashMap::new();\nwords.insert(\"hello\", 1);\nwords.insert(\"world\", 2);"),
            hints: &[
                "First line: `use std::collections::HashMap;`",
                "Create it: `let mut words = HashMap::new();`",
                "Insert: `words.insert(\"hello\", 1);`",
            ],
        },
        success_message: "Perfect! You've used the standard dictionary type.",
    },
    TutorialModule {
        id: "collections-7-concept",
        title: "7. Concept: The Entry API",
        module_type: ModuleType::Concept,
        content: r#"
# Updating a HashMap Safely

Often you want to insert a value only if the key has no value, or update a value if the key exists.

The `entry` API is perfect for this:

```rust
let mut map = HashMap::new();
map.insert("Blue", 10);

// Only inserts if "Yellow" doesn't exist
map.entry("Yellow").or_insert(50);
map.entry("Blue").or_insert(50); // Does nothing

// Updating a value based on the old value
let count = map.entry("Red").or_insert(0); // Returns a mutable reference!
*count += 1;
```
        "#,
        initial_code: "// Read about the Entry API, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "collections-8-practice",
        title: "8. Practice: Letter Frequencies",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Count Characters

### Task:
Complete the loop to count the occurrences of each character in `text`.
Use `map.entry(c).or_insert(0)` and immediately dereference and increment it.
        "#,
        initial_code: "use std::collections::HashMap;\n\nfn count_chars(text: &str) -> HashMap<char, i32> {\n    let mut map = HashMap::new();\n    for c in text.chars() {\n        // Use the entry API to count `c`\n    }\n    map\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use `map.entry(c)`",
                    matcher: RuleMatcher::Contains("map.entry(c)"),
                },
                ValidationRule {
                    label: "chain `.or_insert(0)`",
                    matcher: RuleMatcher::Contains(".or_insert(0)"),
                },
                ValidationRule {
                    label: "dereference and increment: `*... += 1`",
                    matcher: RuleMatcher::Regex(r#"\*\w+\s*\+=\s*1|let\s+count\s*=\s*map\.entry.*?\n\s*\*count\s*\+=\s*1|\*map\.entry\(c\)\.or_insert\(0\)\s*\+=\s*1"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::collections::HashMap;\n\nfn count_chars(text: &str) -> HashMap<char, i32> {\n    let mut map = HashMap::new();\n    for c in text.chars() {\n        let count = map.entry(c).or_insert(0);\n        *count += 1;\n    }\n    map\n}"),
            hints: &[
                "You can do this in one line: `*map.entry(c).or_insert(0) += 1;`",
                "Or two lines: `let count = map.entry(c).or_insert(0); *count += 1;`",
            ],
        },
        success_message: "Awesome! The Entry API is idiomatic Rust for frequency counting.",
    },
    TutorialModule {
        id: "collections-9-concept",
        title: "9. Concept: `HashSet<T>`",
        module_type: ModuleType::Concept,
        content: r#"
# Hash Sets

A `HashSet<T>` is simply a `HashMap<T, ()>` under the hood. It stores a collection of unique values.

It is very fast for checking if an item exists in the collection.

```rust
use std::collections::HashSet;

let mut seen = HashSet::new();
seen.insert("apple");
seen.insert("banana");

// Returns true
let has_apple = seen.contains("apple");

// Inserting a duplicate returns false
let was_new = seen.insert("apple"); // false
```
        "#,
        initial_code: "// Study HashSet, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "collections-10-practice",
        title: "10. Practice: Find Uniques",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Unique Numbers

### Task:
Create a `HashSet` called `uniques`. Insert the numbers `1`, `2`, and `1` again.
Remember to import `HashSet` from `std::collections`.
        "#,
        initial_code: "// Import HashSet\n\n// Create `uniques` and insert 1, 2, 1\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "import `HashSet`",
                    matcher: RuleMatcher::Regex(r#"use\s+std::collections::HashSet;"#),
                },
                ValidationRule {
                    label: "define `let mut uniques`",
                    matcher: RuleMatcher::Regex(r#"let\s+mut\s+uniques\s*="#),
                },
                ValidationRule {
                    label: "insert `1`",
                    matcher: RuleMatcher::Regex(r#"uniques\.insert\(\s*1\s*\)"#),
                },
                ValidationRule {
                    label: "insert `2`",
                    matcher: RuleMatcher::Regex(r#"uniques\.insert\(\s*2\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::collections::HashSet;\n\nlet mut uniques = HashSet::new();\nuniques.insert(1);\nuniques.insert(2);\nuniques.insert(1);"),
            hints: &[
                "First line: `use std::collections::HashSet;`",
                "Create: `let mut uniques = HashSet::new();`",
                "Then call `uniques.insert(...)` three times.",
            ],
        },
        success_message: "Fantastic! You've mastered the core Rust collections.",
    },
];
