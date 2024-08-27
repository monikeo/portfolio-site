use leptos::*;

use crate::components::button::Button;

#[component]
pub fn HeroLangaugeTool() -> impl IntoView {
    let hero_title = "Language and Tool";
    view!{
        <div className="min-h-screen hero bg-gradient-radial from-2023-bavarian-gold-4 to-2023-void-2 bg-blend-overlay pt-10">
            <div className="hero-overlay w-0 bg-opacity-60">
            </div>
            <div className="text-center hero-content">
                <div className="max-w-md">
                    <h1 className="mb-5 text-2023-manga-3 text-5xl font-bold uppercase">
                        {hero_title}
                    </h1>

                    <div className="dropdown dropdown-top">
                        <Button class="umami--click--otherlangs-button">
              Other languages
            </Button>
                    </div>
                </div>
            </div>
        </div>
    }
}
