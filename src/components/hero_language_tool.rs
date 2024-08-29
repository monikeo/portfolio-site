use leptos::*;

use crate::{
    data::language_tool_data::{
        PROGRAMMING_LANGUAGE,
        TECH_TOOL
    },
    components::button::{
        button::Button1,
        language_button::LanguageButton
    }
};


use crate::configs::language_tool_config::{
    get_config,
    Skill
};

/*
#[component]
pub fn LanguageToolButtons(languages: &'static Vec<Skill>) -> impl IntoView {
    view!{
        {
            languages.iter().map(|language| {
                view!{
                    <LanguageButton text={&language.name} />
                }
            }).collect::<Vec<_>>()
        }
    }
}
*/

#[component]
pub fn HeroLangaugeTool() -> impl IntoView {
    let hero_title = "Language and Tool";

    view!{
        <div class="hero bg-base-100 min-h-fit">
            <div class="hero-content text-center">
                <div class="max-w-xl">
                    <h1 class="mb-6 text-3xl font-bold uppercase p-5">
                        {hero_title}
                    </h1>
                    <div class="dropdown dropdown-top">
                        {

                            PROGRAMMING_LANGUAGE.iter().map(|language| {
                                view!{
                                    <LanguageButton text={language.0} icon_name={language.1} />
                                }
                            }).collect::<Vec<_>>()
                        }
                        {
                            TECH_TOOL.iter().map(|tool| {
                                view!{
                                    <LanguageButton text={tool.0} icon_name={tool.1}/>
                                }
                            }).collect::<Vec<_>>()
                        }

                    </div>
                </div>
            </div>
        </div>
    }
}
