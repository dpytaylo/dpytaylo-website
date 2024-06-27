use leptos::*;

use crate::atoms::external_anchor::ExtAnchor;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-12 py-10 bg-white">
            <div class="mx-auto max-w-screen-sm text-base text-slate-400">
                <p class="text-center">
                    <span>"Made with ❤️ using "</span>
                    <ExtAnchor href="https://www.rust-lang.org/">"Rust"</ExtAnchor>
                    " & "
                    <ExtAnchor href="https://leptos.dev/">"Leptos"</ExtAnchor>
                </p>

                <Await
                    future=|| get_current_year()
                    let:data
                >
                    <p class="text-center">
                        {data.clone()}
                    </p>
                </Await>
            </div>
        </footer>
    }
}

#[server(GetCurrentYear, "/api")]
pub async fn get_current_year() -> Result<String, ServerFnError> {
    use chrono::{Datelike, Utc};
    Ok(Utc::now().year().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_current_year_test() {
        assert!(
            get_current_year()
                .await
                .unwrap()
                .chars()
                .all(|val| char::is_ascii_digit(&val)),
            "Excepted a valid year number",
        );
    }
}
