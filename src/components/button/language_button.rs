use leptos::*;
use crate::components::button::button::Button;

#[component]
pub fn LanguageButton(text: &'static str, icon_name: &'static str) -> impl IntoView {
    let img_src = format!("https://skillicons.dev/icons?i={}", icon_name);
    view!{
        <Button class="flex items-center">
            // add language or tool icon
            <img src={img_src} class="size-8 pr-2"/>
            {text}
        </Button>
    }
}
