use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "smart-ptr-1-concept",
        title: "1. Concept: What is a Smart Pointer?",
        module_type: ModuleType::Concept,
        content: r#"
# Smart Pointers

A pointer is a variable that contains an address in memory. In Rust, the most common pointers are references (`&T`).

**Smart pointers** are data structures that act like a pointer but have additional metadata and capabilities. Most smart pointers *own* the data they point to.

### `Box<T>`
The most straightforward smart pointer is a `Box<T>`. It allows you to store data on the **heap** rather than the stack.

```rust
let b = Box::new(5);
println!("b = {}", b); // It implements Deref, so you can use it like a normal value
```

You use `Box` when:
- You have a type whose size can't be known at compile time.
- You have a large amount of data and want to transfer ownership without copying it.
        "#,
        initial_code: "// Study Box<T>, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "smart-ptr-2-practice",
        title: "2. Practice: Heap Allocation",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `Box::new`

### Task:
Create a variable `heap_num` that holds the integer `42` on the heap using `Box::new`.
        "#,
        initial_code: "fn main() {\n    // let heap_num = ...\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let heap_num =`",
                    matcher: RuleMatcher::Regex(r#"let\s+heap_num\s*="#),
                },
                ValidationRule {
                    label: "use `Box::new`",
                    matcher: RuleMatcher::Contains("Box::new(42)"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn main() {\n    let heap_num = Box::new(42);\n}"),
            hints: &[
                "The syntax is `let heap_num = Box::new(42);`",
            ],
        },
        success_message: "Great! The value is now safely stored on the heap.",
    },
    TutorialModule {
        id: "smart-ptr-3-concept",
        title: "3. Concept: Recursive Types",
        module_type: ModuleType::Concept,
        content: r#"
# Recursive Types and `Box`

At compile time, Rust needs to know exactly how much space a type takes up.

If you have a recursive type (like a List node that contains another List node), its theoretical size is infinite!

```rust
// This will NOT compile!
enum List {
    Cons(i32, List), // "recursive type has infinite size"
    Nil,
}
```

Because a `Box` is just a pointer, it has a fixed size. We can use it to break the infinite recursion:
```rust
enum List {
    Cons(i32, Box<List>), // This works! The Box has a known size.
    Nil,
}
```
        "#,
        initial_code: "// Understand why Box enables recursive types, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "smart-ptr-4-practice",
        title: "4. Practice: Build a Node",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Fix the Node

### Task:
Fix the `Node` struct so it compiles. The `next` field should hold an `Option` containing a heap-allocated `Node`.
Wrap the `Node` in a `Box`.
        "#,
        initial_code: "struct Node {\n    value: i32,\n    next: Option<Node>, // Fix this line!\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "wrap `Node` in `Box`",
                    matcher: RuleMatcher::Regex(r#"next\s*:\s*Option\s*<\s*Box\s*<\s*Node\s*>\s*>"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("struct Node {\n    value: i32,\n    next: Option<Box<Node>>,\n}"),
            hints: &[
                "Change `Option<Node>` to `Option<Box<Node>>`.",
            ],
        },
        success_message: "Perfect! You've successfully defined a recursive data structure.",
    },
    TutorialModule {
        id: "smart-ptr-5-concept",
        title: "5. Concept: `Rc<T>` for Shared Ownership",
        module_type: ModuleType::Concept,
        content: r#"
# Reference Counting (`Rc`)

Rust's strict ownership rules state that a value can only have one owner. 

But sometimes, data *needs* multiple owners (e.g., in a graph, a node might have multiple edges pointing to it).

`Rc<T>` (Reference Counted) enables multiple ownership. It keeps track of the number of references to a value, which determines whether or not the value is still in use.

```rust
use std::rc::Rc;

let a = Rc::new(String::from("hello"));
// Rc::clone doesn't deep copy the string; it just increments the reference count!
let b = Rc::clone(&a); 
let c = Rc::clone(&a);

println!("Count: {}", Rc::strong_count(&a)); // 3
```

*Note: `Rc<T>` is strictly for single-threaded scenarios.*
        "#,
        initial_code: "// Study Rc<T>, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "smart-ptr-6-practice",
        title: "6. Practice: Share a String",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Multiple Owners

### Task:
Create `owner1` which is an `Rc` containing the string `"Shared"`.
Then create `owner2` which clones `owner1` using `Rc::clone()`.
        "#,
        initial_code: "use std::rc::Rc;\n\nfn main() {\n    // let owner1 = ...\n    // let owner2 = ...\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "create `owner1` with `Rc::new`",
                    matcher: RuleMatcher::Regex(r#"let\s+owner1\s*=\s*Rc::new\(\s*(?:String::from\("Shared"\)|"Shared"\.to_string\(\))\s*\)"#),
                },
                ValidationRule {
                    label: "create `owner2` with `Rc::clone`",
                    matcher: RuleMatcher::Regex(r#"let\s+owner2\s*=\s*Rc::clone\(\s*&owner1\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::rc::Rc;\n\nfn main() {\n    let owner1 = Rc::new(String::from(\"Shared\"));\n    let owner2 = Rc::clone(&owner1);\n}"),
            hints: &[
                "owner1: `Rc::new(String::from(\"Shared\"))`",
                "owner2: `Rc::clone(&owner1)`",
            ],
        },
        success_message: "Awesome! You safely bypassed the single-owner rule.",
    },
    TutorialModule {
        id: "smart-ptr-7-concept",
        title: "7. Concept: `RefCell<T>`",
        module_type: ModuleType::Concept,
        content: r#"
# Interior Mutability

Normally, if you have an immutable reference (`&T`), you cannot mutate the data it points to.

`RefCell<T>` provides **interior mutability**. It allows you to mutate data even when there are immutable references to that data! It achieves this by checking borrowing rules at *runtime* instead of compile time.

```rust
use std::cell::RefCell;

let value = RefCell::new(5);

// We borrow mutably at runtime
*value.borrow_mut() += 10;

println!("{}", value.borrow()); // 15
```

If you break the borrowing rules at runtime (e.g., trying to get two mutable borrows at the same time), the program will `panic!`.
        "#,
        initial_code: "// Study RefCell, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "smart-ptr-8-practice",
        title: "8. Practice: Mutate via `RefCell`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Interior Mutability

### Task:
Notice `data` is NOT declared as `mut`.
Use `data.borrow_mut()` to push the string `"World"` into the vector.
        "#,
        initial_code: "use std::cell::RefCell;\n\nfn main() {\n    let data = RefCell::new(vec![\"Hello\"]);\n    // Mutate the vector inside the RefCell here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "call `borrow_mut()`",
                    matcher: RuleMatcher::Contains("data.borrow_mut()"),
                },
                ValidationRule {
                    label: "push \"World\"",
                    matcher: RuleMatcher::Contains(".push(\"World\")"),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not make data mutable",
                    matcher: RuleMatcher::Regex(r#"let\s+mut\s+data"#),
                },
            ],
            canonical_solution: Some("use std::cell::RefCell;\n\nfn main() {\n    let data = RefCell::new(vec![\"Hello\"]);\n    data.borrow_mut().push(\"World\");\n}"),
            hints: &[
                "The syntax is `data.borrow_mut().push(\"World\");`",
            ],
        },
        success_message: "Excellent! You mutated data through an immutable binding.",
    },
    TutorialModule {
        id: "smart-ptr-9-concept",
        title: "9. Concept: Combining `Rc` and `RefCell`",
        module_type: ModuleType::Concept,
        content: r#"
# The Ultimate Combo: `Rc<RefCell<T>>`

`Rc` allows multiple owners, but it only allows *immutable* borrowing.
`RefCell` allows mutable borrowing, but only has *one* owner.

If you combine them, you get a value that can have **multiple owners and can be mutated by any of them!**

```rust
use std::rc::Rc;
use std::cell::RefCell;

// Multiple owners can mutate the shared state
let shared_state = Rc::new(RefCell::new(0));

let owner1 = Rc::clone(&shared_state);
let owner2 = Rc::clone(&shared_state);

*owner1.borrow_mut() += 1;
*owner2.borrow_mut() += 5;

println!("{}", shared_state.borrow()); // 6
```
        "#,
        initial_code: "// Marvel at Rc<RefCell<T>>, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "smart-ptr-10-practice",
        title: "10. Practice: Shared Mutable State",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `Rc<RefCell<String>>`

### Task:
Wrap the `String` in an `Rc<RefCell<String>>`. 
Then use `Rc::clone` to create a second owner.
Finally, use `borrow_mut()` on the second owner to push `", world!"` onto the string.
        "#,
        initial_code: "use std::rc::Rc;\nuse std::cell::RefCell;\n\nfn main() {\n    // 1. Create `shared_text` containing \"Hello\"\n    \n    // 2. Clone it to `second_owner`\n    \n    // 3. Mutate it via `second_owner`\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "create `shared_text`",
                    matcher: RuleMatcher::Regex(r#"let\s+shared_text\s*=\s*Rc::new\(\s*RefCell::new\(\s*(?:String::from\("Hello"\)|"Hello"\.to_string\(\))\s*\)\s*\)"#),
                },
                ValidationRule {
                    label: "clone to `second_owner`",
                    matcher: RuleMatcher::Regex(r#"let\s+second_owner\s*=\s*Rc::clone\(\s*&shared_text\s*\)"#),
                },
                ValidationRule {
                    label: "mutate via `second_owner`",
                    matcher: RuleMatcher::Contains("second_owner.borrow_mut().push_str(\", world!\")"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::rc::Rc;\nuse std::cell::RefCell;\n\nfn main() {\n    let shared_text = Rc::new(RefCell::new(String::from(\"Hello\")));\n    let second_owner = Rc::clone(&shared_text);\n    second_owner.borrow_mut().push_str(\", world!\");\n}"),
            hints: &[
                "1. `Rc::new(RefCell::new(String::from(\"Hello\")));`",
                "2. `Rc::clone(&shared_text);`",
                "3. `second_owner.borrow_mut().push_str(\", world!\");`",
            ],
        },
        success_message: "Congratulations! You've mastered advanced memory management in Rust.",
    },
];
