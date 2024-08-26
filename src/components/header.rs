use leptos::*;

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
    let (is_home_page, set_is_home_page) = create_signal(true);
    let (scroll_y, set_scroll_y) = create_signal(0);
    let (header_height, set_header_height) = create_signal(0);
    let (header_mb, set_header_mb) = create_signal(0);
    let (avatar_transform, set_avatar_transform) = create_signal("translate3d(0, 0, 0) scale(1)");

    view! {
        <header 
            class="pointer-events-none relative z-50 flex flex-col"
        >
            <h3>haha</h3>
        </header>
        <h1> Header </h1>
    }
}
