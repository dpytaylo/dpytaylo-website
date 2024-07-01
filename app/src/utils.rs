use leptos::{create_effect, create_signal, document, window, ReadSignal};
use std::fmt::{Debug, Display};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Zero;

impl Display for Zero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("0px")
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Px(pub f64);

impl Display for Px {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}px", self.0))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Em(pub f64);

impl Display for Em {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}em", self.0))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rem(pub f64);

impl Display for Rem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}rem", self.0))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percent(pub f64);

impl Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}%", self.0))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Auto;

impl Display for Auto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Auto")
    }
}

pub trait CssUnit: Debug + Default + Display + Clone + Copy + PartialEq + PartialOrd {}
impl CssUnit for Zero {}
impl CssUnit for Px {}
impl CssUnit for Em {}
impl CssUnit for Rem {}
impl CssUnit for Percent {}
impl CssUnit for Auto {}

pub trait ToPixels {
    fn to_pixels(&self) -> ReadSignal<Px>;
}

impl ToPixels for Zero {
    fn to_pixels(&self) -> ReadSignal<Px> {
        let (pixels, _) = create_signal(Px::default());
        pixels
    }
}

impl ToPixels for Px {
    fn to_pixels(&self) -> ReadSignal<Px> {
        let (pixels, _) = create_signal(*self);
        pixels
    }
}

impl ToPixels for Rem {
    fn to_pixels(&self) -> ReadSignal<Px> {
        let (pixels, set_pixels) = create_signal(Px::default());

        let this = self.clone();
        create_effect(move |_| {
            let window = window();
            let document = document();

            let root_font_size = window
                .get_computed_style(&document.document_element().unwrap())
                .unwrap()
                .unwrap()
                .get_property_value("font-size")
                .unwrap()
                .trim_end_matches("px")
                .parse::<f64>()
                .unwrap();

            set_pixels(Px(root_font_size * this.0));
        });

        pixels
    }
}
