use leptos::prelude::*;
use pet_projects::PetProjects;
use short_overview::ShortOverview;
use technologies::Technologies;

use crate::{
    atoms::{anchor::Anchor, external_anchor::ExtAnchor},
    components::{
        footer::Footer,
        header::{CurrentPage, Header},
    },
    pages::{about_me::work_experience::WorkExperience, contacts::CONTACTS_URL},
};

mod pet_projects;
mod short_overview;
mod technologies;
mod work_experience;

pub const ABOUT_ME_URL: &str = "/about-me";

#[component]
pub fn AboutMe() -> impl IntoView {
    view! {
        <Header current_page=CurrentPage::AboutMe />
        <main>
            <section class="pt-7 pb-12">
                <div class="mx-auto max-w-5xl">
                    <div class="mx-2">
                        <div class="text-center">
                            <h1 class="text-5xl font-black leading-[1.1]">
                                <span
                                    class="
                                        text-transparent bg-clip-text
                                        bg-linear-to-r from-[rgba(124,78,238,255)] via-purple-500 to-[rgba(211,77,188,255)]
                                    "
                                >
                                    "Dmitry Pytaylo"
                                </span>
                            </h1>
                            <p class="mt-1 text-2xl">"Backend Developer"</p>
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

                        <div class="mx-auto mt-5 sm:mt-10 px-7 py-7 sm:px-10 sm:py-8 max-w-5xl rounded-xl bg-slate-100">
                            <p class="text-2xl font-semibold">"Overview"</p>
                            <p class="mt-1 sm:text-lg">
                                "As a Backend Developer with extensive programming experience since 2016, I specialize in "
                                "building reliable and efficient systems. My expertise in Python, Rust, and Java enables me to develop "
                                "scalable, high-performance systems from rapid prototypes and execute complex low-level optimizations. "
                                "My professional background ensures the delivery of robust and high-quality solutions."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            <section class="mx-1 pb-12">
                <div class="mx-auto max-w-5xl">
                    <h2 class="mb-5 text-2xl text-center">"My primary programming stack consists of"</h2>
                    <ShortOverview/>
                </div>
            </section>

            <section class="py-16 bg-linear-to-b from-gray-50">
                <div class="mx-auto max-w-5xl">
                    <div class="mx-2">
                        <h2 class="mb-5 text-3xl text-center">"Also, I have experience in these domains:"</h2>
                        <div class="mx-auto flex flex-col gap-14 max-w-3xl">
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

            <section class="py-16 bg-linear-to-b from-gray-50">
                <div class="mx-2">
                    <div class="mx-auto max-w-5xl">
                        <h2 class="flex justify-center items-center text-4xl tracking-tighter">
                            <img src="/assets/icons/work.svg" class="inline-block w-10 h-10 mr-2" />
                            "My working experience"
                        </h2>
                    </div>

                    <WorkExperience/>
                </div>
            </section>

            <PetProjects/>

            <section class="py-16 bg-linear-to-b from-gray-50">
                <div class="mx-2">
                    <div class="mx-auto max-w-5xl">
                        <h2 class="mb-5 flex justify-center items-center text-4xl tracking-tighter">
                            <img src="/assets/icons/school_icon.svg" class="inline-block w-10 h-10 mr-2" />
                            "My education"
                        </h2>
                        <div class="mx-auto mt-6 flex flex-col gap-3 text-left max-w-3xl bg-white">
                            <div class="p-6 border border-gray-200 rounded-xl shadow-md">
                                <div class="mx-auto px-2 max-w-3xl text-lg">
                                    <div class="flex justify-between gap-2">
                                        <div class="flex-none">
                                            <img src="/assets/logos/sdc_logo.jpg" class="inline-block mr-1 w-12 h-12 sm:w-16 sm:h-16" alt="Logo of EHU SDC" />
                                        </div>
                                        <div class="px-4 flex-auto content-center">
                                            <p class="font-bold">"EHU SDC"</p>
                                            <p class="text-base">"Bachelor's degree, Computer Science"</p>
                                        </div>
                                        <p class="text-base sm:text-lg">
                                            <span class="whitespace-nowrap">"2022 —"</span>" 2026"
                                        </p>
                                    </div>
                                    <div class="text-base">
                                        <p class="mt-4 mb-2">"My university was launched as the EHU EPAM School of Digital Engineering."</p>
                                        <div>
                                            <p class="mx-1 p-5 max-w-3xl rounded-2xl text-sm bg-slate-100">
                                                "\"The EHU/EPAM School of Digital Engineering is an experimental unit of EPAM, aimed at training
                                                bachelor's and master's degree students in partnership with traditional universities or new educational institutions.\""
                                            </p>

                                            <p class="text-center text-sm text-gray-600">
                                                "Origin: the official "
                                                <ExtAnchor href="http://web.archive.org/web/20230613020858/https://ehu.epambachelor.com/">"EHU/ESDE website"</ExtAnchor>
                                                "."
                                            </p>
                                        </div>

                                        <p class="mt-3">"Over time, it smoothly transitioned into the EHU School of Digital Competences."</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="py-16 bg-linear-to-b from-gray-50">
                <div class="mx-auto max-w-5xl">
                    <h2 class="mb-5 flex justify-center items-center text-4xl tracking-tighter">
                        <img src="/assets/icons/description_icon.svg" class="inline-block w-10 h-10 mr-2" />
                        "My certificates"
                    </h2>

                    <div class="mx-2">
                        <div class="mx-auto p-5 max-w-3xl border border-gray-200 rounded-xl shadow-md hmw:grid hmw:grid-cols-2 bg-white">
                            <img
                                src="/assets/summer_camp/esde_summer_camp_2023_certificate_screenshot.webp"
                                class="block mx-auto mb-5 hmw:mb-0 h-96 border border-gray-200"
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

            <section class="py-36 bg-linear-to-b from-gray-50">
                <div class="mx-auto max-w-5xl">
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
