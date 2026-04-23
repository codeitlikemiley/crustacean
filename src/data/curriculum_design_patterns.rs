// ─── Curriculum: Design Patterns in Rust ─────────────────────
//
// A guided path combining idiomatic Rust design patterns.

use crate::data::model::ModuleGroup;
use crate::data::lesson_pool;

pub const MODULE_GROUPS: &[ModuleGroup] = &[
    ModuleGroup {
        label: "Newtype Pattern",
        modules: lesson_pool::NEWTYPE,
    },
    ModuleGroup {
        label: "Builder Pattern",
        modules: lesson_pool::BUILDER,
    },
    ModuleGroup {
        label: "Type State Pattern",
        modules: lesson_pool::TYPESTATE,
    },
    ModuleGroup {
        label: "State Machine with Enums",
        modules: lesson_pool::STATE_MACHINE,
    },
    ModuleGroup {
        label: "Phantom Types",
        modules: lesson_pool::PHANTOM_TYPES,
    },
    ModuleGroup {
        label: "Sealed Traits",
        modules: lesson_pool::SEALED_TRAITS,
    },
];
