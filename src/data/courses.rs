use crate::data::model::Course;
use crate::data::rust_ownership::MODULES as RUST_OWNERSHIP_MODULES;
use crate::data::rust_traits::MODULES as RUST_TRAITS_MODULES;
use crate::data::rust_variables::MODULES as RUST_VARIABLES_MODULES;
use crate::data::rust_primitives::MODULES as RUST_PRIMITIVES_MODULES;
use crate::data::rust_control_flow::MODULES as RUST_CONTROL_FLOW_MODULES;
use crate::data::rust_functions::MODULES as RUST_FUNCTIONS_MODULES;
use crate::data::rust_strings::MODULES as RUST_STRINGS_MODULES;
use crate::data::rust_structs::MODULES as RUST_STRUCTS_MODULES;
use crate::data::rust_enums::MODULES as RUST_ENUMS_MODULES;
use crate::data::rust_collections::MODULES as RUST_COLLECTIONS_MODULES;
use crate::data::rust_errors::MODULES as RUST_ERRORS_MODULES;
use crate::data::rust_generics::MODULES as RUST_GENERICS_MODULES;
use crate::data::rust_modules::MODULES as RUST_MODULES_MODULES;
use crate::data::rust_iterators::MODULES as RUST_ITERATORS_MODULES;
use crate::data::rust_smart_pointers::MODULES as RUST_SMART_POINTERS_MODULES;
use crate::data::rust_concurrency::MODULES as RUST_CONCURRENCY_MODULES;
use crate::data::rust_async::MODULES as RUST_ASYNC_MODULES;
use crate::data::rust_unsafe::MODULES as RUST_UNSAFE_MODULES;
use crate::data::rust_macros::MODULES as RUST_MACROS_MODULES;

