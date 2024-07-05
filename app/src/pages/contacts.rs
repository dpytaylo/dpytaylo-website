use leptos::*;
use leptos_icons::Icon;

use crate::{
    atoms::external_anchor::ExtAnchor,
    components::{
        footer::Footer,
        header::{CurrentPage, Header},
    },
};

pub const CONTACTS_URL: &str = "/contacts";

#[component]
pub fn Contacts() -> impl IntoView {
    view! {
        <div class="h-screen flex flex-col">
            <Header current_page=CurrentPage::Links />
            <main class="mt-7 h-full">
                <p class="text-5xl text-center font-black leading-tight text-slate-900">"Contacts"</p>

                <div class="mx-auto w-full max-w-96">
                    <div class="flex mx-2 mt-3 flex-col gap-2 font-semibold tracking-wide">
                        <ExtAnchor
                            href="mailto:dpytaylo@gmail.com"
                            class="
                                flex py-3 px-8 max-w-screen-md justify-center items-center
                                bg-zinc-100 hover:bg-zinc-200 rounded-xl cursor-pointer gap-3
                                transition-colors
                            "
                        >
                            <Icon icon=icondata::BsEnvelopeFill width="1em" height="1em" class="w-5 h-5 text-gray-700" />
                            "Email"
                            <Icon icon=icondata::BsBoxArrowUpRight width="1em" height="1em" class="w-4 h-4 text-gray-700" />
                        </ExtAnchor>

                        <ExtAnchor
                            href="https://www.linkedin.com/in/dmitry-pytaylo-a216bb261/"
                            class="
                                flex py-3 px-8 max-w-screen-md justify-center items-center
                                bg-zinc-100 hover:bg-zinc-200 rounded-xl cursor-pointer gap-3
                                transition-colors
                            "
                        >
                            <Icon icon=icondata::BsLinkedin width="1em" height="1em" class="w-5 h-5 text-gray-700" />
                            "LinkedIn"
                            <Icon icon=icondata::BsBoxArrowUpRight width="1em" height="1em" class="w-4 h-4 text-gray-700" />
                        </ExtAnchor>

                        <ExtAnchor
                            href="https://github.com/dpytaylo"
                            class="
                                flex py-3 px-8 max-w-screen-md justify-center items-center
                                bg-zinc-100 hover:bg-zinc-200 rounded-xl cursor-pointer gap-3
                                transition-colors
                            "
                        >
                            <Icon icon=icondata::BsGithub width="1em" height="1em" class="w-5 h-5 text-gray-700" />
                            "GitHub"
                            <Icon icon=icondata::BsBoxArrowUpRight width="1em" height="1em" class="w-4 h-4 text-gray-700" />
                        </ExtAnchor>

                        <ExtAnchor
                            href="https://t.me/dpytaylo"
                            class="
                                flex py-3 px-8 max-w-screen-md justify-center items-center
                                bg-zinc-100 hover:bg-zinc-200 rounded-xl cursor-pointer gap-3
                                transition-colors
                            "
                        >
                            <Icon icon=icondata::BsTelegram width="1em" height="1em" class="w-5 h-5 text-gray-700" />
                            "Telegram"
                            <Icon icon=icondata::BsBoxArrowUpRight width="1em" height="1em" class="w-4 h-4 text-gray-700" />
                        </ExtAnchor>

                        <ExtAnchor
                            href="https://discord.com/users/362258590926372864"
                            class="
                                flex py-3 px-8 max-w-screen-md justify-center items-center
                                bg-zinc-100 hover:bg-zinc-200 rounded-xl cursor-pointer gap-3
                                transition-colors
                            "
                        >
                            <Icon icon=icondata::BsDiscord width="1em" height="1em" class="w-5 h-5 text-gray-700" />
                            "Discord"
                            <Icon icon=icondata::BsBoxArrowUpRight width="1em" height="1em" class="w-4 h-4 text-gray-700" />
                        </ExtAnchor>
                    </div>
                </div>
            </main>
            <Footer/>
        </div>
    }
}
