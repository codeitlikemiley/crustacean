pub mod model;
pub mod courses;
mod rust_ownership;
mod rust_traits;
mod rust_variables;
mod rust_primitives;

pub use model::{Course, TutorialModule, ModuleType, Difficulty};
pub use courses::COURSES;
pub use rust_ownership::MODULES as RUST_OWNERSHIP_MODULES;
pub use rust_traits::MODULES as RUST_TRAIT_MASTERY_MODULES;
pub use rust_variables::MODULES as RUST_VARIABLES_MODULES;
pub use rust_primitives::MODULES as RUST_PRIMITIVES_MODULES;
