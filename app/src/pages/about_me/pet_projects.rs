use leptos::*;

use crate::atoms::external_anchor::ExtAnchor;
use crate::atoms::masonry::Masonry;
use crate::utils::Rem;

const CARD_MAX_WIDTH_PX: f64 = 384.0;

#[component]
pub fn PetProjects() -> impl IntoView {
    let projects: Vec<_> = [(
        "/assets/previews/website.webp",
        "Personal website",
        view! {
            <p>
                "This website is built using the Leptos Rust fullstack framework, showcasing my skills in web development and design."
            </p>
        }.into_view(),
    ),
    (
        "/assets/previews/tic-tac-toe.webp",
        "Tic-tac-toe Discord bot",
        view! {
            <p>
                <span class="underline">"Links"</span>
                ": "
                <ExtAnchor href="https://github.com/dpytaylo/tic-tac-toe-discord-bot">"GitHub"</ExtAnchor>
                ", "
                <ExtAnchor href="https://www.youtube.com/watch?v=ap8j1Ht-Oxk">"YouTube"</ExtAnchor>
                "."
            </p>
            <p>"A Discord bot for playing tic-tac-toe, demonstrating skills in bot development and real-time interaction using Rust."</p>
        }.into_view(),
    ),
    (
        "/assets/previews/api-error-derive.webp",
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
        "/assets/previews/simple-messenger.webp",
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
    .map(|val| {
        view! {
            <div class="border rounded-xl overflow-hidden">
                <img src=val.0 class="w-full h-56 object-cover" />
                <div class="p-6 shadow-md bg-white">
                    <h2 class="text-xl text-center font-bold">{val.1}</h2>
                    <p class="mt-2">
                        {val.2}
                    </p>
                </div>
            </div>
        }
    })
    .collect();

    view! {
        <section class="px-2 py-16 bg-gradient-to-b from-gray-50">
            <div class="mx-auto max-w-screen-lg">
                <h1 class="mb-5 flex justify-center items-center text-4xl tracking-tighter">
                    <img src="/assets/icons/settings_heart.svg" class="inline-block w-10 h-10 mr-2" />
                    "My pet projects"
                </h1>

                <Masonry
                    elements=projects
                    max_card_width_px=CARD_MAX_WIDTH_PX
                    gap=Rem(0.75)
                />
            </div>
        </section>
    }
}
