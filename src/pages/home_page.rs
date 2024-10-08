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
    blog_highlight::BlogHighLight,
    project_highlight::ProjectHighLight,
    portfolio::{
        education::Education
    }
};

#[warn(non_snake_case)]
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        // Hero1
        <main>
            <ContainerInner>
                <Hero1 />
            </ContainerInner>
        </main>

        // Tech Stack and Tool
        <main>
            <ContainerInner>
                <HeroLangaugeTool />
            </ContainerInner>
        </main>

        // Project
        <main>
            <ContainerInner>
                <ProjectHighLight />
            </ContainerInner>
        </main>

        // Blog 
        <main>
            <ContainerInner>
                <BlogHighLight />
            </ContainerInner>
        </main>

        // Education
        <main>
            <ContainerInner>
                <Education />
            </ContainerInner>
        </main>
    }
}
