use leptos::*;

#[component]
pub fn ExtAnchor(
    #[prop(into)] href: String,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = class.unwrap_or_else(|| "text-blue-500 hover:text-blue-400".to_owned());

    view! {
        <a href=href class=class target="_blank">
            {children.map(|val| val())}
        </a>
    }
}
