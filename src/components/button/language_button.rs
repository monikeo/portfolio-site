use leptos::*;
use crate::components::button::button::Button;

#[component]
pub fn LanguageButton(text: &'static str) -> impl IntoView {
    view!{
        <Button class="flex items-center gap-3">
            // add language or tool icon
            {text}
        </Button>
    }
}
