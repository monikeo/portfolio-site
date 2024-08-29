use leptos::*;

#[component]
pub fn Title(text: &'static str) -> impl IntoView {
    view!{
        <h1 class="mb-6 text-3xl font-bold uppercase p-5 text-accent">
            {text}
        </h1>
    }
}
