// ─── Curriculum: Ownership & Lifetimes Deep Path ─────────────
//
// A guided path combining ownership-related focus modules into
// a comprehensive journey from borrowing to Pin & Unpin.

use crate::data::model::ModuleGroup;
use crate::data::lesson_pool;

pub const MODULE_GROUPS: &[ModuleGroup] = &[
    ModuleGroup {
        label: "Lifetime Elision Rules",
        modules: lesson_pool::LIFETIME_ELISION,
    },
    ModuleGroup {
        label: "The 'static Lifetime",
        modules: lesson_pool::STATIC_LIFETIME,
    },
    ModuleGroup {
        label: "Cow (Clone on Write)",
        modules: lesson_pool::COW,
    },
    ModuleGroup {
        label: "Pin & Unpin",
        modules: lesson_pool::PIN_UNPIN,
    },
];
