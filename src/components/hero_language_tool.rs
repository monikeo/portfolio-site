use leptos::*;

use crate::{
    components::button::{
        button::Button1,
        language_button::LanguageButton
    }
};



// <div class="flex flex-wrap items-start justify-center p-5 py-10">
#[component]
pub fn HeroLangaugeTool() -> impl IntoView {
    let hero_title = "Language and Tool";
    let path = "./../../.config/data.config.toml";

    view!{
        <div class="hero bg-base-200 min-h-screen">
            <div class="hero-content text-center">
                <div class="max-w-md">
                    <h1 class="mb-5 text-3xl font-bold uppercase">
                        {hero_title}
                    </h1>
                    <div class="dropdown dropdown-top">
                        <LanguageButton text="Rust" />
                    </div>
                </div>
            </div>
        </div>
    }
}
