use crate::data::model::Course;
use crate::data::rust_ownership::MODULES as RUST_OWNERSHIP_MODULES;
use crate::data::rust_traits::MODULES as RUST_TRAITS_MODULES;
use crate::data::rust_variables::MODULES as RUST_VARIABLES_MODULES;

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
        id: "rust-ownership",
        title: "Rust Ownership Foundations",
        subtitle: "16 lessons on moves, borrowing, cloning, and slices",
        icon: "\u{1F512}",
        accent: "cyan",
        modules: RUST_OWNERSHIP_MODULES,
        difficulty: crate::data::model::Difficulty::Beginner,
        estimated_time: "75-90 min",
    },
];
