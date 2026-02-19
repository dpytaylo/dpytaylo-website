use std::iter;

use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::utils::Pausable;
use leptos_use::{UseElementSizeReturn, use_element_size, use_raf_fn};
use web_sys::HtmlDivElement;

use crate::utils::{CssUnit, ToPixels};

const ONE_COLUMN_MAX_WIDTH_PX: f64 = 640.0; // max-w-screen-sm

#[component]
pub fn Masonry<Gap>(
    #[prop(into)] max_card_width_px: f64,
    gap: Gap,
    children: ChildrenFragment,
) -> impl IntoView
where
    Gap: CssUnit + ToPixels + Send + Sync + 'static,
{
    let (loaded, set_loaded) = signal(false);
    let Pausable { pause, .. } = use_raf_fn(move |_| {
        set_loaded(true);
    });

    Effect::new(move || {
        if loaded() {
            pause();
        }
    });

    let container = NodeRef::<Div>::new();
    let UseElementSizeReturn { width, .. } = use_element_size(container);

    let (nodes, elements): (Vec<_>, Vec<_>) = children()
        .nodes
        .into_iter()
        .map(|child| {
            let node_ref = NodeRef::new();
            (node_ref, view! { <div node_ref=node_ref>{child}</div> })
        })
        .unzip();

    let column_cards = RwSignal::new(iter::repeat(0usize).zip(nodes).collect::<Vec<_>>());

    let gap_px = gap.to_pixels();
    let exp_column_count = Memo::new(move |_| {
        let gap_px = gap_px();

        ((width() + gap_px.0) / (max_card_width_px + gap_px.0))
            .max(1.0)
            .floor() as usize
    });

    let (column_node_refs, set_column_node_refs) = signal(vec![NodeRef::new()]);
    Effect::new(move |_| {
        set_column_node_refs.update(|x| x.resize(exp_column_count(), NodeRef::new()));
    });

    let all_node_refs_were_set = move || column_node_refs.read().iter().all(|x| x.get().is_some());

    Effect::new(move || {
        if !all_node_refs_were_set() {
            return;
        }

        let column_nodes = column_node_refs.read();

        let exp_count = exp_column_count();
        let mut heights = vec![0i64; exp_count];

        let mut column_cards = column_cards.write_untracked();
        for (idx, card_node_ref) in column_cards.iter_mut() {
            let i = heights
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0;

            let card_node = card_node_ref.get().unwrap();
            let height = card_node.scroll_height();

            let new_column: HtmlDivElement = column_nodes[i].get().unwrap();
            new_column.append_child(&card_node).unwrap();

            heights[i] += i64::from(height);
            *idx = i;
        }
    });

    let column_style = move || {
        let count = exp_column_count();
        let gap = gap.clone();
        let max_card_width_px = max_card_width_px.clone();
        move || {
            if count == 1 {
                format!("gap: {gap}; max-width: {ONE_COLUMN_MAX_WIDTH_PX}px")
            } else {
                format!("gap: {gap}; max-width: {max_card_width_px}px")
            }
        }
    };

    let other_columns = move || {
        column_node_refs
            .read()
            .iter()
            .skip(1)
            .map(|node_ref| {
                let node_ref = node_ref.to_owned();
                view! {
                     <div
                        class="flex w-full flex-col"
                        style=column_style
                        node_ref=node_ref
                    >
                    </div>
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <div
            class="flex justify-center"
            style=format!("gap: {gap}")
            node_ref=container
        >
            <div
                class="flex w-full flex-col"
                style=column_style
                node_ref=column_node_refs.read_untracked().first().unwrap().clone()
            >
                {elements}
            </div>
            {other_columns}
        </div>
    }
}
