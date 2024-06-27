use leptos::*;
use strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};
use uuid::Uuid;

use crate::atoms::{
    knowledge_level::{AdvancedLevel, BeginnerLevel, IntermediateLevel, KnowledgeLevel},
    select::Select,
};

#[derive(Debug, Clone, PartialEq, EnumIter, EnumString, IntoStaticStr)]
enum Domain {
    Any,
    Backend,
    BotDevelopment,
    Graphics,
    Science,
    Fullstack,
}

impl Domain {
    fn to_string(&self) -> &'static str {
        match self {
            Self::Any => "All domains",
            Self::Backend => "Backend",
            Self::BotDevelopment => "Bot Development",
            Self::Graphics => "Graphics",
            Self::Science => "Science",
            Self::Fullstack => "Fullstack",
        }
    }

    fn icon_path(&self) -> &'static str {
        match self {
            Self::Any => panic!("invalid domain"),
            Self::Backend => "/assets/icons/server_icon.svg",
            Self::BotDevelopment => "/assets/icons/bot_icon.svg",
            Self::Graphics => "/assets/icons/gpu_icon.svg",
            Self::Science => "/assets/icons/science_icon.svg",
            Self::Fullstack => "/assets/icons/monitor_icon.svg",
        }
    }

    fn alt_text(&self) -> &'static str {
        match self {
            Self::Any => panic!("invalid domain"),
            Self::Backend => "server icon",
            Self::BotDevelopment => "bot icon",
            Self::Graphics => "gpu icon",
            Self::Science => "math icon",
            Self::Fullstack => "monitor icon",
        }
    }
}

#[derive(Debug, Clone, PartialEq, EnumString, IntoStaticStr)]
enum Language {
    Any,
    Cpp,
    Css,
    Java,
    Python,
    Rust,
}

