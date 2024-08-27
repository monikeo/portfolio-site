use leptos::*;

#[warn(non_snake_case)]
#[component]
pub fn Hero1() -> impl IntoView {
    let hero1_title = "I am Moni";
    let hero1_content = "I am Moni Keo, a dedicated Cyber Security major with a strong commitment  to discipline and continuous learning. As a Taekwondo national athlete, I bring resilience and  focus to everything I do. I love exploring technology, contributing to  open-source projects, and sharing my knowledge through blogging. My  passion for self-improvement and community engagement drives me to  constantly expand my skills and make a positive impact in the tech  world.";
    let hero1_button = "Get resume";
    let hero1_img = "./../public/assets/images/hero1_profile.jpg";
    view!{
        <div class="hero bg-base-200 min-h-screen">
            <div class="hero-content flex-col lg:flex-row-reverse">
                <img
                    src={hero1_img}
                    class="max-w-sm rounded-lg shadow-2xl" />
                <div>
                    <h1 class="text-5xl font-bold">
                        {hero1_title}
                    </h1>
                    <p class="py-6 w-4/5">
                        {hero1_content}
                    </p>
                    <button class="btn btn-primary">
                        {hero1_button}
                    </button>
                </div>
            </div>
        </div>
    }
}

