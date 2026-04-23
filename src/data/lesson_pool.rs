// ─── Lesson Pool ─────────────────────────────────────────────
//
// This module re-exports every focus course's MODULES array
// as named constants. Curriculum courses reference these
// constants to compose learning paths WITHOUT duplicating
// any lesson content.
//
// To add a new topic to the pool:
// 1. Create `src/data/focus_<topic>.rs` using the rust-tuts skill.
// 2. Register it in `mod.rs` (mod + pub use).
// 3. Add a `pub use` line here.
//
// Curriculums then reference e.g. `pool::TRAIT_OBJECTS` in their
// module_groups array.

pub use crate::data::focus_trait_objects::MODULES as TRAIT_OBJECTS;
pub use crate::data::focus_from_into::MODULES as FROM_INTO;
pub use crate::data::focus_newtype::MODULES as NEWTYPE;
pub use crate::data::focus_cow::MODULES as COW;
pub use crate::data::focus_builder::MODULES as BUILDER;
pub use crate::data::focus_deref::MODULES as DEREF;
pub use crate::data::focus_fn_traits::MODULES as FN_TRAITS;
pub use crate::data::focus_object_safety::MODULES as OBJECT_SAFETY;
pub use crate::data::focus_blanket_impls::MODULES as BLANKET_IMPLS;
pub use crate::data::focus_hrtb::MODULES as HRTB;
pub use crate::data::focus_associated_types::MODULES as ASSOCIATED_TYPES;
pub use crate::data::focus_typestate::MODULES as TYPESTATE;
pub use crate::data::focus_supertraits::MODULES as SUPERTRAITS;
pub use crate::data::focus_drop::MODULES as DROP;
pub use crate::data::focus_default::MODULES as DEFAULT;
pub use crate::data::focus_orphan_rule::MODULES as ORPHAN_RULE;
pub use crate::data::focus_pin_unpin::MODULES as PIN_UNPIN;
pub use crate::data::focus_lifetime_elision::MODULES as LIFETIME_ELISION;
pub use crate::data::focus_state_machine::MODULES as STATE_MACHINE;
pub use crate::data::focus_sealed_traits::MODULES as SEALED_TRAITS;
pub use crate::data::focus_phantom_types::MODULES as PHANTOM_TYPES;
pub use crate::data::focus_never_type::MODULES as NEVER_TYPE;
pub use crate::data::focus_display_debug::MODULES as DISPLAY_DEBUG;
pub use crate::data::focus_operator_overloading::MODULES as OPERATOR_OVERLOADING;
pub use crate::data::focus_impl_trait::MODULES as IMPL_TRAIT;
pub use crate::data::focus_static_lifetime::MODULES as STATIC_LIFETIME;
pub use crate::data::focus_into_iterator::MODULES as INTO_ITERATOR;
pub use crate::data::focus_thiserror::MODULES as THISERROR;
pub use crate::data::focus_gats::MODULES as GATS;
pub use crate::data::focus_variance::MODULES as VARIANCE;
pub use crate::data::focus_futures::MODULES as FUTURES;
pub use crate::data::focus_safe_unsafe::MODULES as SAFE_UNSAFE;
pub use crate::data::focus_macro_rules::MODULES as MACRO_RULES;
pub use crate::data::focus_derive_macros::MODULES as DERIVE_MACROS;
pub use crate::data::focus_select::MODULES as SELECT;
pub use crate::data::focus_atomics::MODULES as ATOMICS;
pub use crate::data::focus_maybe_uninit::MODULES as MAYBE_UNINIT;
pub use crate::data::focus_transmute::MODULES as TRANSMUTE;