pub const COURSES: &[Course] = &[
    Course {
        id: "rust-traits",
        title: "Rust Trait Mastery",
        subtitle: "32-step journey from basic contracts to GATs and HRTBs",
        icon: "\u{1F980}",
        accent: "orange",
        modules: RUST_TRAITS_MODULES,
        difficulty: crate::data::model::Difficulty::Intermediate,
        estimated_time: "2-3 hours",
    },
    Course {
        id: "rust-modules",
        title: "Modules & Crates",
        subtitle: "Organize large codebases with modules, use paths, and external crates",
        icon: "\u{1F4E6}",
        accent: "sky",
        modules: RUST_MODULES_MODULES,
        difficulty: crate::data::model::Difficulty::Intermediate,
        estimated_time: "30 min",
    },
    Course {
        id: "rust-iterators",
        title: "Iterators & Closures",
        subtitle: "Write elegant, functional-style data processing pipelines with zero-cost abstractions",
        icon: "\u{1F504}",
        accent: "indigo",
        modules: RUST_ITERATORS_MODULES,
        difficulty: crate::data::model::Difficulty::Intermediate,
        estimated_time: "50 min",
    },
    Course {
        id: "rust-smart-pointers",
        title: "Smart Pointers",
        subtitle: "Master Box, Rc, and RefCell for advanced memory management and shared state",
        icon: "\u{1F9E0}",
        accent: "emerald",
        modules: RUST_SMART_POINTERS_MODULES,
        difficulty: crate::data::model::Difficulty::Advanced,
        estimated_time: "55 min",
    },
    Course {
        id: "rust-concurrency",
        title: "Concurrency Basics",
        subtitle: "Safe multithreading with std::thread, message passing (mpsc), and shared state",
        icon: "\u{26A1}",
        accent: "yellow",
        modules: RUST_CONCURRENCY_MODULES,
        difficulty: crate::data::model::Difficulty::Advanced,
        estimated_time: "60 min",
    },
    Course {
        id: "rust-async",
        title: "Async Rust",
        subtitle: "Concurrent execution with Futures, .await, and Executors (e.g. Tokio)",
        icon: "\u{23F1}\u{FE0F}",
        accent: "blue",
        modules: RUST_ASYNC_MODULES,
        difficulty: crate::data::model::Difficulty::Advanced,
        estimated_time: "50 min",
    },
    Course {
        id: "rust-unsafe",
        title: "Unsafe Rust",
        subtitle: "Escape the borrow checker to use raw pointers, mutable statics, and FFI",
        icon: "\u{1F525}",
        accent: "red",
        modules: RUST_UNSAFE_MODULES,
        difficulty: crate::data::model::Difficulty::Advanced,
        estimated_time: "30 min",
    },
    Course {
        id: "rust-macros",
        title: "Macros Fundamentals",
        subtitle: "Metaprogramming in Rust: write code that writes other code using macro_rules!",
        icon: "\u{2728}",
        accent: "fuchsia",
        modules: RUST_MACROS_MODULES,
        difficulty: crate::data::model::Difficulty::Advanced,
        estimated_time: "35 min",
    },
    Course {
        id: "rust-variables",
        title: "Rust Variables Mastery",
        subtitle: "10 guided lessons on bindings, mutability, constants, and patterns",
        icon: "\u{1F4E6}",
        accent: "emerald",
        modules: RUST_VARIABLES_MODULES,
        difficulty: crate::data::model::Difficulty::Beginner,
        estimated_time: "45-60 min",
    },
    Course {
        id: "rust-primitives",
        title: "Rust Primitive Types",
        subtitle: "Learn scalars (integers, floats, bools, chars) and compound types (arrays, tuples)",
        icon: "\u{1F9F1}",
        accent: "indigo",
        modules: RUST_PRIMITIVES_MODULES,
        difficulty: crate::data::model::Difficulty::Beginner,
        estimated_time: "20-30 min",
    },
    Course {
        id: "rust-functions",
        title: "Rust Functions & Expressions",
        subtitle: "Learn how to define functions, return values, and understand expressions vs statements",
        icon: "\u{1F524}",
        accent: "blue",
        modules: RUST_FUNCTIONS_MODULES,
        difficulty: crate::data::model::Difficulty::Beginner,
        estimated_time: "30 min",
    },
    Course {
        id: "rust-strings",
        title: "Rust Strings & Slices",
        subtitle: "Master text handling, from heap-allocated Strings to lightweight string slices",
        icon: "\u{1F4DD}",
        accent: "fuchsia",
        modules: RUST_STRINGS_MODULES,
        difficulty: crate::data::model::Difficulty::Beginner,
        estimated_time: "40 min",
    },
    Course {
        id: "rust-structs",
        title: "Rust Structs & Methods",
        subtitle: "Define custom data types and attach behavior to them using impl blocks",
        icon: "\u{1F4CF}",
        accent: "teal",
        modules: RUST_STRUCTS_MODULES,
        difficulty: crate::data::model::Difficulty::Beginner,
        estimated_time: "45 min",
    },
    Course {
        id: "rust-enums",
        title: "Rust Enums & Pattern Matching",
        subtitle: "Harness the power of algebraic data types, Option, Result, and match",
        icon: "\u{1F3F7}",
        accent: "emerald",
        modules: RUST_ENUMS_MODULES,
        difficulty: crate::data::model::Difficulty::Beginner,
        estimated_time: "50 min",
    },
    Course {
        id: "rust-control-flow",
        title: "Rust Control Flow",
        subtitle: "Master branching with if/else and repetition with loop, while, and for",
        icon: "\u{1F500}",
        accent: "orange",
        modules: RUST_CONTROL_FLOW_MODULES,
        difficulty: crate::data::model::Difficulty::Beginner,
        estimated_time: "30-45 min",
    },
    Course {
        id: "rust-ownership",
        title: "Rust Ownership Foundations",
        subtitle: "16 lessons on moves, borrowing, cloning, and slices",
        icon: "\u{1F512}",
        accent: "cyan",
        modules: RUST_OWNERSHIP_MODULES,
        difficulty: crate::data::model::Difficulty::Beginner,
        estimated_time: "75-90 min",
    },
    Course {
        id: "rust-collections",
        title: "Rust Collections",
        subtitle: "Store multiple values with Vec, HashMap, and HashSet",
        icon: "\u{1F4DA}",
        accent: "indigo",
        modules: RUST_COLLECTIONS_MODULES,
        difficulty: crate::data::model::Difficulty::Beginner,
        estimated_time: "40 min",
    },
    Course {
        id: "rust-errors",
        title: "Rust Error Handling",
        subtitle: "Gracefully handle failures with Result, panic!, and the ? operator",
        icon: "\u{26A0}\u{FE0F}",
        accent: "rose",
        modules: RUST_ERRORS_MODULES,
        difficulty: crate::data::model::Difficulty::Intermediate,
        estimated_time: "45 min",
    },
    Course {
        id: "rust-generics",
        title: "Generics & Lifetimes",
        subtitle: "Write reusable code and understand Rust's borrow checker annotations",
        icon: "\u{1F9EC}",
        accent: "purple",
        modules: RUST_GENERICS_MODULES,
        difficulty: crate::data::model::Difficulty::Intermediate,
        estimated_time: "60 min",
    },
];
