use leptos::*;

#[warn(non_snake_case)]
#[component]
pub fn Button(class: &'static str,children: Children) -> impl IntoView {
    view!{
        <div class="relative group m-3 w-fit inline-block">
            <div class="absolute -inset-0 group-hover:bg-gradient-to-r from-2023-bavarian-red-2 via-2023-bavarian-gold-2 to-2023-bavarian-blue-2 rounded-lg blur opacity-25 group-hover:opacity-100 transition duration-1000 group-hover:duration-200">
            </div>
            <button class="relative p-6 bg-2023-void-2 hover:bg-2023-void-2 btn btn-lg text-white ring-1 ring-2023-manga-3 rounded-2xl leading-none hover:text-shadow hover:shadow-[2px_5px_50px_-25px_rgba(46,194,126,1)] {class}">
                {children()}
            </button>
        </div>
    }
}

#[warn(non_snake_case)]
#[component]
pub fn Button1() -> impl IntoView {
    view! {
                    <button
                        class="btn relative inline-block text-white no-underline shadow-2xl group ">
                        <span
                            class="absolute inset-0 overflow-hidden">
                            <span
                                class="absolute inset-0 bg-[image:radial-gradient(75%_100%_at_50%_0%,rgba(56,189,248,0.6)_0%,rgba(56,189,248,0)_75%)] opacity-0 transition-opacity duration-500 group-hover:opacity-30">

                            </span>
                        </span>
                        <div
                            class="relative z-10 flex items-center px-6 py-3">
                            <span>Lets get started</span>
                        </div>
                        <span
                            class="absolute -bottom-0 left-[1.125rem] h-px w-[calc(100%-2.25rem)] bg-gradient-to-r from-emerald-400/0 via-gray-400/90 to-emerald-400/0 transition-opacity duration-500 group-hover:opacity-40"></span>
                    </button>

    }
}
