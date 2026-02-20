use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Anchor(
    #[prop(into)] href: String,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] alt: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = class.unwrap_or_else(|| "text-blue-500 hover:text-blue-400".to_owned());

    view! {
        <A href=href attr:class=class attr:alt=alt>
            {children.map(|val| val())}
        </A>
    }
}
