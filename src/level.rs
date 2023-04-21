
use dominator::{DomBuilder, Dom, html};
use web_sys::HtmlElement;

// TODO go back to the solution where the DomBuilder is created directly and just expose useful methods
// in the context of bulma components - the aim here is not to exposure all of DomBuilder methods.
pub struct Level {
    left_builder: DomBuilder<HtmlElement>,
    right_builder: DomBuilder<HtmlElement>,
    builder: DomBuilder<HtmlElement>,
}

impl Into<dominator::Dom> for Level {
    fn into(self) -> dominator::Dom {
        self.builder
            .child(self.left_builder.into_dom())
            .child(self.right_builder.into_dom())
            .into_dom()
    }
}

impl Level {
    pub fn new() -> Self {
        Self {
            left_builder: DomBuilder::new_html("div")
                .class("level-left"),
            right_builder: DomBuilder::new_html("div")
                .class("level-right"),
            builder: DomBuilder::new_html("nav")
                .class("level"),
        }
    }

    pub fn mobile(mut self) -> Self {
        self.builder = self.builder.class("is-mobile");
        self
    }

    pub fn append_left<C>(mut self, component: C) -> Self
    where C: Into<Dom> {
        self.left_builder = self.left_builder.child(html!("div", {
            .class("level-item")
            .child(component.into())
        }));
        self
    }

    pub fn append_right<C>(mut self, component: C) -> Self
        where C: Into<Dom> {
        self.right_builder = self.right_builder.child(html!("div", {
            .class("level-item")
            .child(component.into())
        }));
        self
    }
}
