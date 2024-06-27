use leptos::*;

use crate::atoms::anchor::Anchor;

#[component]
pub fn ErrorPageTemplate(
    #[prop(into)] error: String,
    #[prop(optional, into)] description: Option<String>,
) -> impl IntoView {
    let go_back = |_| {
        window().history().unwrap().back().unwrap();
    };

    view! {
        <div class="h-screen flex flex-col">
            <main class="mx-auto px-2 mt-28 mb-3 w-full max-w-screen-md text-lg sm:text-xl">
                <h1 class="text-4xl sm:text-6xl font-black leading-tight text-slate-900">{error}</h1>
                <hr class="my-3 border-gray-300" />
                <p class="mt-5">{description}</p>

                <p class="flex mt-7 justify-between gap-2 flex-wrap">
                    <Anchor
                        href="#"
                        class="
                            text-base px-4 py-2 border rounded-md border-gray-300 cursor-pointer
                            text-white bg-blue-500 hover:bg-blue-400 transition-color select-none
                        "
                        on:click=go_back
                    >
                        "Return back"
                    </Anchor>
                    <Anchor
                        href="/"
                        class="
                            text-base px-4 py-2 border rounded-md border-gray-300 cursor-pointer 
                            bg-slate-50 hover:bg-slate-200 transition-colors select-none
                        "
                    >
                        "Go to the home page"
                    </Anchor>
                </p>
            </main>
        </div>
    }
}
