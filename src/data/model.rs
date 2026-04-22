use serde::{Serialize, Deserialize};
use crate::validation::ValidationSpec;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ModuleType {
    Concept,
    Practice,
}

#[derive(Clone, Debug)]
pub struct TutorialModule {
    pub id: &'static str,
    pub title: &'static str,
    pub module_type: ModuleType,
    pub content: &'static str,
    pub initial_code: &'static str,
    pub validation: ValidationSpec,
    pub success_message: &'static str,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Difficulty {
    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Beginner => "Beginner",
            Difficulty::Intermediate => "Intermediate",
            Difficulty::Advanced => "Advanced",
        }
    }

    pub fn badge_color(&self) -> &'static str {
        match self {
            Difficulty::Beginner => "bg-green-500/20 text-green-400 border-green-500/30",
            Difficulty::Intermediate => "bg-yellow-500/20 text-yellow-400 border-yellow-500/30",
            Difficulty::Advanced => "bg-red-500/20 text-red-400 border-red-500/30",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Course {
    pub id: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub accent: &'static str,
    pub modules: &'static [TutorialModule],
    pub difficulty: Difficulty,
    pub estimated_time: &'static str,
}

impl Course {
    pub fn lesson_count(&self) -> usize {
        self.modules.len()
    }
}
