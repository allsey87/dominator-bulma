use dominator::{DomBuilder, Dom};
use web_sys::HtmlElement;

pub struct Control {
    builder: DomBuilder<HtmlElement>,
}

impl Control {
    pub fn new() -> Self {
        Self {
            builder: DomBuilder::new_html("div")
                .class("control"),
        }
    }

    pub fn expanded(mut self) -> Self {
        self.builder = self.builder.class("is-expanded");
        self
    }

    pub fn component<C>(mut self, component: C) -> Self 
        where C: Into<Dom> {
        self.builder = self.builder.child(component.into());
        self
    }

    pub fn left_icon(mut self, icon: crate::Icon) -> Self {
        self.builder = self.builder
            .class("has-icons-left")
            .child(icon.class("is-left").into_dom());
        self
    }

    pub fn right_icon(mut self, icon: crate::Icon) -> Self {
        self.builder = self.builder
            .class("has-icons-right")
            .child(icon.class("is-right").into_dom());
        self
    }

    pub fn into_dom(self) -> Dom {
        self.builder.into_dom()
    }


}
