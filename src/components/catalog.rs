#![allow(unused_parens)]
use crate::data::COURSES;
use crate::data::Difficulty;
use crate::data::model::CourseKind;
use crate::state::AppState;
use leptos::prelude::*;

#[derive(Clone, PartialEq, Copy)]
enum CatalogTab {
    GuidedPaths,
    DeepDives,
}

#[derive(Clone, PartialEq)]
enum DifficultyFilter {
    All,
    Beginner,
    Intermediate,
    Advanced,
}

impl DifficultyFilter {
    fn label(&self) -> &'static str {
        match self {
            DifficultyFilter::All => "All",
            DifficultyFilter::Beginner => "Beginner",
            DifficultyFilter::Intermediate => "Intermediate",
            DifficultyFilter::Advanced => "Advanced",
        }
    }

    fn matches(&self, difficulty: &Difficulty) -> bool {
        match (self, difficulty) {
            (DifficultyFilter::All, _) => true,
            (DifficultyFilter::Beginner, Difficulty::Beginner) => true,
            (DifficultyFilter::Intermediate, Difficulty::Intermediate) => true,
            (DifficultyFilter::Advanced, Difficulty::Advanced) => true,
            _ => false,
        }
    }

    fn active_class(&self, is_active: bool) -> &'static str {
        if is_active {
            "bg-orange-500 text-white border-orange-500 shadow-[0_0_12px_rgba(249,115,22,0.3)]"
        } else {
            "bg-white/5 text-slate-400 border-white/10 hover:bg-white/10 hover:text-slate-300"
        }
    }
}

#[derive(Clone, PartialEq)]
enum TagFilter {
    All,
    Tag(String),
}

impl TagFilter {
    fn active_class(&self, is_active: bool) -> &'static str {
        if is_active {
            "bg-indigo-500 text-white border-indigo-500 shadow-[0_0_12px_rgba(99,102,241,0.3)]"
        } else {
            "bg-white/5 text-slate-400 border-white/10 hover:bg-white/10 hover:text-slate-300"
        }
    }
}

fn get_unique_tags() -> Vec<String> {
    let mut tags = std::collections::HashSet::new();
    for c in COURSES {
        if matches!(c.kind, CourseKind::Focus) {
            for t in c.tags {
                tags.insert(t.to_string());
            }
        }
    }
    let mut vec: Vec<String> = tags.into_iter().collect();
    vec.sort();
    vec
}

fn course_matches_search(course: &crate::data::model::Course, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let q = query.to_lowercase();
    course.title.to_lowercase().contains(&q)
        || course.subtitle.to_lowercase().contains(&q)
        || course.difficulty.label().to_lowercase().contains(&q)
        || course.id.to_lowercase().contains(&q)
}

