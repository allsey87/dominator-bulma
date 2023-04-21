use dominator::{DomBuilder, Dom};
use futures_signals::signal_vec::{SignalVec, SignalVecExt};
use web_sys::HtmlElement;

pub struct Field {
    builder: DomBuilder<HtmlElement>,
}

impl Into<dominator::Dom> for Field {
    fn into(self) -> dominator::Dom {
        self.builder.into_dom()
    }
}

impl Field {
    pub fn new () -> Self {
        Self {
            builder: DomBuilder::new_html("div")
                .class("field"),
        }
    }

    pub fn grouped(mut self) -> Self {
        self.builder = self.builder.class("is-grouped");
        self
    }

    pub fn grouped_multiline(mut self) -> Self {
        self.builder = self.builder.class("is-grouped-multiline");
        self
    }

    pub fn label(mut self, text: &str) -> Self {
        let label = DomBuilder::<HtmlElement>::new_html("label")
            .class("label")
            .text(text);
        self.builder = self.builder.child(label.into_dom());
        self
    }

    pub fn control(mut self, control: crate::Control) -> Self {
        self.builder = self.builder.child(control.into_dom());
        self
    }

    pub fn controls_signal_vec(
        mut self,
        signal_vec: impl SignalVec<Item = crate::Control> + 'static
    ) -> Self {
        self.builder = self.builder.children_signal_vec(signal_vec.map(|item| item.into_dom()));
        self
    } 

    pub fn help(mut self, text: &str, color: crate::color::Color) -> Self {
        let label = DomBuilder::<HtmlElement>::new_html("p")
            .class("help")
            .class(color.as_class())
            .text(text);
        self.builder = self.builder.child(label.into_dom());
        self
    }
}

