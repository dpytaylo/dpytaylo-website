use leptos::*;
use std::str::FromStr;

#[component]
pub fn SelectOption<T, U>(value: T, value_view: U, selected: ReadSignal<T>) -> impl IntoView
where
    T: Clone + FromStr + Into<&'static str> + PartialEq + 'static,
    U: IntoView + Clone + 'static,
{
    view! {
        <option
            value=value.clone().into()
            selected=move || value == selected()
        >
            {value_view}
        </option>
    }
}
