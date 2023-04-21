use dominator::{DomBuilder, events, Dom};
use futures_signals::signal::{Signal, SignalExt};
use strum::IntoEnumIterator;
use web_sys::HtmlElement;

use crate::color::Color;

pub struct Button {
    builder: DomBuilder<HtmlElement>
}

impl Into<dominator::Dom> for Button {
    fn into(self) -> dominator::Dom {
        self.builder.into_dom()
    }
}

impl Button {
    pub fn new() -> Self {
        Self {
            builder: DomBuilder::new_html("button")
                .class("button")
        }
    }

    pub fn fullwidth(mut self) -> Self {
        self.builder = self.builder.class("is-fullwidth");
        self
    }

    pub fn rounded(mut self) -> Self {
        self.builder = self.builder.class("is-rounded");
        self
    }

    pub fn apply<F>(mut self, function: F) -> Self where
        F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        self.builder = self.builder.apply(function);
        self
    }

    pub fn label(mut self, text: &str) -> Self {
        self.builder = self.builder.text(text);
        self
    }

    pub fn color(mut self, color: Color, light: bool) -> Self {
        self.builder = self.builder
            .class(color.as_class())
            .apply_if(light, |dom| dom.class("is-light"));
        self
    }

    pub fn color_signal<S: Signal<Item = (Color, bool)> + 'static>(mut self, signal: S) -> Self {
        let signal = SignalExt::broadcast(signal);
        self.builder = Color::iter().fold(self.builder, |builder, color| {
            builder.class_signal(color.as_class(), signal.signal_ref(move |(color_signal, _)| {
                    color_signal == &color
            }))
        });
        self.builder = self.builder
            .class_signal("is-light", signal.signal_ref(|(_, light)| *light));
        self
    }

    pub fn on_click(mut self, event: impl FnMut(events::Click) + 'static) -> Self {
        self.builder = self.builder.event(event);
        self
    }

    pub fn disable_signal<S: Signal<Item = bool> + 'static>(mut self, signal: S) -> Self {
        self.builder = self.builder.prop_signal("disabled", signal);
        self
    }
}

