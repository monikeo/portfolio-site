use leptos::*;

use crate::components::{
    title::Title,
    project_card::ProjectCard,
};

pub fn ProjectHighLight() -> impl IntoView {
    view!{
        <div class="hero bg-base-100 min-h-fit">
            <div class="hero-content text-center">
                <div class="max-w-full">
                    <Title text="My Project" />

                </div>
            </div>
        </div>
    }
}
