// ─── Curriculum: Trait Mastery ────────────────────────────────
//
// A guided learning path composed entirely from focus modules.
// No lessons are defined here — they are all borrowed from the
// lesson pool via module_groups.
//
// This replaces the old monolithic `rust_traits.rs` (32 inline
// lessons) with a composition manifest that references shared
// focus modules, eliminating all content duplication.

use crate::data::model::ModuleGroup;
use crate::data::lesson_pool;

pub const MODULE_GROUPS: &[ModuleGroup] = &[
    ModuleGroup {
        label: "Trait Objects & Dynamic Dispatch",
        modules: lesson_pool::TRAIT_OBJECTS,
    },
    ModuleGroup {
        label: "Object Safety",
        modules: lesson_pool::OBJECT_SAFETY,
    },
    ModuleGroup {
        label: "The Newtype Pattern",
        modules: lesson_pool::NEWTYPE,
    },
    ModuleGroup {
        label: "Orphan Rule & Coherence",
        modules: lesson_pool::ORPHAN_RULE,
    },
    ModuleGroup {
        label: "Associated Types vs Generics",
        modules: lesson_pool::ASSOCIATED_TYPES,
    },
    ModuleGroup {
        label: "Supertraits",
        modules: lesson_pool::SUPERTRAITS,
    },
    ModuleGroup {
        label: "impl Trait (APIT & RPIT)",
        modules: lesson_pool::IMPL_TRAIT,
    },
    ModuleGroup {
        label: "Blanket Implementations",
        modules: lesson_pool::BLANKET_IMPLS,
    },
    ModuleGroup {
        label: "Higher-Rank Trait Bounds",
        modules: lesson_pool::HRTB,
    },
    ModuleGroup {
        label: "Deref Coercion",
        modules: lesson_pool::DEREF,
    },
    ModuleGroup {
        label: "Display vs Debug",
        modules: lesson_pool::DISPLAY_DEBUG,
    },
    ModuleGroup {
        label: "The Drop Trait",
        modules: lesson_pool::DROP,
    },
    ModuleGroup {
        label: "The Default Trait",
        modules: lesson_pool::DEFAULT,
    },
    ModuleGroup {
        label: "Operator Overloading",
        modules: lesson_pool::OPERATOR_OVERLOADING,
    },
    ModuleGroup {
        label: "Sealed Traits",
        modules: lesson_pool::SEALED_TRAITS,
    },
];
