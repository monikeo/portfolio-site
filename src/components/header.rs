use leptos::*;

use crate::components::{
    navigation::Nav,
    container::{
        ContainerOuter,
        ContainerInner
    }
};
// Define the navigation items
const NAVIGATIONS: &[(&str, &str)] = &[
    ("/", "Home"),
    ("/projects", "Projects"),
    ("/blogs", "Blogs"),
    ("/resume", "Resume"),
    ("/contact", "Contact"),
    ("/about", "About"),
];


// Define AvatarContainer component
#[component]
fn AvatarContainer(class: Option<String>, children: Children) -> impl IntoView {
    view! { 
        <div class={class.unwrap_or("h-10 w-10 rounded-full bg-white/90 p-0.5 shadow-lg shadow-zinc-800/5 ring-1 ring-zinc-900/5 backdrop-blur dark:bg-zinc-800/90 dark:ring-white/10".to_string())}>
            {children()}
        </div>
    }
}

// Define Avatar component
#[component]
fn Avatar(large: bool, class: Option<String>) -> impl IntoView {
    view! { 
        <div class={class.unwrap_or(if large { "block h-16 w-16 origin-left".to_string() } else { "block h-8 w-8".to_string() })}>
        </div>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="footer footer-center text-base-centent rounded pt-1 pb-1">
            <ContainerOuter>
                <div class="w-full">
                    <ContainerInner>
                        <Nav />
                    </ContainerInner>
                </div>
            </ContainerOuter>

        </header>
    }
}
