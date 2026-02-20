use leptos::prelude::*;

use crate::atoms::external_anchor::ExtAnchor;

#[component]
pub fn WorkExperience() -> impl IntoView {
    let projects = [
        (
            "Junior Software Developer",
            "https://epam.com",
            "EPAM",
            "Jul 2025",
            "Present",
            view! {
                <ol class="mt-2 list-disc list-inside text-gray-600">
                    <li>"Developed RESTful APIs using modern backend frameworks and followed best architecture principles."</li>
                    <li>"Worked with relational and non-relational databases, implementing efficient data models and queries by using `alchemy`."</li>
                    <li>"Implemented unit and integration tests, increasing code coverage to 80%+."</li>
                    <li>"Participated in 50+ code reviews, improving overall code quality and reducing post-release defects."</li>
                    <li>"Contributed to 3 production releases, resolving 25+ bug tickets and feature requests."</li>
                    <li>"Worked in a Scrum team of 5 — 8 members, participating in daily stand-ups, sprint planning, backlog refinement, sprint reviews, and retrospectives."</li>
                    <li>"Delivered 90%+ of committed sprint tasks on time across multiple 2-week sprint cycles."</li>
                </ol>
            }.into_any(),
        ),
        (
            "[Specialization] Backend Trainee",
            "https://epam.com",
            "EPAM",
            "Apr 2025",
            "Jun 2025",
            view! {
                <ol class="mt-2 list-disc list-inside text-gray-600">
                    <li>"Learned in depth how to develop backend systems."</li>
                    <li>"Completed intensive backend engineering training focused on building scalable and maintainable systems."</li>
                    <li>"Applied object-oriented design, SOLID principles, and design patterns in practical assignments"</li>
                </ol>
            }.into_any(),
        ),
        (
            "Java Backend Developer",
            "https://ehu.epambachelor.com/",
            "ESDE",
            "Dec 2023",
            "May 2024",
            view! {
                <p class="mt-2 text-base">"Developed Student's Hub system for own university under the EPAM specialists mentoring."</p>
                <ol class="mt-2 list-disc list-inside text-gray-600">
                    <li>"Inner freelance exchange system for university that works between students and companies."</li>
                    <li>"Worked as a backend developer in the team of 3 people. Used technologies: Spring Boot, Hibernate, Liquibase, PostgreSQL."</li>
                </ol>
            }.into_any(),
        ),
    ];

    projects.into_iter().map(|project| {
        let (position, link, company_name, start, end, content) = project;

        view! {
            <div class="mx-auto mt-6 flex flex-col gap-3 text-left max-w-3xl bg-white">
                <div class="p-6 border border-gray-200 rounded-xl shadow-md">
                    <div class="flex justify-between gap-2 text-lg">
                        <div>
                            <p class="text-lg font-bold">{position}</p>
                            <ExtAnchor href=link>{company_name}</ExtAnchor>
                        </div>
                        <p class="text-base sm:text-lg"><span class="whitespace-nowrap">{start}" —"</span>" "{end}</p>
                    </div>
                    {content}
                </div>
            </div>
        }
    }).collect::<Vec<_>>()
}
