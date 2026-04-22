use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "iterators-1-concept",
        title: "1. Concept: Closures",
        module_type: ModuleType::Concept,
        content: r#"
# Closures

Closures are anonymous functions you can save in a variable or pass as arguments to other functions.

Unlike regular functions, closures can **capture** values from the scope in which they are defined.

```rust
let add_one = |x: i32| -> i32 { x + 1 };

// Types can often be inferred:
let add_two = |x| x + 2;

println!("{}", add_one(5)); // 6
```

Notice the parameters are between vertical pipes `| |` instead of parentheses `( )`.
        "#,
        initial_code: "// Study closures, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "iterators-2-practice",
        title: "2. Practice: Define a Closure",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `multiply` closure

### Task:
Define a closure named `multiply` that takes two parameters `x` and `y` and returns `x * y`.
        "#,
        initial_code: "fn main() {\n    // Define `let multiply = ...` here\n    \n    // println!(\"{}\", multiply(2, 3));\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let multiply =`",
                    matcher: RuleMatcher::Regex(r#"let\s+multiply\s*="#),
                },
                ValidationRule {
                    label: "use pipes `|x, y|`",
                    matcher: RuleMatcher::Regex(r#"\|\s*x\s*,\s*y\s*\|"#),
                },
                ValidationRule {
                    label: "return `x * y`",
                    matcher: RuleMatcher::Regex(r#"x\s*\*\s*y"#),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not use the `fn` keyword",
                    matcher: RuleMatcher::Regex(r#"let\s+multiply\s*=\s*fn"#),
                },
            ],
            canonical_solution: Some("fn main() {\n    let multiply = |x, y| x * y;\n}"),
            hints: &[
                "The syntax is `let multiply = |x, y| x * y;`",
            ],
        },
        success_message: "Nice! Closures are incredibly concise.",
    },
    TutorialModule {
        id: "iterators-3-concept",
        title: "3. Concept: Closure Capture",
        module_type: ModuleType::Concept,
        content: r#"
# Capturing the Environment

Closures can capture variables from their surrounding scope in three ways, which correspond to three traits:

1. **`Fn`**: borrows immutably (`&T`)
2. **`FnMut`**: borrows mutably (`&mut T`)
3. **`FnOnce`**: takes ownership (`T`), consuming the variable

The compiler automatically infers which trait the closure implements based on what you do with the captured variables inside the closure body.

```rust
let mut count = 0;
// This closure mutates `count`, so it implements `FnMut`
let mut inc = || {
    count += 1;
};
inc();
```
        "#,
        initial_code: "// Study capture modes, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "iterators-4-practice",
        title: "4. Practice: Mutating Capture",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `FnMut`

### Task:
We have a mutable variable `total`. 
Define a mutable closure named `add_to_total` that takes a parameter `val` and adds it to `total` using `total += val`.
        "#,
        initial_code: "fn main() {\n    let mut total = 0;\n    // Define `let mut add_to_total = ...` here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let mut add_to_total =`",
                    matcher: RuleMatcher::Regex(r#"let\s+mut\s+add_to_total\s*="#),
                },
                ValidationRule {
                    label: "take parameter `|val|`",
                    matcher: RuleMatcher::Regex(r#"\|\s*val\s*\|"#),
                },
                ValidationRule {
                    label: "mutate `total += val`",
                    matcher: RuleMatcher::Regex(r#"total\s*\+=\s*val"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn main() {\n    let mut total = 0;\n    let mut add_to_total = |val| total += val;\n}"),
            hints: &[
                "The closure must be mutable: `let mut add_to_total =`",
                "The body should be `total += val`",
            ],
        },
        success_message: "Great! This is incredibly useful for stateful callbacks.",
    },
    TutorialModule {
        id: "iterators-5-concept",
        title: "5. Concept: The `Iterator` Trait",
        module_type: ModuleType::Concept,
        content: r#"
# The Iterator Pattern

In Rust, iterators are **lazy**. They do absolutely nothing until you call methods that consume them.

All iterators implement the `Iterator` trait, which requires defining one method: `next`.

```rust
pub trait Iterator {
    type Item; // The type of element being iterated
    fn next(&mut self) -> Option<Self::Item>;
}
```

When `next` returns `Some(value)`, you get the next element. When it returns `None`, the iteration is over.

```rust
let v = vec![1, 2];
let mut iter = v.iter(); // Nothing happens yet!

assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), None);
```
        "#,
        initial_code: "// Study the Iterator trait, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "iterators-6-practice",
        title: "6. Practice: Implement `Iterator`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Custom Iterator

### Task:
Implement the `Iterator` trait for the `Counter` struct. It should return numbers from `1` to `5`.
        "#,
        initial_code: "struct Counter {\n    count: u32,\n}\n\nimpl Iterator for Counter {\n    type Item = u32;\n\n    fn next(&mut self) -> Option<Self::Item> {\n        // Increment self.count.\n        // If count < 6, return Some(count).\n        // Else, return None.\n    }\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "increment `self.count += 1`",
                    matcher: RuleMatcher::Regex(r#"self\.count\s*\+=\s*1"#),
                },
                ValidationRule {
                    label: "if count < 6",
                    matcher: RuleMatcher::Regex(r#"if\s+self\.count\s*<\s*6"#),
                },
                ValidationRule {
                    label: "return Some",
                    matcher: RuleMatcher::Regex(r#"Some\(\s*self\.count\s*\)"#),
                },
                ValidationRule {
                    label: "return None",
                    matcher: RuleMatcher::Regex(r#"\{\s*None\s*\}|=>\s*None"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl Iterator for Counter {\n    type Item = u32;\n    fn next(&mut self) -> Option<Self::Item> {\n        self.count += 1;\n        if self.count < 6 {\n            Some(self.count)\n        } else {\n            None\n        }\n    }\n}"),
            hints: &[
                "First line inside `next`: `self.count += 1;`",
                "Then use `if self.count < 6 { Some(self.count) } else { None }`.",
            ],
        },
        success_message: "Awesome! You can now use your custom `Counter` in a `for` loop.",
    },
    TutorialModule {
        id: "iterators-7-concept",
        title: "7. Concept: Iterator Adapters",
        module_type: ModuleType::Concept,
        content: r#"
# Adapters (Transforming Iterators)

Adapters take an iterator and return a *new* iterator with modified behavior. Because iterators are lazy, you have to call a "consumer" method to actually trigger the work.

### Common Adapters:
- `.map(|x| ...)`: Transforms each element.
- `.filter(|x| ...)`: Keeps elements where the closure returns `true`.
- `.take(n)`: Stops after `n` elements.

```rust
let v = vec![1, 2, 3];

// This does NOTHING!
v.iter().map(|x| x + 1);

// This does the work and collects into a new Vec
let new_v: Vec<i32> = v.iter().map(|x| x + 1).collect();
```
        "#,
        initial_code: "// Study adapters, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "iterators-8-practice",
        title: "8. Practice: `filter` and `map`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Chaining Adapters

### Task:
Take `v.into_iter()`. Chain a `.filter()` to keep only even numbers (`x % 2 == 0`). Then chain a `.map()` to multiply each by 2.
Assign the result to `iter`.
        "#,
        initial_code: "fn main() {\n    let v = vec![1, 2, 3, 4, 5];\n    // let iter = v.into_iter()...\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let iter =`",
                    matcher: RuleMatcher::Regex(r#"let\s+iter\s*="#),
                },
                ValidationRule {
                    label: "chain `.filter(|x| x % 2 == 0)`",
                    matcher: RuleMatcher::Regex(r#"\.filter\s*\(\s*\|\w+\|\s*\w+\s*%\s*2\s*==\s*0\s*\)"#),
                },
                ValidationRule {
                    label: "chain `.map(|x| x * 2)`",
                    matcher: RuleMatcher::Regex(r#"\.map\s*\(\s*\|\w+\|\s*\w+\s*\*\s*2\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn main() {\n    let v = vec![1, 2, 3, 4, 5];\n    let iter = v.into_iter().filter(|x| x % 2 == 0).map(|x| x * 2);\n}"),
            hints: &[
                "Chain the methods: `v.into_iter().filter(|x| x % 2 == 0).map(|x| x * 2);`",
            ],
        },
        success_message: "Perfect! You've built a lazy data processing pipeline.",
    },
    TutorialModule {
        id: "iterators-9-concept",
        title: "9. Concept: Iterator Consumers",
        module_type: ModuleType::Concept,
        content: r#"
# Consumers (Executing Iterators)

Consumers are methods that call `next()` internally, pulling elements through the pipeline and producing a final value.

### Common Consumers:
- `.collect()`: Gathers elements into a collection (like `Vec` or `String`). You often need to specify the type.
- `.sum()`: Adds all elements together.
- `.fold(init, closure)`: Reduces the iterator to a single value by repeatedly applying a closure.

```rust
let v = vec![1, 2, 3];

let sum: i32 = v.iter().sum(); // 6

// fold takes an initial value (0) and an accumulator (acc)
let sum2 = v.iter().fold(0, |acc, x| acc + x); // 6
```
        "#,
        initial_code: "// Study consumers, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "iterators-10-practice",
        title: "10. Practice: Using `fold`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `fold`

### Task:
Use `v.iter().fold(...)` to calculate the sum of the **squares** of the elements in `v`.
Start with `0`, and in the closure `|acc, x|`, add `x * x` to `acc`.
        "#,
        initial_code: "fn main() {\n    let v = vec![1, 2, 3];\n    // let sum_of_squares = ...\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let sum_of_squares =`",
                    matcher: RuleMatcher::Regex(r#"let\s+sum_of_squares\s*="#),
                },
                ValidationRule {
                    label: "use `.fold(0, ...)`",
                    matcher: RuleMatcher::Regex(r#"\.fold\s*\(\s*0\s*,"#),
                },
                ValidationRule {
                    label: "closure adds square: `acc + (x * x)`",
                    matcher: RuleMatcher::Regex(r#"\|\s*\w+\s*,\s*\w+\s*\|\s*\w+\s*\+\s*\w+\s*\*\s*\w+"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn main() {\n    let v = vec![1, 2, 3];\n    let sum_of_squares = v.iter().fold(0, |acc, x| acc + x * x);\n}"),
            hints: &[
                "The syntax is `v.iter().fold(0, |acc, x| acc + x * x)`.",
            ],
        },
        success_message: "Outstanding! `fold` is incredibly powerful for complex reductions.",
    },
    TutorialModule {
        id: "iterators-11-concept",
        title: "11. Concept: Chaining it all together",
        module_type: ModuleType::Concept,
        content: r#"
# The Power of Chaining

In Rust, you rarely write explicit `for` loops for data transformation. Instead, you chain adapters and consumers.

This is highly optimized by the compiler (Zero-Cost Abstractions) and often compiles down to the exact same assembly as a hand-written `while` loop!

```rust
let text = "hello world rust programming";

// Find words longer than 4 characters, make them uppercase,
// and join them with a space.
let result: String = text.split_whitespace()
    .filter(|word| word.len() > 4)
    .map(|word| word.to_uppercase())
    .collect::<Vec<String>>()
    .join(" ");
```
        "#,
        initial_code: "// Marvel at iterator chaining, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "iterators-12-practice",
        title: "12. Practice: Data Pipeline",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Build a Pipeline

### Task:
You have a vector of strings. We want a `Vec<usize>` containing the lengths of words that start with the letter `"a"`.

Chain `.into_iter()`, `.filter(...)`, `.map(...)`, and `.collect::<Vec<usize>>()`.
        "#,
        initial_code: "fn main() {\n    let words = vec![\"apple\", \"banana\", \"apricot\", \"cherry\"];\n    // let a_lengths: Vec<usize> = words.into_iter()...\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "filter words starting with `a`",
                    matcher: RuleMatcher::Regex(r#"\.filter\s*\(\s*\|\w+\|\s*\w+\.starts_with\(\s*"a"\s*\)\s*\)"#),
                },
                ValidationRule {
                    label: "map to length",
                    matcher: RuleMatcher::Regex(r#"\.map\s*\(\s*\|\w+\|\s*\w+\.len\(\)\s*\)"#),
                },
                ValidationRule {
                    label: "collect",
                    matcher: RuleMatcher::Regex(r#"\.collect\s*<\s*Vec\s*<\s*usize\s*>\s*>\s*\(\s*\)|\.collect\s*\(\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn main() {\n    let words = vec![\"apple\", \"banana\", \"apricot\", \"cherry\"];\n    let a_lengths: Vec<usize> = words.into_iter()\n        .filter(|w| w.starts_with(\"a\"))\n        .map(|w| w.len())\n        .collect();\n}"),
            hints: &[
                "Filter: `.filter(|w| w.starts_with(\"a\"))`",
                "Map: `.map(|w| w.len())`",
                "Consume: `.collect()`",
            ],
        },
        success_message: "Masterful! You've unlocked the functional side of Rust.",
    },
];
