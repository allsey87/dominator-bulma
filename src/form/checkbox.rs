use dominator::{DomBuilder, Dom};
use futures_signals::signal::{Signal, SignalExt};
use web_sys::HtmlElement;

pub struct Checkbox {
    outer: DomBuilder<HtmlElement>,
    inner: DomBuilder<HtmlElement>,
    label: Option<String>,
}

impl Into<dominator::Dom> for Checkbox {
    fn into(self) -> dominator::Dom {
        self.outer
            .child(self.inner.into_dom())
            .apply_if(self.label.is_some(), |outer| {
                outer.text(&self.label.unwrap())
            })
            .into_dom()
    }
}

impl Checkbox {
    pub fn new() -> Self {
        Self {
            outer: DomBuilder::new_html("label")
                .class("checkbox"),
            inner: DomBuilder::new_html("input")
                .attr("type", "checkbox"),
            label: None
        }
    }

    pub fn label(mut self, text: &str) -> Self {
        self.label = Some(text.to_owned());
        self
    }

    pub fn disable_signal<S: Signal<Item = bool> + 'static>(mut self, signal: S) -> Self {
        let broadcaster = signal.broadcast();
        self.inner = self.inner.prop_signal("disabled", broadcaster.signal());
        self.outer = self.outer.prop_signal("disabled", broadcaster.signal());
        self
    }
}