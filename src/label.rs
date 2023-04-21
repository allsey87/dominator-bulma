use dominator::{DomBuilder};
use web_sys::HtmlElement;

pub struct Label {
    builder: DomBuilder<HtmlElement>,
}

impl Into<dominator::Dom> for Label {
    fn into(self) -> dominator::Dom {
        self.builder.into_dom()
    }
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            builder: DomBuilder::new_html("label")
                .class("label")
                .text(text),
        }
    }
}
