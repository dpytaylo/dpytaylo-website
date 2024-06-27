use leptos::*;
use leptos_icons::Icon;

use crate::{
    atoms::external_anchor::ExtAnchor,
    components::{
        footer::Footer,
        header::{CurrentPage, Header},
    },
};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="h-screen flex flex-col">
            <Header current_page=CurrentPage::Home />
            <main class="h-full">
                <div class="relative top-[50%] -translate-y-1/2 text-center">
                    <h1 class="text-6xl font-semibold leading-[1.1]">
                        <span class="
                            bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500
                            font-black text-transparent bg-clip-text bg-300% animate-gradient
                        ">
                            "Dmitry Pytaylo"
                        </span>
                    </h1>
                    <h2 class="mt-2 text-2xl">"Backend Software Developer"</h2>
                    <div class="mt-3 flex justify-center items-center gap-3">
                        <ExtAnchor
                            href="mailto:dpytaylo@gmail.com"
                        >
                            <span class="sr-only">"Mail"</span>
                            <Icon icon=icondata::BsEnvelopeFill width="1em" height="1em" class="w-7 h-7 text-gray-400 hover:text-gray-500 cursor-pointer" />
                        </ExtAnchor>

                        <ExtAnchor
                            href="https://www.linkedin.com/in/dmitry-pytaylo-a216bb261/"
                        >
                            <span class="sr-only">"LinkedIn"</span>
                            <Icon icon=icondata::BsLinkedin width="1em" height="1em" class="w-6 h-6 text-gray-400 hover:text-gray-500 cursor-pointer" />
                        </ExtAnchor>

                        <ExtAnchor
                            href="https://github.com/dpytaylo"
                        >
                            <span class="sr-only">"GitHub"</span>
                            <Icon icon=icondata::BsGithub width="1em" height="1em" class="w-6 h-6 text-gray-400 hover:text-gray-500 cursor-pointer" />
                        </ExtAnchor>

                        <ExtAnchor
                            href="https://t.me/dpytaylo"
                        >
                            <span class="sr-only">"Telegram"</span>
                            <Icon icon=icondata::BsTelegram width="1em" height="1em" class="w-6 h-6 text-gray-400 hover:text-gray-500 cursor-pointer" />
                        </ExtAnchor>

                        <ExtAnchor
                            href="https://discord.com/users/362258590926372864"
                        >
                            <span class="sr-only">"Discord"</span>
                            <Icon icon=icondata::BsDiscord width="1em" height="1em" class="w-7 h-7 text-gray-400 hover:text-gray-500 cursor-pointer" />
                        </ExtAnchor>
                    </div>
                </div>
            </main>
            <Footer/>
        </div>
    }
}
