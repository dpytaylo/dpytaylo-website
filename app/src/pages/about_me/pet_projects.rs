use leptos::*;

use crate::atoms::external_anchor::ExtAnchor;

#[component]
pub fn PetProjects() -> impl IntoView {
    let projects = [(
        "Tic-tac-toe Discord bot",
        view! {
            <p>
                <span class="underline">"Links"</span>
                ": "
                <ExtAnchor href="https://github.com/dpytaylo/tic-tac-toe-discord-bot">"GitHub"</ExtAnchor>
                ", "
                <ExtAnchor href="https://www.youtube-nocookie.com/embed/ap8j1Ht-Oxk?si=zIhSZCNANxVldQvj">"YouTube"</ExtAnchor>
                "."
            </p>
            <p>"A Discord bot for playing tic-tac-toe, demonstrating skills in bot development and real-time interaction using Rust."</p>
        }.into_view(),
    ),
    (
        "api-error-derive",
        view! {
            <p>
                <span class="underline">"Links"</span>
                ": "
                <ExtAnchor href="https://github.com/dpytaylo/api-error-derive">"GitHub"</ExtAnchor>
                "."    
            </p>
            <p>"A procedural macro for deriving error handling in Rust, aimed at simplifying error management in API development."</p>
        }.into_view(),
    ),
    (
        "simple-messenger",
        view! {
            <p>
                <span class="underline">"Links"</span>
                ": "
                <ExtAnchor href="https://github.com/dpytaylo/simple-messenger">"GitHub"</ExtAnchor>
                "."    
            </p>
            <p>
                "A comprehensive full-stack application built using axum, sea-orm (PostgreSQL), redis, and leptos. "
                "This project showcases advanced skills in building a robust and efficient messaging platform."
            </p>
        }.into_view(),
    )]
    .into_iter()
    .map(|val| view! {
        <div class="mx-auto mt-6 flex flex-col gap-3 text-left max-w-screen-md">
            <div class="p-6 border rounded-xl shadow-md bg-white">
                <p class="text-xl text-center font-bold">{val.0}</p>
                <p class="mt-2">
                    {val.1}
                </p>
            </div>
        </div>
    })
    .collect_view();

    view! {
        <section class="px-2 py-16 bg-gradient-to-b from-gray-50">
            <div class="mx-auto max-w-screen-lg">
                <p class="mb-5 flex justify-center items-center text-4xl tracking-tighter">
                    <img src="/assets/icons/settings_heart.svg" class="inline-block w-10 h-10 mr-2" />
                    "My pet projects"
                </p>

                {projects}
            </div>
        </section>
    }
}
