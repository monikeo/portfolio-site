use leptos::*;

#[derive(Clone)]
pub struct NavigationLink {
    pub href: &'static str,
    pub label: &'static str,
}


#[component]
pub fn DesktopNavigation(links: Vec<NavigationLink>, class_name: Option<String>) -> impl IntoView {
    view! {
        <nav class={class_name.unwrap_or_default()}>
            <ul class="flex space-x-4">
                {links.into_iter().map(|link| view! {
                    <li><a href={link.href} class="text-gray-700 dark:text-gray-300">{link.label}</a></li>
                }).collect::<Vec<_>>()}
            </ul>
        </nav>
    }
}


#[component]
pub fn MobileNavigation(links: Vec<NavigationLink>, class_name: Option<String>) -> impl IntoView {
    view! { 
        <nav class={class_name.unwrap_or_default()}>
            <ul class="flex flex-col space-y-2">
                {links.into_iter().map(|link| view! {
                    <li><a href={link.href} class="text-gray-700 dark:text-gray-300">{link.label}</a></li>
                }).collect::<Vec<_>>()}
            </ul>
        </nav>
    }
}
