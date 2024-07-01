use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::about_me::AboutMe;
use crate::pages::contacts::Contacts;
use crate::pages::error_page_template::ErrorPageTemplate;
use crate::pages::home::HomePage;

mod atoms;
mod components;
pub mod error_template;
mod pages;
mod utils;

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

        <Router fallback=|| {
            view! {
                <ErrorPageTemplate
                    error="Page Not Found"
                    description="The page you're looking for can't be found. It might have been moved, deleted, or perhaps it never existed. Let's help you get back on track."
                />
            }
        }>
            <div class="font-inter">
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="about_me" view=AboutMe />
                    <Route path="contacts" view=Contacts />
                </Routes>
            </div>
        </Router>
    }
}
