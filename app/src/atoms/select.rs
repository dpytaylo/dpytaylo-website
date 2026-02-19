use leptos::prelude::*;
use std::{fmt::Debug, str::FromStr};

use crate::atoms::select_option::SelectOption;

#[component]
pub fn Select<T, U>(
    #[prop(optional, into)] class: Option<Signal<String>>,
    #[prop(into)] options: Signal<Vec<(T, U)>>,
    #[prop(into)] selected: ReadSignal<T>,
    #[prop(into)] set_selected: WriteSignal<T>,
) -> impl IntoView
where
    T: Clone + FromStr<Err: Debug> + Into<&'static str> + PartialEq + Send + Sync + 'static,
    U: IntoView + Clone + Send + Sync + 'static,
{
    let on_change = move |ev| {
        let new_selected = event_target_value(&ev);
        set_selected.set(new_selected.parse().unwrap());
    };

    let each = move || {
        options
            .read()
            .iter()
            .cloned()
            .enumerate()
            .collect::<Vec<_>>()
    };

    view! {
        <select class=class on:change=on_change>
            <For
                each=each
                key=|(idx, _)| *idx
                children=move |(_, (value, value_view))| {
                    view! {
                        <SelectOption value value_view selected />
                    }
                }
            />
        </select>
    }
}
