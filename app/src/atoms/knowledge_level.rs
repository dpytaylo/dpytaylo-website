use leptos::*;

#[derive(Debug, Clone, PartialEq)]
pub enum KnowledgeLevel {
    Beginner,
    Intermediate,
    Advanced,
}

#[component]
pub fn BeginnerLevel() -> impl IntoView {
    view! {
        <img src="/assets/levels/beginner.svg" class="inline-block w-20 h-7" alt="beginner level" />
    }
}

#[component]
pub fn IntermediateLevel() -> impl IntoView {
    view! {
        <img src="/assets/levels/intermediate.svg" class="inline-block w-20 h-7" alt="intermediate level" />
    }
}

#[component]
pub fn AdvancedLevel() -> impl IntoView {
    view! {
        <img src="/assets/levels/advanced.svg" class="inline-block w-20 h-7" alt="advanced level" />
    }
}
