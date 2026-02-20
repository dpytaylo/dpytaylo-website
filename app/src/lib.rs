use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*;

use crate::pages::about_me::AboutMe;
use crate::pages::contacts::Contacts;
use crate::pages::error_page_template::ErrorPageTemplate;
use crate::pages::home::HomePage;

mod atoms;
mod components;
pub mod error_template;
mod pages;
pub mod utils;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Link rel="preload" href="/assets/fonts/Inter/Inter.woff2" as_="font" crossorigin="anonymous" />
        <Style>
            "
            @font-face {
                font-family: 'Inter';
                src: local('Inter'), url('/assets/fonts/Inter/Inter.woff2');
            }
            "
        </Style>

        <Stylesheet id="leptos" href="/pkg/main.css"/>
        <Title text="dpytaylo"/>

        <Router>
            <div class="font-inter">
                <Routes fallback=|| {
                    view! {
                        <ErrorPageTemplate
                            error="Page Not Found"
                            description="The page you're looking for can't be found. It might have been moved, deleted, or perhaps it never existed. Let's help you get back on track."
                        />
                    }
                }>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("about-me") view=AboutMe />
                    <Route path=path!("contacts") view=Contacts />
                </Routes>
            </div>
        </Router>
    }
}
