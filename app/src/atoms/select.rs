use leptos::*;
use std::{fmt::Debug, str::FromStr};

use super::select_option::SelectOption;

#[component]
pub fn Select<T, U>(
    #[prop(optional, into)] class: Option<MaybeSignal<String>>,
    #[prop(into)] options: MaybeSignal<Vec<(T, U)>>,
    #[prop(into)] selected: ReadSignal<T>,
    #[prop(into)] set_selected: WriteSignal<T>,
) -> impl IntoView
where
    T: Clone + FromStr<Err: Debug> + Into<&'static str> + PartialEq + 'static,
    U: IntoView + Clone + 'static,
{
    let on_change = move |ev| {
        let new_selected = event_target_value(&ev);
        set_selected(new_selected.parse().unwrap());
    };

    let each = move || options.get().into_iter().enumerate();

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
