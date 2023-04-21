use dominator::{DomBuilder, Dom};
use futures_signals::signal::{Signal, SignalExt};
use strum::IntoEnumIterator;
use web_sys::HtmlElement;

use crate::{error::Error, color::{Variant, Color}};

// TODO go back to the solution where the DomBuilder is created directly and just expose useful methods
// in the context of bulma components - the aim here is not to exposure all of DomBuilder methods.
// REMOVE 


impl Color {
    pub fn as_text_color_class(&self, variant: &Option<Variant>) -> Result<&'static str, Error> {
        match variant {
            None => match self {
                Color::White => Ok("has-text-white"),
                Color::Light => Ok("has-text-light"),
                Color::Grey => Ok("has-text-grey"),
                Color::Dark => Ok("has-text-dark"),
                Color::Black => Ok("has-text-black"),
                Color::Primary => Ok("has-text-primary"),
                Color::Link => Ok("has-text-link"),
                Color::Info => Ok("has-text-info"),
                Color::Success => Ok("has-text-success"),
                Color::Warning => Ok("has-text-warning"),
                Color::Danger => Ok("has-text-danger"),
            }
            Some(Variant::Light) => match self {
                Color::Grey => Ok("has-text-grey-light"),
                Color::Primary => Ok("has-text-primary-light"),
                Color::Link => Ok("has-text-link-light"),
                Color::Info => Ok("has-text-info-light"),
                Color::Success => Ok("has-text-success-light"),
                Color::Warning => Ok("has-text-warning-light"),
                Color::Danger => Ok("has-text-danger-light"),
                _ => Err(Error::BadVariant)
            }
            Some(Variant::Dark) => match self {
                Color::Grey => Ok("has-text-grey-dark"),
                Color::Primary => Ok("has-text-primary-dark"),
                Color::Link => Ok("has-text-link-dark"),
                Color::Info => Ok("has-text-info-dark"),
                Color::Success => Ok("has-text-success-dark"),
                Color::Warning => Ok("has-text-warning-dark"),
                Color::Danger => Ok("has-text-danger-dark"),
                _ => Err(Error::BadVariant)
            }
        }
    }
}

pub struct Text {
    builder: DomBuilder<HtmlElement>,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            builder: DomBuilder::new_html("p")
                .text(text),
        }
    }

    pub fn color(mut self, color: Color, variant: Option<Variant>) -> Self {
        self.builder = self.builder
            .class(color.as_text_color_class(&variant).unwrap());
        self
    }

    pub fn color_signal<S: Signal<Item = (Color, Option<Variant>)> + 'static>(mut self, signal: S) -> Self {
        let signal = SignalExt::broadcast(signal);
        let color_variants = Color::iter()
            .map(|color| [
                (color, None, color.as_text_color_class(&None)),
                (color, Some(Variant::Light), color.as_text_color_class(&Some(Variant::Light))),
                (color, Some(Variant::Dark), color.as_text_color_class(&Some(Variant::Dark))),
            ]
            .into_iter()
            .filter_map(|(color, variant, as_str)| as_str.ok().map(|as_str| (color, variant, as_str))))
            .flatten();
        self.builder = color_variants.fold(self.builder, |builder, (color, variant, as_str)| {
            builder.class_signal(as_str, signal.signal_ref(move |(color_signal, variant_signal)| {
                (color_signal == &color) && (variant_signal == &variant)
            }))
        });
        self
    }

    pub fn visibility(mut self, visibility: crate::Visibility) -> Self {
        self.builder = self.builder
            .class(visibility.as_class());
        self
    }

    pub fn into_dom(self) -> Dom {
        self.builder.into_dom()
    }
}
