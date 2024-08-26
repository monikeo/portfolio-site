use leptos::*;

// Define the navigation items
const NAVIGATIONS: &[(&str, &str)] = &[
    ("/", "Home"),
    ("/#publications", "Publications"),
    ("/projects", "Projects"),
    ("/courses", "Courses"),
    ("/snippets", "Snippets"),
    ("/resources", "Resources"),
    ("/articles", "Posts"),
];

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header className="pointer-events-none relative z-50 flex flex-col">
            <h3>haha</h3>
        </header>
        <h1> Header </h1>
    }
}
