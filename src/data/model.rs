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

/// Distinguishes standalone focus courses from composed curriculum paths.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CourseKind {
    /// A bite-sized, self-contained topic (2-6 lessons).
    Focus,
    /// A guided learning path composed from focus module groups.
    Curriculum,
    /// A legacy inline course (all lessons defined in one file).
    Standalone,
}

impl CourseKind {
    pub fn label(&self) -> &'static str {
        match self {
            CourseKind::Focus => "Deep Dive",
            CourseKind::Curriculum => "Guided Path",
            CourseKind::Standalone => "Full Course",
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            CourseKind::Focus => "\u{1F3AF}",
            CourseKind::Curriculum => "\u{1F4D6}",
            CourseKind::Standalone => "\u{1F4DA}",
        }
    }
}

/// A section label for a group of modules inside a curriculum.
#[derive(Clone, Debug)]
pub struct ModuleGroup {
    pub label: &'static str,
    pub modules: &'static [TutorialModule],
}

#[derive(Clone, Debug)]
pub struct Course {
    pub id: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub accent: &'static str,
    /// Legacy: flat module array. Kept for backward compat during migration.
    pub modules: &'static [TutorialModule],
    /// Composed module groups (for curriculum courses).
    /// When non-empty, takes priority over `modules`.
    pub module_groups: &'static [ModuleGroup],
    pub difficulty: Difficulty,
    pub estimated_time: &'static str,
    pub kind: CourseKind,
    pub tags: &'static [&'static str],
}

impl Course {
    /// Returns all modules, preferring module_groups if non-empty.
    pub fn effective_modules(&self) -> Vec<&TutorialModule> {
        if !self.module_groups.is_empty() {
            self.module_groups
                .iter()
                .flat_map(|g| g.modules.iter())
                .collect()
        } else {
            self.modules.iter().collect()
        }
    }

    /// Total lesson count across all groups or the flat array.
    pub fn lesson_count(&self) -> usize {
        if !self.module_groups.is_empty() {
            self.module_groups.iter().map(|g| g.modules.len()).sum()
        } else {
            self.modules.len()
        }
    }

    /// Get a module by flat index, spanning across groups if needed.
    pub fn get_module(&self, idx: usize) -> Option<&TutorialModule> {
        if !self.module_groups.is_empty() {
            let mut offset = 0;
            for group in self.module_groups {
                if idx < offset + group.modules.len() {
                    return Some(&group.modules[idx - offset]);
                }
                offset += group.modules.len();
            }
            None
        } else {
            self.modules.get(idx)
        }
    }
}
