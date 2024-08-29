use leptos::*;
use leptos_meta::*;

use crate::components::{
    hero_section::{
        Hero1
    },
    hero_language_tool::{
        HeroLangaugeTool
    },
    container::{
        ContainerInner,
        ContainerOuter
    },
    project_highlight::ProjectHighLight
};

#[warn(non_snake_case)]
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main>
            <ContainerInner>
                <Hero1 />
            </ContainerInner>
        </main>

        <main>
            <ContainerInner>
                <HeroLangaugeTool />
            </ContainerInner>
        </main>

        <main>
            <ContainerInner>
                <ProjectHighLight />
            </ContainerInner>
        </main>
    }
}
