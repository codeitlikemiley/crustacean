use leptos::prelude::{RwSignal, Effect, Get, GetUntracked, Set, Update, set_timeout};
use crate::data::{COURSES, Course, ModuleType};
use crate::validation::{validate_module, DiffKind};
use web_sys::{window, Storage};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

const STORAGE_KEY: &str = "tuts_platform_v1";
const LEGACY_KEY: &str = "tuts_rust_trait_mastery";

// ---------- View routing ----------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppView {
    Catalog,
    CourseDetail { course_id: String },
    Lesson { course_id: String },
}

// ---------- Persistence types ----------

#[derive(Clone, Serialize, Deserialize)]
pub struct PerCourseProgress {
    pub current_step: usize,
    pub codes: Vec<String>,
    pub completed: Vec<bool>,
    pub skipped: Vec<bool>,
    pub started: bool,
}

impl Default for PerCourseProgress {
    fn default() -> Self {
        Self {
            current_step: 0,
            codes: vec![],
            completed: vec![],
            skipped: vec![],
            started: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct SavedPlatformState {
    current_view: AppView,
    selected_course: Option<String>,
    selected_lesson: Option<usize>,
    courses: Vec<(String, PerCourseProgress)>,
}

fn default_saved_state() -> SavedPlatformState {
    SavedPlatformState {
        current_view: AppView::Catalog,
        selected_course: None,
        selected_lesson: None,
        courses: vec![],
    }
}

fn get_storage() -> Option<Storage> {
    window().and_then(|w| w.local_storage().ok().flatten())
}

fn migrate_legacy() -> Vec<(String, PerCourseProgress)> {
    let mut courses = vec![];
    if let Some(storage) = get_storage() {
        if let Ok(Some(json)) = storage.get_item(LEGACY_KEY) {
            if let Ok(legacy) = serde_json::from_str::<LegacyState>(&json) {
                let module_count = COURSES
                    .iter()
                    .find(|c| c.id == "rust-traits")
                    .map(|c| c.modules.len())
                    .unwrap_or(32);

                let mut codes = legacy.codes;
                // Pad codes to match module count
                while codes.len() < module_count {
                    if let Some(course) = COURSES.iter().find(|c| c.id == "rust-traits") {
                        let idx = codes.len();
                        if idx < course.modules.len() {
                            codes.push(course.modules[idx].initial_code.to_string());
                        }
                    } else {
                        break;
                    }
                }

                let progress = PerCourseProgress {
                    current_step: legacy.current_step.min(module_count.saturating_sub(1)),
                    codes,
                    completed: {
                        let mut c = legacy.completed;
                        while c.len() < module_count {
                            c.push(false);
                        }
                        c
                    },
                    skipped: {
                        let mut s = legacy.skipped;
                        while s.len() < module_count {
                            s.push(false);
                        }
                        s
                    },
                    started: true,
                };
                courses.push(("rust-traits".to_string(), progress));

                // Clean up old key
                let _ = storage.remove_item(LEGACY_KEY);
            }
        }
    }
    courses
}

#[derive(Deserialize)]
struct LegacyState {
    current_step: usize,
    codes: Vec<String>,
    completed: Vec<bool>,
    skipped: Vec<bool>,
}

fn load_state() -> SavedPlatformState {
    if let Some(storage) = get_storage() {
        if let Ok(Some(json)) = storage.get_item(STORAGE_KEY) {
            if let Ok(state) = serde_json::from_str(&json) {
                return state;
            }
        }
    }

    // Try migrating from legacy key
    let migrated_courses = migrate_legacy();
    if !migrated_courses.is_empty() {
        return SavedPlatformState {
            current_view: AppView::Lesson {
                course_id: migrated_courses[0].0.clone(),
            },
            selected_course: Some(migrated_courses[0].0.clone()),
            selected_lesson: None,
            courses: migrated_courses,
        };
    }

    default_saved_state()
}

fn save_state(state: &SavedPlatformState) {
    if let Some(storage) = get_storage() {
        if let Ok(json) = serde_json::to_string(state) {
            let _ = storage.set_item(STORAGE_KEY, &json);
        }
    }
}

// ---------- Public AppState ----------

#[derive(Clone, Copy)]
pub struct AppState {
    pub current_view: RwSignal<AppView>,
    pub selected_course: RwSignal<Option<String>>,
    pub selected_lesson: RwSignal<Option<usize>>,
    pub highlight_active: RwSignal<bool>,
    pub course_progress: RwSignal<Vec<(String, PerCourseProgress)>>,
    // Lesson-scoped signals (derived from course_progress + selected_course)
    pub current_step: RwSignal<usize>,
    pub code: RwSignal<String>,
    pub completed: RwSignal<Vec<bool>>,
    pub skipped: RwSignal<Vec<bool>>,
    pub is_success: RwSignal<bool>,
    pub terminal_lines: RwSignal<Vec<TerminalLine>>,
    // Per-step diagnostics storage: key = "course_id:step_index"
    pub step_diagnostics: RwSignal<HashMap<String, Vec<TerminalLine>>>,
    // Command palette visibility
    pub show_command_palette: RwSignal<bool>,
    pub show_step_picker: RwSignal<bool>,
}

impl AppState {
    pub fn init() -> Self {
        let saved = load_state();

        let current_view = RwSignal::new(saved.current_view);
        let selected_course = RwSignal::new(saved.selected_course);
        let selected_lesson = RwSignal::new(saved.selected_lesson);
        let highlight_active = RwSignal::new(false);
        let course_progress = RwSignal::new(saved.courses);

        // Derive lesson state from selected course
        let (step, code, completed, skipped) = Self::load_lesson_state(
            &selected_course,
            &current_view,
            &course_progress,
        );

        let current_step = RwSignal::new(step);
        let code_signal = RwSignal::new(code);
        let completed_signal = RwSignal::new(completed);
        let skipped_signal = RwSignal::new(skipped);
        let is_success = RwSignal::new(false);
        let terminal_lines = RwSignal::new(vec![TerminalLine {
            text: "System: Rust Mastery Initialized. Welcome.".to_string(),
            line_type: LineType::Info,
        }]);
        let step_diagnostics = RwSignal::new(HashMap::new());
        let show_command_palette = RwSignal::new(false);
        let show_step_picker = RwSignal::new(false);

        // Sync lesson state back to course_progress on changes
        let cp = course_progress;
        let sc = selected_course;
        let cv = current_view;
        let sl = current_step;
        let cc = code_signal.clone();
        let cs = completed_signal.clone();
        let sk = skipped_signal.clone();

        let _ = Effect::new(move |_| {
            let _ = sl.get();
            let _ = cs.get();
            let _ = sk.get();
            let _ = cc.get();
            Self::sync_lesson_state(
                &sc,
                &cv,
                &cp,
                current_step,
                &cc,
                &completed_signal,
                &skipped_signal,
            );
        });

        let _ = Effect::new(move |_| {
            let _ = current_view.get();
            let _ = selected_course.get();
            let _ = selected_lesson.get();
            let _ = course_progress.get();
            save_state(&SavedPlatformState {
                current_view: current_view.get_untracked(),
                selected_course: selected_course.get_untracked(),
                selected_lesson: selected_lesson.get_untracked(),
                courses: course_progress.get_untracked(),
            });
        });

        AppState {
            current_view,
            selected_course,
            selected_lesson,
            course_progress,
            current_step,
            code: code_signal,
            completed: completed_signal,
            skipped: skipped_signal,
            is_success,
            terminal_lines,
            step_diagnostics,
            show_command_palette,
            show_step_picker,
            highlight_active,
        }
    }

    fn load_lesson_state(
        selected_course: &RwSignal<Option<String>>,
        current_view: &RwSignal<AppView>,
        course_progress: &RwSignal<Vec<(String, PerCourseProgress)>>,
    ) -> (usize, String, Vec<bool>, Vec<bool>) {
        let view = current_view.get_untracked();
        let course_id = match view {
            AppView::Lesson { ref course_id } => Some(course_id.clone()),
            AppView::CourseDetail { ref course_id } => Some(course_id.clone()),
            AppView::Catalog => None,
        };

        let course_id = course_id.or_else(|| selected_course.get_untracked());

        if let Some(cid) = course_id {
            let progress = course_progress.get_untracked();
            if let Some((_, p)) = progress.iter().find(|(id, _)| id == &cid) {
                let course = COURSES.iter().find(|c| c.id == cid.as_str());
                let step = p.current_step.min(
                    course.map(|c| c.modules.len()).unwrap_or(1).saturating_sub(1),
                );
                let code = p.codes.get(step).cloned().unwrap_or_else(|| {
                    course
                        .map(|c| c.modules[step].initial_code.to_string())
                        .unwrap_or_default()
                });
                return (step, code, p.completed.clone(), p.skipped.clone());
            }
        }
        (0, String::new(), vec![], vec![])
    }

    fn sync_lesson_state(
        selected_course: &RwSignal<Option<String>>,
        current_view: &RwSignal<AppView>,
        course_progress: &RwSignal<Vec<(String, PerCourseProgress)>>,
        current_step: RwSignal<usize>,
        code: &RwSignal<String>,
        completed: &RwSignal<Vec<bool>>,
        skipped: &RwSignal<Vec<bool>>,
    ) {
        let view = current_view.get_untracked();
        let course_id = match view {
            AppView::Lesson { ref course_id } => Some(course_id.clone()),
            _ => selected_course.get_untracked(),
        };

        if let Some(cid) = course_id {
            let step = current_step.get_untracked();
            let code_val = code.get_untracked();
            let comp = completed.get_untracked();
            let skip = skipped.get_untracked();

            course_progress.update(|courses| {
                if let Some((_, p)) = courses.iter_mut().find(|(id, _)| id == &cid) {
                    p.current_step = step;
                    while p.codes.len() <= step {
                        if let Some(course) = COURSES.iter().find(|c| c.id == cid.as_str()) {
                            let idx = p.codes.len();
                            if idx < course.modules.len() {
                                p.codes.push(course.modules[idx].initial_code.to_string());
                            } else {
                                p.codes.push(String::new());
                            }
                        }
                    }
                    p.codes[step] = code_val;
                    p.completed = comp;
                    p.skipped = skip;
                    p.started = true;
                } else {
                    // Create new progress entry
                    if let Some(course) = COURSES.iter().find(|c| c.id == cid.as_str()) {
                        let mut codes: Vec<String> =
                            course.modules.iter().map(|m| m.initial_code.to_string()).collect();
                        if step < codes.len() {
                            codes[step] = code_val;
                        }
                        courses.push((
                            cid,
                            PerCourseProgress {
                                current_step: step,
                                codes,
                                completed: comp,
                                skipped: skip,
                                started: true,
                            },
                        ));
                    }
                }
            });
        }
    }

    pub fn get_course(&self, id: &str) -> Option<&'static Course> {
        COURSES.iter().find(|c| c.id == id)
    }

    pub fn get_courses(&self) -> &'static [Course] {
        COURSES
    }

    pub fn get_course_progress(&self, course_id: &str) -> (usize, usize) {
        let progress = self.course_progress.get();
        if let Some((_, p)) = progress.iter().find(|(id, _)| id == course_id) {
            let done = p.completed.iter().filter(|&&b| b).count();
            let total = p.completed.len();
            return (done, total);
        }
        if let Some(course) = self.get_course(course_id) {
            (0, course.modules.len())
        } else {
            (0, 0)
        }
    }

    pub fn get_course_progress_pct(&self, course_id: &str) -> usize {
        let (done, total) = self.get_course_progress(course_id);
        if total == 0 {
            return 0;
        }
        ((done as f64) / (total as f64) * 100.0).round() as usize
    }

    // Navigation
    pub fn go_to_catalog(&self) {
        // Save current step diagnostics before leaving lesson
        self.save_diagnostics_for_current_step();
        self.current_view.set(AppView::Catalog);
        self.selected_course.set(None);
        self.selected_lesson.set(None);
        self.is_success.set(false);
        self.show_command_palette.set(false);
        self.show_step_picker.set(false);
    }

    pub fn go_to_course_detail(&self, course_id: String) {
        self.current_view.set(AppView::CourseDetail {
            course_id: course_id.clone(),
        });
        self.selected_course.set(Some(course_id.clone()));
        
        // Find prioritized lesson (first incomplete)
        let prio = self.get_prioritized_lesson_index(&course_id);
        self.selected_lesson.set(Some(prio));
        
        self.trigger_highlight();
        self.is_success.set(false);
    }

    pub fn get_prioritized_lesson_index(&self, course_id: &str) -> usize {
        if let Some(course) = self.get_course(course_id) {
            let progress = self.course_progress.get_untracked();
            if let Some((_, p)) = progress.iter().find(|(id, _)| id == course_id) {
                // Find first incomplete (not completed)
                for (idx, &is_comp) in p.completed.iter().enumerate() {
                    if !is_comp {
                        return idx;
                    }
                }
                // If all completed, return the last one
                return course.modules.len().saturating_sub(1);
            }
        }
        0
    }

    pub fn resume_prioritized_course(&self, course_id: &str) {
        let prio = self.get_prioritized_lesson_index(course_id);
        self.start_course(course_id);
        self.navigate_to(prio);
    }

    pub fn trigger_highlight(&self) {
        self.highlight_active.set(true);
        let ha = self.highlight_active;
        // Fade after 2 seconds
        set_timeout(move || ha.set(false), std::time::Duration::from_millis(2000));
    }

    pub fn select_next_lesson(&self) {
        if let AppView::CourseDetail { ref course_id } = self.current_view.get_untracked() {
            if let Some(course) = self.get_course(course_id) {
                let current = self.selected_lesson.get_untracked().unwrap_or(0);
                if current < course.modules.len() - 1 {
                    self.selected_lesson.set(Some(current + 1));
                    self.trigger_highlight();
                }
            }
        }
    }

    pub fn select_prev_lesson(&self) {
        if let AppView::CourseDetail { .. } = self.current_view.get_untracked() {
            let current = self.selected_lesson.get_untracked().unwrap_or(0);
            if current > 0 {
                self.selected_lesson.set(Some(current - 1));
                self.trigger_highlight();
            }
        }
    }

    pub fn open_selected_lesson(&self) {
        if let AppView::CourseDetail { ref course_id } = self.current_view.get_untracked() {
            if let Some(idx) = self.selected_lesson.get_untracked() {
                let cid = course_id.clone();
                self.start_course(&cid);
                self.navigate_to(idx);
            }
        }
    }

    pub fn start_course(&self, course_id: &str) {
        // Initialize progress if not exists
        self.ensure_course_progress(course_id);

        self.current_view.set(AppView::Lesson {
            course_id: course_id.to_string(),
        });
        self.selected_course.set(Some(course_id.to_string()));
        self.selected_lesson.set(None);

        let progress = self.course_progress.get();
        let start_step = if let Some((_, p)) = progress.iter().find(|(id, _)| id == course_id) {
            p.current_step
        } else {
            0
        };

        if let Some(course) = self.get_course(course_id) {
            let step = start_step.min(course.modules.len().saturating_sub(1));
            self.current_step.set(step);
            let progress = self.course_progress.get();
            if let Some((_, p)) = progress.iter().find(|(id, _)| id == course_id) {
                let code = p
                    .codes
                    .get(step)
                    .cloned()
                    .unwrap_or_else(|| course.modules[step].initial_code.to_string());
                self.code.set(code);
            }
        }
        self.completed.set(vec![
            false;
            self.get_course(course_id)
                .map(|c| c.modules.len())
                .unwrap_or(0)
        ]);
        self.skipped.set(vec![
            false;
            self.get_course(course_id)
                .map(|c| c.modules.len())
                .unwrap_or(0)
        ]);
        // Restore completed/skipped from persisted state
        if let Some(course) = self.get_course(course_id) {
            let progress = self.course_progress.get();
            if let Some((_, p)) = progress.iter().find(|(id, _)| id == course_id) {
                self.completed.set(p.completed.clone());
                self.skipped.set(p.skipped.clone());
                self.current_step
                    .set(p.current_step.min(course.modules.len().saturating_sub(1)));
            }
        }
        // Load diagnostics for the starting step
        self.load_diagnostics_for_step(course_id, self.current_step.get());
        self.is_success.set(false);
    }

    pub fn resume_course(&self, course_id: &str) {
        self.start_course(course_id);
    }

    pub fn ensure_course_progress(&self, course_id: &str) {
        let progress = self.course_progress.get();
        if !progress.iter().any(|(id, _)| id == course_id) {
            if let Some(course) = self.get_course(course_id) {
                let new_progress = PerCourseProgress {
                    current_step: 0,
                    codes: course
                        .modules
                        .iter()
                        .map(|m| m.initial_code.to_string())
                        .collect(),
                    completed: vec![false; course.modules.len()],
                    skipped: vec![false; course.modules.len()],
                    started: false,
                };
                self.course_progress
                    .update(|c| c.push((course_id.to_string(), new_progress)));
            }
        }
    }

    pub fn go_to_step(&self, step: usize) {
        let course_id = match self.current_view.get() {
            AppView::Lesson { ref course_id } => course_id.clone(),
            AppView::CourseDetail { ref course_id } => course_id.clone(),
            _ => return,
        };

        // Save current step diagnostics before switching
        self.save_diagnostics_for_current_step();

        if let Some(course) = self.get_course(&course_id) {
            let max_step = course.modules.len().saturating_sub(1);
            let target_step = step.min(max_step);
            self.current_step.set(target_step);
            
            // Load the code for this step
            let progress = self.course_progress.get();
            if let Some((_, p)) = progress.iter().find(|(id, _)| id == &course_id) {
                let saved_code = p
                    .codes
                    .get(target_step)
                    .cloned()
                    .unwrap_or_else(|| course.modules[target_step].initial_code.to_string());
                self.code.set(saved_code);
            }
            
            // Load diagnostics for the target step
            self.load_diagnostics_for_step(&course_id, target_step);
        }
        self.is_success.set(false);
    }

    // Lesson navigation
    pub fn navigate_to(&self, step: usize) {
        let course_id = match self.current_view.get() {
            AppView::Lesson { ref course_id } => course_id.clone(),
            _ => return,
        };

        if let Some(course) = self.get_course(&course_id) {
            if step >= course.modules.len() {
                return;
            }
            // Save current step diagnostics before switching
            self.save_diagnostics_for_current_step();
            self.save_current_code();
            self.current_step.set(step);
            // Get saved code from course_progress
            let progress = self.course_progress.get();
            if let Some((_, p)) = progress.iter().find(|(id, _)| id == &course_id) {
                let saved = p
                    .codes
                    .get(step)
                    .cloned()
                    .unwrap_or_else(|| course.modules[step].initial_code.to_string());
                self.code.set(saved);
            }
            // Load diagnostics for the target step
            self.load_diagnostics_for_step(&course_id, step);
            self.is_success.set(false);
        }
    }

    pub fn save_current_code(&self) {
        let idx = self.current_step.get();
        let val = self.code.get();
        let course_id = match self.current_view.get() {
            AppView::Lesson { ref course_id } => course_id.clone(),
            _ => return,
        };

        self.course_progress.update(|courses| {
            if let Some((_, p)) = courses.iter_mut().find(|(id, _)| id == &course_id) {
                while p.codes.len() <= idx {
                    if let Some(course) = COURSES.iter().find(|c| c.id == course_id.as_str()) {
                        let i = p.codes.len();
                        if i < course.modules.len() {
                            p.codes.push(course.modules[i].initial_code.to_string());
                        } else {
                            p.codes.push(String::new());
                        }
                    }
                }
                p.codes[idx] = val;
            }
        });
    }

    pub fn set_code(&self, val: String) {
        self.code.set(val);
    }

    pub fn next_step(&self) {
        let course_id = match self.current_view.get() {
            AppView::Lesson { ref course_id } => course_id.clone(),
            _ => return,
        };

        if let Some(course) = self.get_course(&course_id) {
            let current = self.current_step.get();
            if current < course.modules.len() - 1 {
                // Save current step diagnostics before switching
                self.save_diagnostics_for_current_step();
                self.save_current_code();

                // If current module is a Concept, mark it completed when moving next
                if matches!(course.modules[current].module_type, ModuleType::Concept) {
                    self.completed.update(|c| {
                        if current < c.len() {
                            c[current] = true;
                        }
                    });
                } else {
                    self.skipped.update(|s| s[current] = true);
                }

                let next = current + 1;
                self.current_step.set(next);
                let progress = self.course_progress.get();
                if let Some((_, p)) = progress.iter().find(|(id, _)| id == &course_id) {
                    let saved = p
                        .codes
                        .get(next)
                        .cloned()
                        .unwrap_or_else(|| course.modules[next].initial_code.to_string());
                    self.code.set(saved);
                }
                // Load diagnostics for the next step
                self.load_diagnostics_for_step(&course_id, next);
                self.is_success.set(false);
            }
        }
    }

    pub fn prev_step(&self) {
        let current = self.current_step.get();
        if current > 0 {
            let course_id = match self.current_view.get() {
                AppView::Lesson { ref course_id } => course_id.clone(),
                _ => return,
            };
            // Save current step diagnostics before switching
            self.save_diagnostics_for_current_step();
            self.save_current_code();
            let prev = current - 1;
            self.current_step.set(prev);
            if let Some(course) = self.get_course(&course_id) {
                let progress = self.course_progress.get();
                if let Some((_, p)) = progress.iter().find(|(id, _)| id == &course_id) {
                    let saved = p
                        .codes
                        .get(prev)
                        .cloned()
                        .unwrap_or_else(|| course.modules[prev].initial_code.to_string());
                    self.code.set(saved);
                }
                // Load diagnostics for the previous step
                self.load_diagnostics_for_step(&course_id, prev);
            }
            self.is_success.set(false);
        }
    }

    pub fn log(&self, msg: &str, line_type: LineType) {
        let prefix = match line_type {
            LineType::Error => "\u{274C} ",
            LineType::Success => "\u{2705} ",
            LineType::Info => "> ",
        };

        let mut lines_to_add = Vec::new();
        for (i, raw_line) in msg.lines().enumerate() {
            let line_text = if i == 0 {
                format!("{}{}", prefix, raw_line)
            } else {
                format!("  {}", raw_line) // Indent sub-lines
            };
            lines_to_add.push(TerminalLine {
                text: line_text,
                line_type: line_type.clone(),
            });
        }

        self.terminal_lines.update(|lines| {
            for line in lines_to_add {
                lines.push(line);
            }
            if lines.len() > 100 {
                let keep = lines.len() - 100;
                lines.drain(0..keep);
            }
        });
    }

    pub fn handle_run(&self) -> bool {
        let course_id = match self.current_view.get() {
            AppView::Lesson { ref course_id } => course_id.clone(),
            _ => return false,
        };

        let step = self.current_step.get();
        if let Some(course) = self.get_course(&course_id) {
            if step >= course.modules.len() {
                return false;
            }

            let module = &course.modules[step];
            let result = validate_module(module, &self.code.get());

            if result.passed {
                if matches!(module.module_type, ModuleType::Concept) {
                    self.log("Concept acknowledged. You can continue.", LineType::Info);
                } else {
                    self.log(&result.summary, LineType::Success);
                }
                self.log(&module.success_message, LineType::Success);
                self.is_success.set(true);
                self.completed.update(|steps| {
                    if step < steps.len() {
                        steps[step] = true;
                    }
                });
                self.save_diagnostics_for_current_step();
                return true;
            } else {
                self.log(&result.summary, LineType::Error);
                for line in result.feedback_lines {
                    if line.starts_with("Hint:") {
                        self.log(&line, LineType::Info);
                    } else {
                        self.log(&line, LineType::Error);
                    }
                }
                if !result.diff_lines.is_empty() {
                    self.log("Diff against the lesson solution:", LineType::Info);
                    for line in result.diff_lines {
                        let (prefix, line_type) = match line.kind {
                            DiffKind::Context => ("  ", LineType::Info),
                            DiffKind::Missing => ("- ", LineType::Error),
                            DiffKind::Extra => ("+ ", LineType::Info),
                        };
                        self.log(&format!("{}{}", prefix, line.text), line_type);
                    }
                }
                self.is_success.set(false);
                self.save_diagnostics_for_current_step();
                return false;
            }
        }
        false
    }

    pub fn reset_code(&self) {
        let course_id = match self.current_view.get() {
            AppView::Lesson { ref course_id } => course_id.clone(),
            _ => return,
        };
        let step = self.current_step.get();
        if let Some(course) = self.get_course(&course_id) {
            if step < course.modules.len() {
                self.code
                    .set(course.modules[step].initial_code.to_string());
            }
        }
    }

    pub fn has_started(&self, course_id: &str) -> bool {
        let progress = self.course_progress.get();
        progress
            .iter()
            .find(|(id, _)| id == course_id)
            .map(|(_, p)| p.started)
            .unwrap_or(false)
    }

    // ---------- Per-step diagnostics ----------

    fn diagnostics_key(course_id: &str, step: usize) -> String {
        format!("{}:{}", course_id, step)
    }

    /// Save current terminal lines for the current step
    pub fn save_diagnostics_for_current_step(&self) {
        if let AppView::Lesson { ref course_id } = self.current_view.get() {
            let step = self.current_step.get();
            let key = Self::diagnostics_key(course_id, step);
            let lines = self.terminal_lines.get();
            self.step_diagnostics.update(|map| {
                map.insert(key, lines);
            });
        }
    }

    /// Load saved diagnostics for a specific step, or initialize with empty welcome
    pub fn load_diagnostics_for_step(&self, course_id: &str, step: usize) {
        let key = Self::diagnostics_key(course_id, step);
        let map = self.step_diagnostics.get();
        if let Some(lines) = map.get(&key) {
            self.terminal_lines.set(lines.clone());
        } else {
            self.terminal_lines.set(vec![TerminalLine {
                text: "System: Ready. Write your code and press Run.".to_string(),
                line_type: LineType::Info,
            }]);
        }
    }

    /// Clear diagnostics for the current step
    pub fn clear_diagnostics(&self) {
        self.terminal_lines.set(vec![TerminalLine {
            text: "System: Terminal cleared.".to_string(),
            line_type: LineType::Info,
        }]);
        // Also remove from persistence
        if let AppView::Lesson { ref course_id } = self.current_view.get() {
            let step = self.current_step.get();
            let key = Self::diagnostics_key(course_id, step);
            self.step_diagnostics.update(|map| {
                map.remove(&key);
            });
        }
    }

    // Platform-wide progress tracking
    pub fn get_total_lessons_completed(&self) -> usize {
        let progress = self.course_progress.get();
        let mut total = 0;
        for (id, p) in progress.iter() {
            if let Some(course) = self.get_course(id) {
                let actual_total = course.modules.len();
                let completed = p.completed.iter().filter(|&&b| b).count().min(actual_total);
                total += completed;
            }
        }
        total
    }

    pub fn get_total_lessons(&self) -> usize {
        COURSES.iter().map(|c| c.modules.len()).sum()
    }

    pub fn get_courses_started_count(&self) -> usize {
        let progress = self.course_progress.get();
        progress.iter().filter(|(_, p)| p.started).count()
    }

    pub fn get_courses_completed_count(&self) -> usize {
        let mut count = 0;
        for course in COURSES.iter() {
            let (done, total) = self.get_course_progress(course.id);
            if total > 0 && done == total {
                count += 1;
            }
        }
        count
    }

    pub fn get_platform_progress_pct(&self) -> usize {
        let total = self.get_total_lessons();
        if total == 0 {
            return 0;
        }
        let completed = self.get_total_lessons_completed();
        ((completed as f64) / (total as f64) * 100.0).round() as usize
    }

    pub fn get_mastery_level(&self) -> (&'static str, &'static str, usize) {
        // Returns (level_name, icon, progress_towards_next)
        let pct = self.get_platform_progress_pct();
        match pct {
            0..=9 => ("Novice", "\u{1F331}", pct * 10),
            10..=24 => ("Apprentice", "\u{1F525}", ((pct - 10) * 100) / 15),
            25..=49 => ("Practitioner", "\u{26A1}", ((pct - 25) * 100) / 25),
            50..=74 => ("Adept", "\u{1F48E}", ((pct - 50) * 100) / 25),
            75..=99 => ("Expert", "\u{1F3C6}", ((pct - 75) * 100) / 25),
            _ => ("Master", "\u{1F451}", 100),
        }
    }

    pub fn get_next_milestone(&self) -> Option<&'static str> {
        let pct = self.get_platform_progress_pct();
        if pct >= 100 {
            return Some("All lessons mastered! \u{1F389}");
        }
        let milestones = [10, 25, 50, 75, 100];
        for m in milestones {
            if pct < m {
                return Some(match m {
                    10 => "Reach Apprentice level (10%)",
                    25 => "Reach Practitioner level (25%)",
                    50 => "Reach Adept level (50%)",
                    75 => "Reach Expert level (75%)",
                    100 => "Complete all lessons (100%)",
                    _ => unreachable!(),
                });
            }
        }
        None
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerminalLine {
    pub text: String,
    pub line_type: LineType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LineType {
    Info,
    Success,
    Error,
}
