#![allow(unused_parens)]
use crate::data::COURSES;
use crate::data::Difficulty;
use crate::state::AppState;
use leptos::prelude::*;

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

fn course_matches_search(course: &crate::data::Course, query: &str) -> bool {
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

    // Derived filtered courses list
    let filtered_courses = Signal::derive({
        move || {
            let query = search_query.get();
            let filter = difficulty_filter.get();
            COURSES
                .iter()
                .filter(|c| course_matches_search(c, &query) && filter.matches(&c.difficulty))
                .collect::<Vec<_>>()
        }
    });

    // Platform-wide stats
    let total_completed = Signal::derive({
        let app = app;
        move || app.get_total_lessons_completed()
    });
    let total_lessons = Signal::derive({
        let app = app;
        move || app.get_total_lessons()
    });
    let courses_started = Signal::derive({
        let app = app;
        move || app.get_courses_started_count()
    });
    let courses_total = COURSES.len();
    let platform_pct = Signal::derive({
        let app = app;
        move || app.get_platform_progress_pct()
    });
    let mastery_level = Signal::derive({
        let app = app;
        move || app.get_mastery_level()
    });
    let next_milestone = Signal::derive({
        let app = app;
        move || app.get_next_milestone()
    });

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
                                <span class="text-orange-500">{"\u{1F980}"}</span>{" Rust Mastery"}
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
                                icon="\u{1F4DA}"
                                label="Lessons Done"
                                value=move || total_completed.get().to_string()
                            />
                            <StatCard
                                icon="\u{1F3AF}"
                                label="Total Lessons"
                                value=move || total_lessons.get().to_string()
                            />
                            <StatCard
                                icon="\u{1F680}"
                                label="Courses Started"
                                value=move || format!("{}/{}", courses_started.get(), courses_total)
                            />
                            <StatCard
                                icon="\u{2B50}"
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

                // Search & Filter Bar
                <div class="flex flex-col sm:flex-row gap-3 mb-8">
                    // Search input
                    <div class="relative flex-1">
                        <span class="absolute left-4 top-1/2 -translate-y-1/2 text-slate-500 text-sm">{"\u{1F50D}"}</span>
                        <input
                            type="text"
                            placeholder="Search courses by title, topic, or difficulty..."
                            class="w-full bg-white/5 border border-white/10 rounded-xl pl-11 pr-4 py-3 text-sm text-slate-300 placeholder-slate-600 focus:outline-none focus:border-orange-500/50 focus:ring-1 focus:ring-orange-500/20 transition-all"
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                search_query.set(value);
                            }
                        />
                        <Show when=move || !search_query.get().is_empty() fallback=|| ()>
                            <button
                                on:click=move |_| search_query.set(String::new())
                                class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-500 hover:text-white text-xs"
                            >
                                {"\u{2715}"}
                            </button>
                        </Show>
                    </div>

                    // Difficulty filter buttons
                    <div class="flex gap-2 flex-shrink-0">
                        <FilterButton
                            filter=DifficultyFilter::All
                            current=difficulty_filter
                        />
                        <FilterButton
                            filter=DifficultyFilter::Beginner
                            current=difficulty_filter
                        />
                        <FilterButton
                            filter=DifficultyFilter::Intermediate
                            current=difficulty_filter
                        />
                        <FilterButton
                            filter=DifficultyFilter::Advanced
                            current=difficulty_filter
                        />
                    </div>
                </div>

                // Active filter indicator
                <Show when=move || !search_query.get().is_empty() || difficulty_filter.get() != DifficultyFilter::All fallback=|| ()>
                    <div class="flex items-center gap-2 mb-6">
                        <span class="text-[10px] font-black uppercase tracking-widest text-slate-500">Active filters:</span>
                        <Show when=move || !search_query.get().is_empty()>
                            <span class="text-[10px] font-mono bg-orange-500/10 text-orange-400 px-2 py-1 rounded-md border border-orange-500/20">
                                {"\u{1F50D}"} {move || search_query.get()}
                            </span>
                        </Show>
                        <Show when=move || difficulty_filter.get() != DifficultyFilter::All>
                            <span class="text-[10px] font-mono bg-orange-500/10 text-orange-400 px-2 py-1 rounded-md border border-orange-500/20">
                                {"\u{1F3F7}"} {move || difficulty_filter.get().label()}
                            </span>
                        </Show>
                        <button
                            on:click=move |_| {
                                search_query.set(String::new());
                                difficulty_filter.set(DifficultyFilter::All);
                            }
                            class="text-[10px] font-mono text-slate-500 hover:text-white underline underline-offset-2 transition-colors"
                        >
                            Clear all
                        </button>
                    </div>
                </Show>

                // Course Grid
                <Show
                    when=move || has_results.get()
                    fallback=move || view! { <EmptyState query=search_query filter=difficulty_filter /> }
                >
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
                        {move || {
                            filtered_courses
                                .get()
                                .into_iter()
                                .map(|course| {
                                    let app = app;
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
fn FilterButton(filter: DifficultyFilter, current: RwSignal<DifficultyFilter>) -> impl IntoView {
    let filter_clone = filter.clone();
    let is_active = Signal::derive({
        let current = current;
        move || current.get() == filter_clone
    });

    let btn_class = Signal::derive({
        let filter = filter.clone();
        move || filter.active_class(is_active.get())
    });

    let label = filter.label();

    view! {
        <button
            on:click=move |_| current.set(filter.clone())
            class=format!(
                "px-3 sm:px-4 py-2 rounded-xl text-[11px] font-black uppercase tracking-wider border transition-all shrink-0 {}",
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
    course: &'static crate::data::Course,
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
    let cta_btn =
        "w-full py-3 rounded-xl font-black text-[11px] tracking-widest transition-all duration-200 active:scale-[0.98] flex items-center justify-center gap-2 bg-orange-500 hover:bg-orange-600 text-white shadow-[0_0_18px_rgba(249,115,22,0.2)] border border-orange-400/40";

    let btn_label = if started {
        if pct >= 100 { "REVIEW" } else { "CONTINUE" }
    } else {
        "START"
    };

    let btn_icon = if started {
        if pct >= 100 { "\u{1F504}" } else { "\u{25B6}" }
    } else {
        "\u{1F680}"
    };

    let card_class = format!(
        "group rounded-2xl border border-white/5 bg-white/[0.02] p-5 sm:p-6 transition-all duration-300 hover:border-white/10 hover:translate-y-[-2px] {}",
        accent_glow
    );

    let course_id = course.id.to_string();

    view! {
        <div class=card_class>
            // Top row: icon + difficulty badge
            <div class="flex items-start justify-between mb-4">
                <div class=format!("{} rounded-xl p-3", accent_bg)>
                    <span class=format!("{} text-xl", accent_text)>{course.icon}</span>
                </div>
                <span class=format!("text-[10px] font-black uppercase tracking-widest px-3 py-1.5 rounded-full border {}", course.difficulty.badge_color())>
                    {course.difficulty.label()}
                </span>
            </div>

            // Title + subtitle
            <h2 class="text-base sm:text-lg font-black text-white mb-1.5 leading-tight">{course.title}</h2>
            <p class="text-slate-500 text-xs mb-4 leading-relaxed line-clamp-2">{course.subtitle}</p>

            // Meta info row
            <div class="flex items-center gap-3 text-[10px] text-slate-600 font-mono mb-5">
                <span class="flex items-center gap-1">
                    <span>{"\u{1F4D6}"}</span> {total} lessons
                </span>
                <span class="text-slate-700">{"\u{2022}"}</span>
                <span class="flex items-center gap-1">
                    <span>{"\u{23F1}"}</span> {course.estimated_time}
                </span>
                <Show when=move || (done > 0)>
                    <span class="text-slate-700">{"\u{2022}"}</span>
                    <span class="flex items-center gap-1 text-green-400">
                        <span>{"\u{2705}"}</span> {done} done
                    </span>
                </Show>
            </div>

            // Progress bar
            <div class="mb-5">
                <div class="flex justify-between text-[10px] font-mono text-slate-600 mb-1.5">
                    <span>Progress</span>
                    <span class="text-slate-400">{format!("{done}/{total} ({pct}%)")}</span>
                </div>
                <div class="h-2 bg-slate-800/80 rounded-full overflow-hidden">
                    <div
                        class=format!("h-full rounded-full transition-all duration-500 {}", accent_bar)
                        style=format!("width: {}%", pct)
                    />
                </div>
            </div>

            // Action button
            <button
                on:click=move |_| {
                    if started {
                        app.resume_course(&course_id);
                    } else {
                        app.start_course(&course_id);
                    }
                }
                class=cta_btn
            >
                <span>{btn_icon}</span>
                <span>{btn_label}</span>
            </button>
        </div>
    }
}

#[component]
fn EmptyState(query: RwSignal<String>, filter: RwSignal<DifficultyFilter>) -> impl IntoView {
    let has_filters = Signal::derive(move || {
        !query.get().is_empty() || filter.get() != DifficultyFilter::All
    });

    view! {
        <div class="flex flex-col items-center justify-center py-20 text-center">
            <div class="text-6xl mb-6 opacity-30">{"\u{1F50D}"}</div>
            <h3 class="text-xl font-black text-white mb-2">No courses found</h3>
            <p class="text-slate-500 text-sm max-w-md mb-6">
                {move || if has_filters.get() {
                    "Try adjusting your search or filter criteria to find what you're looking for."
                } else {
                    "No courses available yet. Check back soon for new content!"
                }}
            </p>
            <Show when=move || has_filters.get()>
                <button
                    on:click=move |_| {
                        query.set(String::new());
                        filter.set(DifficultyFilter::All);
                    }
                    class="px-6 py-3 bg-orange-500 hover:bg-orange-600 text-white rounded-xl font-black text-[11px] tracking-widest transition-all active:scale-95"
                >
                    {"\u{2715}"} CLEAR ALL FILTERS
                </button>
            </Show>
        </div>
    }
}
