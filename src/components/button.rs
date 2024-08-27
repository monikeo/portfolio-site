use leptos::*;

#[warn(non_snake_case)]
#[component]
pub fn Button(class: &'static str,children: Children) -> impl IntoView {
    view!{
        <div class="relative group m-3 w-fit inline-block">
            <div class="absolute -inset-0 group-hover:bg-gradient-to-r from-2023-bavarian-red-2 via-2023-bavarian-gold-2 to-2023-bavarian-blue-2 rounded-lg blur opacity-25 group-hover:opacity-100 transition duration-1000 group-hover:duration-200">
            </div>
            <button class="relative p-6 bg-2023-void-2 hover:bg-2023-void-2 btn btn-lg text-white ring-1 ring-2023-manga-3 rounded-2xl leading-none hover:text-shadow hover:shadow-2023-bavarian-gold-3 {class}">
                {children()}
            </button>
        </div>
    }
}
