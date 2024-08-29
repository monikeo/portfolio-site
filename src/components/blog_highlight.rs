use leptos::*;

use crate::components::{
    title::Title,
    blog_card::{BlogInfo, BlogCard, BlogInfoList}
};

#[component]
pub fn BlogHighLight() -> impl IntoView {
    let blog1 = BlogInfo {
        title: Some(String::from("Portfolio Site")),
        status: Some(String::from("Developing")),
        description: Some(String::from("My personal portfolio site build with rust leptos ")),
        tag: Some(vec![String::from("rust"), String::from("wasm")]),
        img_src: Some(String::from("https://media.dev.to/cdn-cgi/image/width=1000,height=500,fit=cover,gravity=auto,format=auto/https%3A%2F%2Fdev-to-uploads.s3.amazonaws.com%2Fi%2Fvlvdmm1clqcgkvbes5c6.png")),
        img_alt: None,
    };

    view!{
        <div class="hero bg-base-100 min-h-screen">
            <div class="hero-content text-center">
                <div class="max-w-full">
                    <Title text="My Blog" />

                    <div class="carousel carousel-center bg-white bg-opacity-10 rounded-box space-x-4 p-4 xl:max-w-full lg:max-w-full md:max-w-md sm:max-w-sm max-w-md">
                        <div class="carousel-item m-1">
                            <BlogCard blog_info={blog1.clone()}/>
                        </div>
                        <div class="carousel-item m-1">
                            <BlogCard blog_info={blog1.clone()}/>     
                        </div>
                        <div class="carousel-item m-1">
                            <BlogCard blog_info={blog1.clone()}/> 
                        </div>
                        <div class="carousel-item m-1">
                            <BlogCard blog_info={blog1.clone()}/>
                        </div>
                    </div>

                </div>
            </div>
        </div>
    }
}
