use leptos::html::Div;
use leptos::*;
use leptos_use::{use_element_size, UseElementSizeReturn};

use crate::utils::{CssUnit, ToPixels};

const ONE_COLUMN_MAX_WIDTH_PX: f64 = 640.0; // max-w-screen-sm

#[component]
pub fn Masonry<T>(
    #[prop(into)] elements: Vec<HtmlElement<Div>>,
    #[prop(into)] max_card_width_px: f64,
    gap: T,
) -> impl IntoView
where
    T: CssUnit + ToPixels + 'static,
{
    let (loaded, set_loaded) = create_signal(false);
    create_effect(move |_| {
        request_animation_frame(move || set_loaded(true));
    });

    let container = create_node_ref::<Div>();

    let UseElementSizeReturn { width, .. } = use_element_size(container);

    let gap_px = gap.to_pixels();
    let column_count = create_memo(move |_| {
        let gap_px = gap_px();

        ((width() + gap_px.0) / (max_card_width_px + gap_px.0))
            .max(1.0)
            .floor() as usize
    });

    let columns = move || {
        let elements = elements.clone();
        let gap = gap.clone();

        if !loaded() {
            return view! {
                <div
                    class="flex w-full flex-col"
                    style=format!("max-width: {ONE_COLUMN_MAX_WIDTH_PX}px; gap: {gap}")
                >
                    {elements}
                </div>
            }
            .into_view();
        }

        let count = column_count();

        let mut heights = vec![0i64; count];
        let mut columns = vec![vec![]; count];

        for element in elements {
            let i = heights
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0;

            heights[i] += i64::from(element.scroll_height());
            columns[i].push(element);
        }

        let column_style = move || {
            if count == 1 {
                format!("gap: {gap}; max-width: {ONE_COLUMN_MAX_WIDTH_PX}px")
            } else {
                format!("gap: {gap}; max-width: {max_card_width_px}px")
            }
        };

        columns
            .into_iter()
            .map(|cards| {
                view! {
                    <div
                        class="flex w-full flex-col"
                        style=column_style
                    >
                        {cards}
                    </div>
                }
            })
            .collect_view()
    };

    view! {
        <div
            class="flex justify-center"
            style=format!("gap: {gap}")
            node_ref=container
        >
            {columns}
        </div>
    }
}