#[component]
pub fn Catalog(app: AppState) -> impl IntoView {
    let search_query = RwSignal::new(String::new());
    let difficulty_filter = RwSignal::new(DifficultyFilter::All);
    let tag_filter = RwSignal::new(TagFilter::All);
    let active_tab = RwSignal::new(CatalogTab::GuidedPaths);

    let all_tags = get_unique_tags();

    // Derived filtered courses list
    let filtered_courses = Signal::derive(move || {
        let query = search_query.get();
        let diff = difficulty_filter.get();
        let tag = tag_filter.get();
        let tab = active_tab.get();

        COURSES
            .iter()
            .filter(|c| {
                let tab_match = match tab {
                    CatalogTab::GuidedPaths => !matches!(c.kind, CourseKind::Focus),
                    CatalogTab::DeepDives => matches!(c.kind, CourseKind::Focus),
                };
                if !tab_match {
                    return false;
                }

                if !course_matches_search(c, &query) {
                    return false;
                }

                match tab {
                    CatalogTab::GuidedPaths => diff.matches(&c.difficulty),
                    CatalogTab::DeepDives => match &tag {
                        TagFilter::All => true,
                        TagFilter::Tag(t) => c.tags.contains(&t.as_str()),
                    },
                }
            })
            .collect::<Vec<_>>()
    });

    // Platform-wide stats
    let total_completed = Signal::derive(move || app.get_total_lessons_completed());
    let total_lessons = Signal::derive(move || app.get_total_lessons());
    let courses_started = Signal::derive(move || app.get_courses_started_count());
    let courses_total = COURSES.len();
    let platform_pct = Signal::derive(move || app.get_platform_progress_pct());
    let mastery_level = Signal::derive(move || app.get_mastery_level());
    let next_milestone = Signal::derive(move || app.get_next_milestone());

    let has_results = Signal::derive(move || !filtered_courses.get().is_empty());
    let has_any_progress = Signal::derive(move || total_completed.get() > 0);

    view! {
        <div class="flex-1 overflow-y-auto custom-scrollbar">
            <div class="max-w-6xl mx-auto px-4 sm:px-8 py-8 sm:py-12">

                // Welcome / Hero Section
                <div class="mb-10">
                    <div class="flex flex-col sm:flex-row sm:items-end sm:justify-between gap-4 mb-6">
                        <div>
                            <h1 class="text-3xl sm:text-4xl font-black text-white mb-2 tracking-tight">
                                <span class="text-orange-500">{"🦀"}</span>{" Rust Mastery"}
                            </h1>
                            <p class="text-slate-500 text-sm sm:text-base">
                                Interactive lessons. Write code. Get instant feedback.
                            </p>
                        </div>
                        <Show when=move || has_any_progress.get() fallback=|| ()>
                            <div class="flex items-center gap-2 bg-white/5 border border-white/10 rounded-xl px-4 py-2">
                                <span class="text-lg">{move || mastery_level.get().1}</span>
                                <div>
                                    <div class="text-[10px] font-black uppercase tracking-widest text-slate-400">Current Rank</div>
                                    <div class="text-sm font-bold text-white">{move || mastery_level.get().0}</div>
                                </div>
                            </div>
                        </Show>
                    </div>

                    // Motivation Stats Row
                    <Show when=move || has_any_progress.get() fallback=|| ()>
                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mb-8">
                            <StatCard
                                icon="📚"
                                label="Lessons Done"
                                value=move || total_completed.get().to_string()
                            />
                            <StatCard
                                icon="🎯"
                                label="Total Lessons"
                                value=move || total_lessons.get().to_string()
                            />
                            <StatCard
                                icon="🚀"
                                label="Courses Started"
                                value=move || format!("{}/{}", courses_started.get(), courses_total)
                            />
                            <StatCard
                                icon="⭐"
                                label="Overall Progress"
                                value=move || format!("{}%", platform_pct.get())
                            />
                        </div>

                        // Overall progress bar
                        <div class="mb-8 bg-white/5 border border-white/10 rounded-xl p-4">
                            <div class="flex justify-between items-center mb-2">
                                <span class="text-[10px] font-black uppercase tracking-widest text-slate-400">Platform Mastery</span>
                                <span class="text-xs font-bold text-white">{move || format!("{}/{} lessons", total_completed.get(), total_lessons.get())}</span>
                            </div>
                            <div class="h-3 bg-slate-800 rounded-full overflow-hidden">
                                <div
                                    class="h-full rounded-full bg-gradient-to-r from-orange-500 to-amber-400 transition-all duration-700 ease-out"
                                    style=move || format!("width: {}%", platform_pct.get())
                                />
                            </div>
                            <div class="mt-2 text-xs text-slate-500">
                                {move || next_milestone.get().unwrap_or("")}
                            </div>
                        </div>
                    </Show>
                </div>

                // TABS
                <div class="flex gap-6 border-b border-white/10 mb-8 pb-4">
                    <button 
                        on:click=move |_| {
                            active_tab.set(CatalogTab::GuidedPaths);
                            search_query.set(String::new());
                        }
                        class=move || {
                            let base = "font-black tracking-wider text-sm transition-colors pb-4 -mb-[17px] border-b-2 ";
                            if active_tab.get() == CatalogTab::GuidedPaths {
                                format!("{} text-white border-orange-500", base)
                            } else {
                                format!("{} text-slate-500 border-transparent hover:text-slate-300", base)
                            }
                        }
                    >"📚 GUIDED PATHS"</button>
                    <button 
                        on:click=move |_| {
                            active_tab.set(CatalogTab::DeepDives);
                            search_query.set(String::new());
                        }
                        class=move || {
                            let base = "font-black tracking-wider text-sm transition-colors pb-4 -mb-[17px] border-b-2 ";
                            if active_tab.get() == CatalogTab::DeepDives {
                                format!("{} text-white border-indigo-500", base)
                            } else {
                                format!("{} text-slate-500 border-transparent hover:text-slate-300", base)
                            }
                        }
                    >"🎯 DEEP DIVES"</button>
                </div>

                // Search & Filter Bar
                <div class="flex flex-col sm:flex-row gap-3 mb-8 items-start">
                    // Search input
                    <div class="relative w-full sm:max-w-md shrink-0">
                        <span class="absolute left-4 top-1/2 -translate-y-1/2 text-slate-500 text-sm">{"🔍"}</span>
                        <input
                            type="text"
                            placeholder="Search courses..."
                            class="w-full bg-white/5 border border-white/10 rounded-xl pl-11 pr-4 py-3 text-sm text-slate-300 placeholder-slate-600 focus:outline-none focus:border-white/20 transition-all"
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                search_query.set(value);
                            }
                        />
                    </div>

                    // Context-aware filters
                    <div class="flex-1 flex gap-2 overflow-x-auto pb-2 scrollbar-hide shrink-0 items-center h-[46px]">
                        <Show 
                            when=move || active_tab.get() == CatalogTab::GuidedPaths
                            fallback=move || view! {
                                <div class="flex gap-2">
                                    <TagFilterButton filter=TagFilter::All current=tag_filter label="All".to_string() />
                                    {all_tags.clone().into_iter().map(|t| view! {
                                        <TagFilterButton filter=TagFilter::Tag(t.clone()) current=tag_filter label=t />
                                    }).collect_view()}
                                </div>
                            }
                        >
                            <div class="flex gap-2">
                                <DifficultyFilterButton filter=DifficultyFilter::All current=difficulty_filter />
                                <DifficultyFilterButton filter=DifficultyFilter::Beginner current=difficulty_filter />
                                <DifficultyFilterButton filter=DifficultyFilter::Intermediate current=difficulty_filter />
                                <DifficultyFilterButton filter=DifficultyFilter::Advanced current=difficulty_filter />
                            </div>
                        </Show>
                    </div>
                </div>

                // Course Grid
                <Show
                    when=move || has_results.get()
                    fallback=move || view! { <EmptyState query=search_query /> }
                >
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
                        {move || {
                            filtered_courses
                                .get()
                                .into_iter()
                                .map(|course| {
                                    let (done, total) = app.get_course_progress(course.id);
                                    let pct = app.get_course_progress_pct(course.id);
                                    let started = app.has_started(course.id);
                                    view! {
                                        <CourseCard
                                            app
                                            course
                                            done
                                            total
                                            pct
                                            started
                                        />
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </div>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn StatCard(
    icon: &'static str,
    label: &'static str,
    value: impl Fn() -> String + 'static,
) -> impl IntoView {
    view! {
        <div class="bg-white/5 border border-white/10 rounded-xl p-3 sm:p-4 flex items-center gap-3">
            <span class="text-xl sm:text-2xl">{icon}</span>
            <div class="min-w-0">
                <div class="text-lg sm:text-xl font-black text-white truncate">{value()}</div>
                <div class="text-[10px] font-black uppercase tracking-widest text-slate-500">{label}</div>
            </div>
        </div>
    }
}

#[component]
fn DifficultyFilterButton(filter: DifficultyFilter, current: RwSignal<DifficultyFilter>) -> impl IntoView {
    let filter_c1 = filter.clone();
    let is_active = Signal::derive(move || current.get() == filter_c1);
    
    let filter_c2 = filter.clone();
    let btn_class = Signal::derive(move || filter_c2.active_class(is_active.get()));
    
    let label = filter.label();
    let filter_c3 = filter.clone();

    view! {
        <button
            on:click=move |_| current.set(filter_c3.clone())
            class=format!(
                "px-4 h-[46px] rounded-xl text-[11px] font-black uppercase tracking-wider border transition-all shrink-0 flex items-center {}",
                btn_class.get()
            )
        >
            {label}
        </button>
    }
}

#[component]
fn TagFilterButton(filter: TagFilter, current: RwSignal<TagFilter>, label: String) -> impl IntoView {
    let filter_c1 = filter.clone();
    let is_active = Signal::derive(move || current.get() == filter_c1);
    
    let filter_c2 = filter.clone();
    let btn_class = Signal::derive(move || filter_c2.active_class(is_active.get()));
    
    let filter_c3 = filter.clone();

    view! {
        <button
            on:click=move |_| current.set(filter_c3.clone())
            class=format!(
                "px-4 h-[46px] rounded-xl text-[11px] font-black uppercase tracking-wider border transition-all shrink-0 flex items-center {}",
                btn_class.get()
            )
        >
            {label}
        </button>
    }
}

#[component]
fn CourseCard(
    app: AppState,
    course: &'static crate::data::model::Course,
    done: usize,
    total: usize,
    pct: usize,
    started: bool,
) -> impl IntoView {
    if matches!(course.kind, CourseKind::Focus) {
        view! { <FocusCard app course done total pct started /> }.into_any()
    } else {
        view! { <CurriculumCard app course done total pct started /> }.into_any()
    }
}

#[component]
fn CurriculumCard(
    app: AppState,
    course: &'static crate::data::model::Course,
    done: usize,
    total: usize,
    pct: usize,
    started: bool,
) -> impl IntoView {
    let accent = course.accent;
    let accent_bg = format!("bg-{}-500/10", accent);
    let accent_text = format!("text-{}-500", accent);
    let accent_bar = format!("bg-gradient-to-r from-{}-500 to-{}-400", accent, accent);
    let accent_glow = format!("hover:border-{}-500/30 hover:shadow-[0_0_30px_rgba(249,115,22,0.05)]", accent);
    
    let btn_label = if started { if pct >= 100 { "REVIEW" } else { "CONTINUE" } } else { "START PATH" };
    let btn_icon = if started { if pct >= 100 { "🔄" } else { "▶" } } else { "🚀" };

    let card_class = format!(
        "group rounded-2xl border border-white/5 bg-gradient-to-b from-white/[0.03] to-transparent p-6 transition-all duration-300 hover:translate-y-[-2px] flex flex-col {}",
        accent_glow
    );

    let course_id = course.id.to_string();

    view! {
        <div class=card_class>
            <div class="flex items-start justify-between mb-4">
                <div class=format!("{} rounded-xl p-4 shadow-inner", accent_bg)>
                    <span class=format!("{} text-3xl", accent_text)>{course.icon}</span>
                </div>
                <span class=format!("text-[10px] font-black uppercase tracking-widest px-3 py-1.5 rounded-full border {}", course.difficulty.badge_color())>
                    {course.difficulty.label()}
                </span>
            </div>

            <h2 class="text-xl font-black text-white mb-2 leading-tight">{course.title}</h2>
            <p class="text-slate-400 text-sm mb-6 leading-relaxed flex-1">{course.subtitle}</p>

            <div class="flex items-center gap-3 text-xs text-slate-500 font-mono mb-6 bg-black/20 p-3 rounded-xl">
                <span class="flex items-center gap-2">
                    <span class="text-slate-400">"📚"</span> {total} lessons
                </span>
                <span class="text-slate-700">|</span>
                <span class="flex items-center gap-2">
                    <span class="text-slate-400">"⏱"</span> {course.estimated_time}
                </span>
            </div>

            <div class="mb-6">
                <div class="flex justify-between text-[10px] font-mono text-slate-500 mb-2">
                    <span class="uppercase tracking-widest">Progress</span>
                    <span class="text-slate-300 font-bold">{format!("{done}/{total} ({pct}%)")}</span>
                </div>
                <div class="h-2 bg-slate-900 rounded-full overflow-hidden shadow-inner">
                    <div
                        class=format!("h-full rounded-full transition-all duration-500 {}", accent_bar)
                        style=format!("width: {}%", pct)
                    />
                </div>
            </div>

            <button
                on:click=move |_| if started { app.resume_course(&course_id) } else { app.start_course(&course_id) }
                class="w-full py-3 sm:py-4 rounded-xl font-black text-[12px] sm:text-sm tracking-widest transition-all duration-200 active:scale-[0.98] flex items-center justify-center gap-2 bg-orange-500 hover:bg-orange-400 text-orange-950 shadow-[0_0_20px_rgba(249,115,22,0.15)]"
            >
                <span>{btn_icon}</span>
                <span>{btn_label}</span>
            </button>
        </div>
    }
}

#[component]
fn FocusCard(
    app: AppState,
    course: &'static crate::data::model::Course,
    done: usize,
    total: usize,
    pct: usize,
    started: bool,
) -> impl IntoView {
    let btn_label = if started { if pct >= 100 { "REVIEW" } else { "CONTINUE" } } else { "PRACTICE" };
    let course_id = course.id.to_string();

    let tag_label = if course.tags.is_empty() {
        "DEEP DIVE"
    } else {
        course.tags[0]
    };

    view! {
        <div class="group rounded-2xl border border-white/5 bg-white/[0.01] p-5 transition-all duration-300 hover:border-indigo-500/30 hover:bg-white/[0.02] flex flex-col">
            <div class="flex items-center gap-3 mb-4">
                <span class="text-2xl">{course.icon}</span>
                <span class="text-[10px] font-black uppercase tracking-widest px-2 py-1 bg-indigo-500/10 text-indigo-400 rounded-md">
                    {tag_label}
                </span>
            </div>

            <h2 class="text-base font-black text-white mb-2 leading-tight line-clamp-1">{course.title}</h2>
            <p class="text-slate-500 text-xs mb-5 line-clamp-2 flex-1">{course.subtitle}</p>

            <div class="flex items-center justify-between mt-auto">
                <div class="flex flex-col gap-1">
                    <span class="text-[10px] font-mono text-slate-500">{total} " lessons • " {course.estimated_time}</span>
                    <div class="w-24 h-1 bg-slate-800 rounded-full overflow-hidden mt-1">
                        <div class="h-full bg-indigo-500 rounded-full" style=format!("width: {}%", pct)></div>
                    </div>
                </div>
                <button
                    on:click=move |_| if started { app.resume_course(&course_id) } else { app.start_course(&course_id) }
                    class="px-4 py-2 bg-white/5 hover:bg-indigo-500 text-slate-300 hover:text-white rounded-lg text-[10px] font-black tracking-widest transition-all"
                >
                    {btn_label}
                </button>
            </div>
        </div>
    }
}

#[component]
fn EmptyState(query: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-20 text-center col-span-full">
            <div class="text-6xl mb-6 opacity-30">"🔍"</div>
            <h3 class="text-xl font-black text-white mb-2">No courses found</h3>
            <p class="text-slate-500 text-sm max-w-md mb-6">
                "Try adjusting your search criteria to find what you're looking for."
            </p>
            <button
                on:click=move |_| query.set(String::new())
                class="px-6 py-3 bg-white/10 hover:bg-white/20 text-white rounded-xl font-black text-[11px] tracking-widest transition-all active:scale-95"
            >
                "✖ CLEAR SEARCH"
            </button>
        </div>
    }
}
