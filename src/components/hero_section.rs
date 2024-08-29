use leptos::*;

use crate::components::{
    social_media::SocialMedia,
};

#[warn(non_snake_case)]
#[component]
pub fn Hero1() -> impl IntoView {
    let hero1_title = "I am Moni";
    let hero1_content = "I am Moni Keo, a dedicated Cyber Security major with a strong commitment  to discipline and continuous learning. As a Taekwondo national athlete, I bring resilience and  focus to everything I do. I love exploring technology, contributing to  open-source projects, and sharing my knowledge through blogging. My  passion for self-improvement and community engagement drives me to  constantly expand my skills and make a positive impact in the tech  world.";
    let hero1_button1 = "Get resume";
    let hero1_button2 = "About";
    let hero1_img = "https://i.postimg.cc/cL95xQBV/hero1-profile.jpg";
    let hero1_img_alt = "Moni's Profile'";

    view!{
        <div class="hero bg-base-100 min-h-screen">
            <div class="hero-content flex-col lg:flex-row-reverse">
                <img
                    src={hero1_img}
                    alt={hero1_img_alt}
                    class="max-w-sm rounded-lg shadow-2xl" />
                <div>
                    <h1 class="text-5xl font-bold text-accent">
                        {hero1_title}
                    </h1>
                    <p class="py-6 w-4/5 text-accent">
                        {hero1_content}
                    </p>
                    <div class="mb-4">
                        <SocialMedia />
                    </div>
                    <div class="flex space-x-8">
                            <button class="btn btn-outline btn-primary">
                                {hero1_button1}
                            </button>
                            <button class="btn btn-outline btn-secondary">
                                {hero1_button2}
                            </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