impl Language {
    fn logo_path(&self) -> &'static str {
        match self {
            Self::Any => panic!("invalid language"),
            Self::Cpp => "/assets/logos/cpp_logo.svg",
            Self::Css => "/assets/logos/css_logo.svg",
            Self::Java => "/assets/logos/java_logo.svg",
            Self::Python => "/assets/logos/python_logo.svg",
            Self::Rust => "/assets/logos/rust_logo.svg",
        }
    }

    fn alt_text(&self) -> &'static str {
        match self {
            Self::Any => panic!("invalid language"),
            Self::Cpp => "C++ logo",
            Self::Css => "CSS logo",
            Self::Java => "Java logo",
            Self::Python => "Python logo",
            Self::Rust => "Rust logo",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Technology {
    domain: Domain,
    language: Language,
    technology_logo: &'static str,
    technology_name: &'static str,
    level: KnowledgeLevel,
}

impl Technology {
    pub fn new(
        domain: Domain,
        language: Language,
        technology_logo: &'static str,
        technology_name: &'static str,
        level: KnowledgeLevel,
    ) -> Self {
        Self {
            domain,
            language,
            technology_logo,
            technology_name,
            level,
        }
    }
}

#[component]
pub fn Technologies() -> impl IntoView {
    #[rustfmt::skip]
    let technologies: Vec<Technology> = vec![
        Technology::new(Domain::Backend, Language::Java, "spring_boot_logo.svg", "Spring Boot", KnowledgeLevel::Intermediate),
        Technology::new(Domain::Backend, Language::Rust, "tokio_logo.svg", "Tokio, Axum", KnowledgeLevel::Intermediate),
        Technology::new(Domain::Graphics, Language::Cpp, "opengl_logo.svg", "OpenGL", KnowledgeLevel::Beginner),
        Technology::new(Domain::Graphics, Language::Cpp, "vulkan_logo.svg", "Vulkan", KnowledgeLevel::Beginner),
        Technology::new(Domain::Graphics, Language::Rust, "vulkano_logo.webp", "Vulkano", KnowledgeLevel::Beginner),
        Technology::new(Domain::Graphics, Language::Rust, "webgl_logo.svg", "WebGL", KnowledgeLevel::Beginner),
        Technology::new(Domain::BotDevelopment, Language::Python, "discord_py_logo.webp", "discord.py", KnowledgeLevel::Intermediate),
        Technology::new(Domain::BotDevelopment, Language::Rust, "serenity_logo.webp", "serenity", KnowledgeLevel::Advanced),
        Technology::new(Domain::Science, Language::Python, "numpy_logo.svg", "NumPy", KnowledgeLevel::Beginner),
        Technology::new(Domain::Science, Language::Python, "sympy_logo.svg", "SymPy", KnowledgeLevel::Beginner),
        Technology::new(Domain::Science, Language::Rust, "nalgebra_logo.svg", "nalgebra", KnowledgeLevel::Beginner),
        Technology::new(Domain::Fullstack, Language::Rust, "dioxus_logo.webp", "Dioxus", KnowledgeLevel::Beginner),
        Technology::new(Domain::Fullstack, Language::Rust, "leptos_logo.svg", "Leptos", KnowledgeLevel::Advanced),
        Technology::new(Domain::Fullstack, Language::Rust, "yew_logo.svg", "Yew", KnowledgeLevel::Beginner),
        Technology::new(Domain::Fullstack, Language::Css, "tailwindcss_logo.svg", "Tailwind CSS", KnowledgeLevel::Intermediate),
    ];

    let (domain, set_domain) = create_signal(Domain::Any);
    let domain_options = Domain::iter()
        .map(|val| (val.clone(), val.to_string()))
        .collect::<Vec<_>>();

    let (language, set_language) = create_signal(Language::Any);
    let language_options = vec![
        (Language::Any, "All languages"),
        (Language::Cpp, "C++"),
        (Language::Css, "CSS"),
        (Language::Java, "Java"),
        (Language::Python, "Python"),
        (Language::Rust, "Rust"),
    ];

    let each = create_memo(move |_| {
        let domain = domain();
        let language = language();

        Domain::iter()
            .skip(1)
            .filter(|val| *val == domain || domain == Domain::Any)
            .map(|val| (Uuid::now_v7(), val))
            .map(|(idx, domain)| {
                (
                    idx,
                    domain.clone(),
                    technologies
                        .iter()
                        .enumerate()
                        .filter(|(_, technology)| {
                            (technology.language == language || language == Language::Any)
                                && (technology.domain == domain || domain == Domain::Any)
                        })
                        .map(|(idx, technology)| (idx, technology.to_owned()))
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(_, _, technologies)| !technologies.is_empty())
            .collect::<Vec<_>>()
    });

    let children = move |(_, domain, technologies): (Uuid, Domain, Vec<(usize, Technology)>)| {
        view! {
            <div>
                <p class="mb-1 flex justify-center items-center text-2xl">
                    <img src={domain.icon_path()} class="inline-block w-8 h-8 mr-2" alt={domain.alt_text()} />
                    {domain.to_string()}
                </p>

                <ol class="flex flex-col gap-2">
                    <For
                        each=move || technologies.clone()
                        key=|(idx, _)| *idx
                        children=move |(_, val)| {
                            view! {
                                <Technology technology=val />
                            }
                        }
                    />
                </ol>
            </div>
        }
    };

    view! {
        <div class="flex justify-center">
            <Select
                class="p-1 w-full max-w-40 border rounded-md"
                options=domain_options
                selected=domain
                set_selected=set_domain
            />

            <Select
                class="p-1 w-full max-w-48 border rounded-md"
                options=language_options
                selected=language
                set_selected=set_language
            />
        </div>

        <div class="flex flex-col gap-6">
            <For
                each=each
                key=|(idx, ..)| *idx
                children=children
            />
            <Show
                when=move || { each().is_empty() }
                fallback=|| view! {}
            >
                <p class="text-lg text-center">"Nothing was found."</p>
            </Show>
        </div>
    }
}

#[component]
fn Technology(technology: Technology) -> impl IntoView {
    let technology_alt = format!("{} logo", technology.technology_name);
    let technology_logo_path = format!("/assets/logos/{}", technology.technology_logo);

    let level = match technology.level {
        KnowledgeLevel::Beginner => view! { <BeginnerLevel/> },
        KnowledgeLevel::Intermediate => view! { <IntermediateLevel/> },
        KnowledgeLevel::Advanced => view! { <AdvancedLevel/> },
    };

    view! {
        <li class="flex justify-between">
            <div>
                <img src=technology.language.logo_path() class="inline-block mr-1 w-7 h-7" alt=technology.language.alt_text() />
                <img src=technology_logo_path class="inline-block mr-2 w-7 h-7" alt=technology_alt />
                <span class="text-xl">{technology.technology_name}</span>
            </div>
            {level}
        </li>
    }
}
