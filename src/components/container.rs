use leptos::*;

#[warn(non_snake_case)]
#[component]
pub fn ContainerOuter(children: Children) -> impl IntoView {
    let class = "container-outer w-11/12";
    view!{
        <div class={class}>
            {children()}
        </div>
    }
}

#[warn(non_snake_case)]
#[component]
pub fn ContainerInner(children: Children) -> impl IntoView {
    let class = "container-inner w-full";
    view!{
        <div class={class}>
            {children()}
        </div>
    }
}

#[warn(non_snake_case)]
#[component]
pub fn Container(class_name: Option<String>, children: Children) -> impl IntoView {
    view! {
        <div class={class_name.unwrap_or_else(|| "container mx-auto".to_string())}>
            {children()}
        </div>
    }
}

#[warn(non_snake_case)]
#[component]
pub fn AvatarContainer(class_name: Option<String>, children: Children) -> impl IntoView {
    view! {
        <div
            class={format!(
                "{} h-10 w-10 rounded-full bg-white/90 p-0.5 shadow-lg shadow-zinc-800/5 ring-1 ring-zinc-900/5 backdrop-blur dark:bg-zinc-800/90 dark:ring-white/10",
                class_name.unwrap_or_default()
            )}
        >
            {children()}
        </div>
    }
}

