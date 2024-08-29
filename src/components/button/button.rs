use leptos::*;

#[warn(non_snake_case)]
#[component]
pub fn Button(class: &'static str,children: Children) -> impl IntoView {
    view!{
        <div class="relative group m-3 w-fit inline-block">
            <button class={format!("relative btn text-accent ring-secondary ring-opacity-20 hover:ring-opacity-100 ring-1 rounded-2xl leading-none hover:text-shadow hover:shadow-[2px_5px_50px_-25px_rgba(46,194,126,1)] {}", class)}>
                {children()}
            </button>
        </div>
    }
}

#[warn(non_snake_case)]
#[component]
pub fn Button1(children: Children) -> impl IntoView {
    view! {
            <button class="relative inline-block text-accent no-underline shadow-lg group p-1 m-1 ">
                <span class="absolute inset-0 overflow-hidden">
                    <span
                    class="absolute inset-0 opacity-0 transition-opacity duration-500 gorup-hover:opacity-30">
//                        class="absolute inset-0 bg-[image:radial-gradient(75%_100%_at_50%_0%,rgba(142,238,150,0.6)_0%,rgba(142,238,150,0)_75%)] opacity-0 transition-opacity duration-500 group-hover:opacity-30">
                    </span>
                </span>
                <div class="relative z-10 flex items-center px-6 py-3">
                    {children()}
                </div>
                <span
                    class="absolute -bottom-0 left-[1.125rem] h-px w-[calc(100%-2.25rem)] bg-gradient-to-r from-emerald-400/0 via-gray-100/90 to-emerald-400/0 transition-opacity duration-500 group-hover:opacity-60"></span>
            </button>

    }
}

#[warn(non_snake_case)]
#[component]
pub fn Button2(text: &'static str) -> impl IntoView {
    view!{
        <button 
            class="bg-base-100 text-accent border border-accent border-b-4 font-medium overflow-hidden relative px-4 py-2 rounded-md hover:brightness-150 hover:border-t-4 hover:border-b active:opacity-75 outline-none duration-300 group">
            <span 
                class="bg-base-100 shadow-accent absolute -top-[150%] left-0 inline-flex w-80 h-[5px] rounded-md opacity-50 group-hover:top-[150%] duration-500 shadow-[0_0_10px_10px_rgba(0,0,0,0.3)]">

            </span>
            {text}
        </button>
    }
}
