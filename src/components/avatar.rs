use leptos::*;

#[component]
pub fn Container(class_name: Option<String>, children: Children) -> impl IntoView {
    view! {
        <div class={class_name.unwrap_or_else(|| "container mx-auto".to_string())}>
            {children()}
        </div>
    }
}

