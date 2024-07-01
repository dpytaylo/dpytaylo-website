use leptos::*;
use leptos_icons::Icon;

use crate::atoms::{anchor::Anchor, external_anchor::ExtAnchor};

#[derive(Clone, Copy, PartialEq)]
pub enum CurrentPage {
    Home,
    AboutMe,
    Links,
}

#[component]
pub fn Header(current_page: CurrentPage) -> impl IntoView {
    view! {
        <header class="px-6 flex-shrink-0 sticky w-full top-0 left-0 h-12 bg-white/[.93] backdrop-blur border-b border-gray-200 z-10">
            <div class="mx-auto max-w-screen-xl h-full flex items-center justify-between">
                <div class="ml-4 hmw:ml-0 hmw:w-full hmw:max-w-48">
                    <Anchor
                        href="/"
                        class="flex mx-auto w-fit h-12 items-center text-2xl font-header whitespace-nowrap select-none"
                        alt="dpytaylo logo"
                    >
                        <span>"dpytaylo"</span>
                    </Anchor>
                </div>

                <input
                    type="checkbox"
                    id="menu_button"
                    class="hidden"
                />

                <nav class="
                    absolute left-[50%] -translate-x-1/2
                    flex-grow hidden hmw:flex flex-wrap justify-between items-center menu-open-handler
                ">
                    <ul class="
                        hidden hmw:flex
                        flex-row font-inter space-x-8 select-none
                        list-none
                        text-gray-900
                    ">
                        <li>
                            <a
                                href="/"
                                class="hover:text-blue-700"
                                class=("text-blue-700", move || current_page == CurrentPage::Home)
                            >
                                "Home"
                            </a>
                        </li>
                        <li>
                            <a
                                href="/about_me"
                                class="hover:text-blue-700"
                                class=("text-blue-700", move || current_page == CurrentPage::AboutMe)
                            >
                                "About me"
                            </a>
                        </li>
                        <li>
                            <a
                                href="/contacts"
                                class="hover:text-blue-700"
                                class=("text-blue-700", move || current_page == CurrentPage::Links)
                            >
                                "Contacts"
                            </a>
                        </li>
                    </ul>
                </nav>

                <div class="hidden hmw:flex gap-4 justify-end items-center">
                    <ExtAnchor
                        href="mailto:dpytaylo@gmail.com"
                    >
                        <span class="sr-only">"Email"</span>
                        <Icon icon=icondata::BsEnvelopeFill width="1em" height="1em" class="w-6 h-6 text-gray-400 hover:text-gray-500 cursor-pointer" />
                    </ExtAnchor>

                    <ExtAnchor
                        href="https://www.linkedin.com/in/dmitry-pytaylo-a216bb261/"
                    >
                        <span class="sr-only">"LinkedIn"</span>
                        <Icon icon=icondata::BsLinkedin width="1em" height="1em" class="w-5 h-5 text-gray-400 hover:text-gray-500 cursor-pointer" />
                    </ExtAnchor>

                    <ExtAnchor
                        href="https://github.com/dpytaylo"
                    >
                        <span class="sr-only">"GitHub"</span>
                        <Icon icon=icondata::BsGithub width="1em" height="1em" class="w-5 h-5 text-gray-400 hover:text-gray-500 cursor-pointer" />
                    </ExtAnchor>

                    <ExtAnchor
                        href="https://t.me/dpytaylo"
                    >
                        <span class="sr-only">"Telegram"</span>
                        <Icon icon=icondata::BsTelegram width="1em" height="1em" class="w-5 h-5 text-gray-400 hover:text-gray-500 cursor-pointer" />
                    </ExtAnchor>

                    <ExtAnchor
                        href="https://discord.com/users/362258590926372864"
                    >
                        <span class="sr-only">"Discord"</span>
                        <Icon icon=icondata::BsDiscord width="1em" height="1em" class="w-6 h-6 text-gray-400 hover:text-gray-500 cursor-pointer" />
                    </ExtAnchor>
                </div>

                <label for="menu_button" class="
                    hmw:hidden w-10 h-10 rounded-lg hover:bg-gray-100 bg-menu-icon bg-cover cursor-pointer
                    menu-button-label
                ">
                    <span class="sr-only">"Open main menu"</span>
                </label>
            </div>
        </header>
    }
}
