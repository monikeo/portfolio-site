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

mod components;
use components:: {
    header::Header,
    footer::Footer,
};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

#[component]
#[warn(non_snake_case)]
fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view!{
        <Stylesheet id="leptos" href="/style/output.css" />
        <Title formatter=|text| format!("{text} - Keo Moni") />
        <Router>
            <div class="">
                <Header /> 
                <main>
                    <Routes>
                        <Route path="/" view=HomePage />
                        <Route path="/about" view=AboutPage />
                        <Route path="/contact" view=ContactPage />
                        <Route path="/projects" view=ProjectsPage />
                        <Route path="/blogs" view=BlogPage />
                        <Route path="/portfolio-site" view=HomePage />
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}

