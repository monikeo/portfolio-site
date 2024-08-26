use leptos::*;
use chrono::{Utc, Datelike};

// Define the navigation items
const NAVIGATIONS: &[(&str, &str)] = &[
    ("/blogs", "Blogs"),
    ("/projects", "Projects"),
    ("/courses", "Courses"),
    ("/achievement", "Achievement"),
    ("/taekwondo", "Taekwondo"),
];

// Define the Container components as placeholders
#[component]
fn ContainerOuter(children: Children) -> impl IntoView {
    view! { 
        <div class="container-outer">
            {children()}
        </div>
    }
}

#[component]
fn ContainerInner(children: Children) -> impl IntoView {
    view! { 
        <div class="container-inner">
            {children()}
        </div>
    }
}

#[component]
fn NavLink(href: &'static str, children: Children) -> impl IntoView {
    view! { 
        <a class="transition hover:text-teal-500 dark:hover:text-teal-400" href=href> 
            {children()}
        </a>
    }
}

pub fn Footer() -> impl IntoView {
    let now = Utc::now();
    let year = now.year();
    let author_name = "Moni Keo";
    view!{
        <footer class="mt-32">
            <ContainerOuter>
                <div class="border-t border-zinc-100 pt-8 pb-16 dark:border-zinc-700/40">
                    <ContainerInner>
                        <div class="flex flex-col items-center justify-between gap-6 sm:flex-row">
                            <div class="flex gap-6 text-sm font-medium text-zinc-800 dark:text-zinc-200">
                            {NAVIGATIONS.iter().map(|nav| {
                                    view! { 
                                        <NavLink href=nav.0>
                                            {nav.1}
                                        </NavLink>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>

                            <p class="text-sm text-zinc-400 dark:text-zinc-500">
                            {format!("© {} {} . all rights reserved.", year, author_name)}
                            </p>
                        </div>
                    </ContainerInner>
                </div>
            </ContainerOuter>
        </footer>
    }
}
