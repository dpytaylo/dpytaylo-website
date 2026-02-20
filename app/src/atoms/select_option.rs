use leptos::prelude::*;
use std::str::FromStr;

#[component]
pub fn SelectOption<T, U>(value: T, value_view: U, selected: ReadSignal<T>) -> impl IntoView
where
    T: Clone + FromStr + Into<&'static str> + PartialEq + Send + Sync + 'static,
    U: IntoView + Clone + 'static,
{
    let value_str: &'static str = value.into();

    view! {
        <option
            value=value_str
            selected=move || selected.with(|x| { let x: &'static str = x.to_owned().into(); x == value_str })
        >
            {value_view}
        </option>
    }
}
