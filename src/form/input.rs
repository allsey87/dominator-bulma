use dominator::{DomBuilder, events, Dom};
use futures_signals::signal::Signal;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;

pub struct Input {
    builder: DomBuilder<HtmlInputElement>,
}

impl Into<dominator::Dom> for Input {
    fn into(self) -> dominator::Dom {
        self.builder.into_dom()
    }
}

impl Input {
    pub fn new_text() -> Self {
        Self {
            builder: DomBuilder::new_html("input")
                .class("input")
                .attr("type", "text"),
        }
    } 

    pub fn new_password() -> Self {
        Self {
            builder: DomBuilder::new_html("input")
                .class("input")
                .attr("type", "password"),
        }
    }

    pub fn new_email() -> Self {
        Self {
            builder: DomBuilder::new_html("input")
                .class("input")
                .attr("type", "email"),
        }
    }

    pub fn new_telephone() -> Self {
        Self {
            builder: DomBuilder::new_html("input")
                .class("input")
                .attr("type", "tel"),
        }
    }

    pub fn placeholder(mut self, text: &str) -> Self {
        self.builder = self.builder.attr("placeholder", text);
        self
    }

    pub fn rounded(mut self) -> Self {
        self.builder = self.builder.class("is-rounded");
        self
    }

    pub fn on_keydown(
        self,
        handler: impl FnMut(events::KeyDown, &HtmlInputElement) + 'static
    ) -> Self {
        Self::on_keydown_with_options(self, &dominator::EventOptions::default(), handler)
    }

    pub fn on_keydown_with_options(
        mut self,
        options: &dominator::EventOptions,
        mut handler: impl FnMut(events::KeyDown, &HtmlInputElement) + 'static
    ) -> Self {
        let element = DomBuilder::__internal_element(&self.builder);
        self.builder = self.builder
            .event_with_options(options, move |event| handler(event, &element));
        self
    }

    pub fn disable_signal(
        mut self,
        signal: impl Signal<Item = bool> + 'static
    ) -> Self {
        self.builder = self.builder.prop_signal("disabled", signal);
        self
    }

    pub fn value_signal(
        mut self,
        signal: impl Signal<Item = impl Into<JsValue>> + 'static
    ) -> Self {
        self.builder = self.builder.prop_signal("value", signal);
        self
    }
}
