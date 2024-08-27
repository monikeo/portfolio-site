use leptos::*;
use chrono::{Utc, Datelike};

use crate::components::{
    container::{
        ContainerOuter,
        ContainerInner
    }
};

// Define the navigation items
const NAVIGATIONS: &[(&str, &str)] = &[
    ("/blogs", "Blogs"),
    ("/projects", "Projects"),
    ("/courses", "Courses"),
    ("/achievement", "Achievement"),
    ("/taekwondo", "Taekwondo"),
];


#[warn(non_snake_case)]
#[component]
fn NavLink(href: &'static str, children: Children) -> impl IntoView {
    view! { 
        <a class="transition hover:text-teal-500 dark:hover:text-teal-400" href=href> 
            {children()}
        </a>
    }
}

#[warn(non_snake_case)]
pub fn Footer() -> impl IntoView {
    let now = Utc::now();
    let year = now.year();
    view!{
        <footer class="footer footer-center bg-base-200 text-base-content rounded p-1">
            <ContainerOuter>
                <div class="border-t border-zinc-100 pt-10 pb-10 dark:border-zinc-700/40 w-full">
                    <ContainerInner>
                        <div class="flex flex-col items-center justify-between gap-6 sm:flex-row">
                            <div class="flex gap-6 text-sm font-medium text-zinc-400 dark:text-zinc-200">
                                {NAVIGATIONS.iter().map(|nav| {
                                    view! { 
                                        <NavLink href=nav.0>
                                            {nav.1}
                                        </NavLink>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>

                            <aside>
                                <p class="text-sm text-zinc-400 dark:text-zinc-500">
                                    {format!("Copyright © {} - All right reserved", year)}
                                </p>
                            </aside>
                        </div>
                    </ContainerInner>
                </div>
            </ContainerOuter>
        </footer>
    }
}
