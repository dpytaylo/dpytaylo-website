use leptos::*;
use pet_projects::PetProjects;
use short_overview::ShortOverview;
use technologies::Technologies;

use crate::{
    atoms::{anchor::Anchor, external_anchor::ExtAnchor},
    components::{
        footer::Footer,
        header::{CurrentPage, Header},
    },
    pages::contacts::CONTACTS_URL,
};

mod pet_projects;
mod short_overview;
mod technologies;

pub const ABOUT_ME_URL: &str = "/about_me";

#[component]
pub fn AboutMe() -> impl IntoView {
    view! {
        <Header current_page=CurrentPage::AboutMe />
        <main>
            <section class="pt-7 pb-12">
                <div class="mx-auto max-w-screen-lg">
                    <div class="mx-2">
                        <div class="text-center">
                            <h1 class="text-5xl font-black leading-[1.1]">
                                <span
                                    class="
                                        text-transparent bg-clip-text
                                        bg-gradient-to-r from-[rgba(124,78,238,255)] via-purple-500 to-[rgba(211,77,188,255)]
                                    "
                                >
                                    "Dmitry Pytaylo"
                                </span>
                            </h1>
                            <p class="mt-1 text-2xl">"Junior Backend Developer"</p>
                        </div>

                        <p class="sm:hidden mt-2 text-xl text-center"><Anchor href="/contacts">"Contacts"</Anchor></p>

                        <div class="hidden sm:flex mt-2 justify-center gap-2 text-xl">
                            <ExtAnchor href="mailto:dpytaylo@gmail.com">"Email"</ExtAnchor>
                            <span>"·"</span>
                            <ExtAnchor href="https://github.com/dpytaylo">"GitHub"</ExtAnchor>
                            <span>"·"</span>
                            <ExtAnchor href="https://www.linkedin.com/in/dmitry-pytaylo-a216bb261/">"LinkedIn"</ExtAnchor>
                            <span>"·"</span>
                            <span>"Lithuania, Vilnius"</span>
                        </div>

                        <div class="mx-auto mt-5 sm:mt-10 px-7 py-7 sm:px-10 sm:py-8 max-w-screen-lg rounded-xl bg-slate-100">
                            <p class="text-2xl font-semibold">"Overview"</p>
                            <p class="mt-1 sm:text-lg">
                                "As a Junior Backend Developer with extensive programming experience since 2016, I specialize in "
                                "building reliable and efficient systems. My expertise in Java, Rust, and Python enables me to develop "
                                "scalable, high-performance systems from rapid prototypes and execute complex low-level optimizations. "
                                "My professional background ensures the delivery of robust and high-quality solutions."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            <section class="mx-1 pb-12">
                <div class="mx-auto max-w-screen-lg">
                    <h2 class="mb-5 text-2xl text-center">"My primary programming stack consists of"</h2>
                    <ShortOverview/>
                </div>
            </section>

            <section class="py-16 bg-gradient-to-b from-gray-50">
                <div class="mx-auto max-w-screen-lg">
                    <div class="mx-2">
                        <h2 class="mb-5 text-3xl text-center">"Also, I have experience in these domains:"</h2>
                        <div class="mx-auto flex flex-col gap-14 max-w-screen-md">
                            <Technologies/>

                            <p class="mt-8 mb-5 text-2xl text-center">
                                "That's "
                                <mark class="px-2 text-white bg-emerald-600 rounded font-bold">"not all"</mark>
                                ". I enjoy learning and utilizing new technologies, so this list isn't exhaustive for me."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            <section class="py-16 bg-gradient-to-b from-gray-50">
                <div class="mx-2">
                    <div class="mx-auto max-w-screen-lg">
                        <h2 class="flex justify-center items-center text-4xl tracking-tighter">
                            <img src="/assets/icons/work.svg" class="inline-block w-10 h-10 mr-2" />
                            "My working experience"
                        </h2>
                    </div>

                    <div class="mx-auto mt-6 flex flex-col gap-3 text-left max-w-screen-md bg-white">
                        <div class="p-6 border rounded-xl shadow-md">
                            <div class="flex justify-between text-lg">
                                <div>
                                    <p class="text-lg font-bold">"Junior Java Web Developer"</p>
                                    <ExtAnchor href="https://ehu.epambachelor.com/">"ESDE"</ExtAnchor>
                                </div>
                                <p>"Dec 2023 - May 2024"</p>
                            </div>

                            <p class="mt-2 text-lg">"Developed Student's Hub system for own university under the EPAM specialists mentoring."</p>
                            <ol class="mt-2 list-disc list-inside text-gray-600">
                                <li>"Inner freelance exchange system for university that works between students and companies."</li>
                                <li>"Worked as a backend developer in the team of 3 people. Used technologies: Spring Boot, Hybernate, Liquibase, PostgreSQL."</li>
                            </ol>
                        </div>
                    </div>
                </div>
            </section>

            <PetProjects/>

            <section class="py-16 bg-gradient-to-b from-gray-50">
                <div class="mx-auto max-w-screen-lg">
                    <h2 class="mb-5 flex justify-center items-center text-4xl tracking-tighter">
                        <img src="/assets/icons/school_icon.svg" class="inline-block w-10 h-10 mr-2" />
                        "My education"
                    </h2>
                    <p class="mx-1 mb-4 text-center text-xl">
                        "Currently, I'm in my second year of studying Computer Science (Java) at "
                        <ExtAnchor href="https://ehu.epambachelor.com/">"EHU/ESDE"</ExtAnchor>
                        "."
                    </p>

                    <div class="mx-auto w-fit">
                        <p class="mx-1 p-5 max-w-screen-md rounded-2xl text-base bg-slate-100">
                            "\"The EHU/EPAM School of Digital Engineering is an experimental unit of EPAM, aimed at training
                            bachelor's and master's degree students in partnership with traditional universities or new educational institutions.\""
                        </p>

                        <p class="text-center text-sm text-gray-600">
                            "Origin: the official "
                            <ExtAnchor href="https://ehu.epambachelor.com/">"EHU/ESDE website"</ExtAnchor>
                            "."
                        </p>
                    </div>
                </div>
            </section>

            <section class="py-16 bg-gradient-to-b from-gray-50">
                <div class="mx-auto max-w-screen-lg">
                    <h2 class="mb-5 flex justify-center items-center text-4xl tracking-tighter">
                        <img src="/assets/icons/description_icon.svg" class="inline-block w-10 h-10 mr-2" />
                        "My certificates"
                    </h2>

                    <div class="mx-2">
                        <div class="mx-auto p-5 max-w-screen-md border rounded-xl shadow-md hmw:grid hmw:grid-cols-2 bg-white">
                            <img
                                src="/assets/summer_camp/esde_summer_camp_2023_certificate_screenshot.webp"
                                class="block mx-auto mb-5 hmw:mb-0 h-96 border"
                                alt="Image of the certificate"
                            />
                            <div class="mx-auto w-fit text-base">
                                <p class="mb-3 text-2xl font-semibold text-center">"ESDE Summer Camp 2023"</p>
                                <p>"Some of the workshops that were in this summer camp:"</p>
                                <ul class="mb-5 list-disc list-inside">
                                    <li>"Agile Fusion: Scrum and Kanban Workshop"</li>
                                    <li>"Artificial Intelligence and Machine Learning"</li>
                                    <li>"Basic Soft Skills in IT"</li>
                                    <li>"Data Science and Big Data"</li>
                                    <li>"Python-Powered Game Development"</li>
                                </ul>
                                <p>
                                    <a
                                        href="/assets/summer_camp/ESDE Summer Camp 2023 Certificate.pdf"
                                        target="_blank"
                                        class="text-blue-500 hover:text-blue-400"
                                    >
                                        "PDF version"
                                    </a>
                                </p>
                                <p>
                                    <a
                                        href="https://certificates.epam.com/certificates/0f3b9940-4e3e-471b-8e35-aa2935f8de0d"
                                        target="_blank"
                                        class="text-blue-500 hover:text-blue-400"
                                    >
                                        "EPAM digital version"
                                    </a>
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="py-36 bg-gradient-to-b from-gray-50">
                <div class="mx-auto max-w-screen-lg">
                    <div class="mx-5 hmw:mx-0">
                        <p class="text-center text-2xl hmw:text-2xl">
                            "If you have any further questions, please "
                            <a href=CONTACTS_URL class="text-blue-500 hover:text-blue-400">
                                "contact me"
                            </a>
                            "."
                        </p>
                    </div>
                </div>
            </section>
        </main>
        <Footer/>
    }
}
