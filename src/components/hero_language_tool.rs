use leptos::*;

use crate::components::button::{
    button::Button1,
    language_button::LanguageButton
};

#[component]
pub fn HeroLangaugeTool() -> impl IntoView {
    let hero_title = "Language and Tool";
    view!{
        <div className="min-h-screen hero bg-gradient-radial from-bavarian-gold-4 to-void-2 bg-blend-overlay pt-10">
            <div className="hero-overlay w-0 bg-opacity-60">
            </div>
            <div className="text-center hero-content">
                <div className="max-w-md">
                    <h1 className="mb-5 text-2023-manga-3 text-5xl font-bold uppercase">
                        {hero_title}
                    </h1>

                    <div className="dropdown dropdown-top">
                        <LanguageButton text="Rust" />
                        <Button1 />
                    </div>
                </div>
            </div>
        </div>
    }
}
