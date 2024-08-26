use leptos::*;
use leptos_router::*;

mod components;
use components::{
    header::Header,
    footer::Footer,
};


fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <Header/>
            /*
            <Router>
                <Route path="/" component={Home}/>
                <Route path="/about" component={About}/>
                <Route path="/projects" component={Projects}/>
                <Route path="/contact" component={Contact}/>
            </Router>
            */
            <Footer/>
        </div>
    }
}
