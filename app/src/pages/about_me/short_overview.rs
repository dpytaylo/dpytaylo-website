use leptos::*;

#[component]
pub fn ShortOverview() -> impl IntoView {
    let values = [
        (
            "/assets/logos/rust_logo.svg",
            "Rust logo",
            "Rust, a modern programming language, to develop backend services with a strong emphasis on safety and concurrency."
        ),
        (
            "/assets/logos/python_logo.svg",
            "Python logo",
            "Python, a versatile high-level programming language, for rapid prototyping and comprehensive data analysis."
        ),
        (
            "/assets/logos/java_logo.svg",
            "Java logo",
            "Java, a time-tested and reliable language, for applications where its robust ecosystem is most advantageous."
        )
    ]
    .into_iter()
    .map(|val| view! {
        <div class="flex-shrink-0 w-60 p-2 border rounded shadow-md bg-white">
            <img src=val.0 class="mx-auto mt-2 mb-4 object-contain w-24 h-24" alt=val.1 />
            <p class="mb-2">
                {val.2}
            </p>
        </div>
    })
    .collect_view();

    view! {
        <div class="mx-auto flex overflow-x-auto gap-4 text-center" style="justify-content: safe center">
            {values}
        </div>
    }
}
