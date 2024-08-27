use leptos::*;

#[component]
pub fn Error404Page() -> impl IntoView {
    view!{
        <div class="hero bg-base-200 min-h-screen m-auto">
            <div class="hero-content text-center">
                <div class="max-w-md">
                     <h1 class="text-5xl font-bold">404</h1>
                     <p class="py-6">
                        You have discovered a secret place
                    </p>
                    <p class="pb-7">
                        Unfortunately, this is only a 404 page. You may have
            mistyped the address, or the page has been moved to another URL.
                    </p>

                    <a href="/">
                        <button class="btn btn-outline btn-error">Go back to home page</button>
                    </a>
                </div>
            </div>
        </div>
    }
}
