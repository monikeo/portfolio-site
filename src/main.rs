use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod pages;
use pages::{
    home_page::HomePage,
    about_page::AboutPage,
    project_page::ProjectsPage,
    contact_page::ContactPage,
    blog_page::BlogPage
};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view!{
        <Stylesheet id="leptos" href="/style/output.css" />
        <Title formatter=|text| format!("{text} - Keo Moni") />
        <Router>
            <Header/> 
            <main>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/about" view=AboutPage />
                    <Route path="/contact" view=ContactPage />
                    <Route path="/projects" view=ProjectsPage />
                    <Route path="/blog" view=BlogPage />
                </Routes>
            </main>
            <Header/>
        </Router>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <h1>"My Portfolio"</h1>
            <nav>
                <ul>
                    <li><a href="/">Home</a></li>
                    <li><a href="/about">About</a></li>
                    <li><a href="/projects">Projects</a></li>
                    <li><a href="/contact">Contact</a></li>
                </ul>
            </nav>
        </header>
    }
}
