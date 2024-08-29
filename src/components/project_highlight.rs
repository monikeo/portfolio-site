use leptos::*;

use crate::components::project_card::{ProjectInfo, ProjectCard, ProjectInfoList};

#[component]
pub fn ProjectHighLight() -> impl IntoView {
    let project1 = ProjectInfo {
        title: Some(String::from("Portfolio Site")),
        status: Some(String::from("Developing")),
        description: Some(String::from("My personal portfolio site build with rust leptos ")),
        tag: Some(vec![String::from("rust"), String::from("wasm")]),
        img_src: Some(String::from("https://media.dev.to/cdn-cgi/image/width=1000,height=500,fit=cover,gravity=auto,format=auto/https%3A%2F%2Fdev-to-uploads.s3.amazonaws.com%2Fi%2Fvlvdmm1clqcgkvbes5c6.png")),
        img_alt: None,
    };

    view!{
        <div class="hero bg-base-200 min-h-screen">
            <div class="hero-content text-center">
                <div class="max-w-full">
                    <h1 class="mb-6 text-3xl font-bold uppercase p-5">My Project</h1>

                    <div class="carousel carousel-center bg-white bg-opacity-10 rounded-box space-x-4 p-4 xl:max-w-full lg:max-w-full md:max-w-md sm:max-w-sm max-w-md">
                        <div class="carousel-item m-1">
                            <ProjectCard project_info={project1.clone()}/>
                        </div>
                        <div class="carousel-item m-1">
                            <ProjectCard project_info={project1.clone()}/>     
                        </div>
                        <div class="carousel-item m-1">
                            <ProjectCard project_info={project1.clone()}/> 
                        </div>
                        <div class="carousel-item m-1">
                            <ProjectCard project_info={project1.clone()}/>
                        </div>
                    </div>

                </div>
            </div>
        </div>
    }
}
