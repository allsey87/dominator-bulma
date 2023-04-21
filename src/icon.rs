use dominator::{DomBuilder, Dom};
use web_sys::HtmlElement;

// TODO go back to the solution where the DomBuilder is created directly and just expose useful methods
// in the context of bulma components - the aim here is not to exposure all of DomBuilder methods.
// REMOVE 

pub enum Size {
    S18,
    S24,
    S36,
    S48,
}

impl Size {
    fn as_class(&self) -> &'static str {
        match self {
            Size::S18 => "mdi-18px",
            Size::S24 => "mdi-24px",
            Size::S36 => "mdi-36px",
            Size::S48 => "mdi-48px",
        }
    }
}

pub enum Rotate {
    R45,
    R90,
    R180,
}

impl Rotate {
    fn as_class(&self) -> &'static str {
        match self {
            Self::R45 => "mdi-rotate-45",
            Self::R90 => "mdi-rotate-90",
            Self::R180 => "mdi-rotate-180",
        }
    }
}

pub enum Flip {
    Horizontal,
    Vertical,
}

impl Flip {
    fn as_class(&self) -> &'static str {
        match self {
            Self::Horizontal => "mdi-flip-h",
            Self::Vertical => "mdi-flip-v",
        }
    }
}

pub struct Icon {
    outer: DomBuilder<HtmlElement>,
    inner: DomBuilder<HtmlElement>,
}

impl Icon {
    pub fn new(name: &str) -> Self {
        Self {
            outer: DomBuilder::new_html("span")
                .class("icon"),
            inner: DomBuilder::new_html("i")
                .class(["mdi", &format!("mdi-{}", name)])
        }
    }

    pub fn color(mut self, color: crate::color::Color, light: bool) -> Self {
        self.outer = self.outer
            .class(&format!("has-text-{}{}", color.as_class(), match light {
                true => "-light",
                false => ""
            }));
        self
    }

    pub fn visibility(mut self, visibility: crate::Visibility) -> Self {
        self.outer = self.outer
            .class(visibility.as_class());
        self
    }

    pub fn container_size(mut self, size: crate::Size) -> Self {
        self.outer = self.outer
            .class(size.as_class());
        self
    }

    pub fn icon_size(mut self, size: Size) -> Self {
        self.inner = self.inner
            .class(size.as_class());
        self
    }

    pub fn flip(mut self, flip: Flip) -> Self {
        self.inner = self.inner
            .class(flip.as_class());
        self
    }

    pub fn rotate(mut self, rotate: Rotate) -> Self {
        self.inner = self.inner
            .class(rotate.as_class());
        self
    }

    // trait method? visibility pub(crate)?
    pub fn class(mut self, class: &str) -> Self {
        self.outer = self.outer.class(class);
        self
    }

    pub fn into_dom(self) -> Dom {
        self.outer
            .child(self.inner.into_dom())
            .into_dom()
    }
}

pub struct IconText {
    builder: DomBuilder<HtmlElement>,
}

impl IconText {
    pub fn new() -> Self {
        Self {
            builder: DomBuilder::new_html("span")
                .class("icon-text")
        }
    }

    pub fn color(mut self, color: crate::color::Color, light: bool) -> Self {
        self.builder = self.builder
            .class(&format!("has-text-{}{}", color.as_class(), match light {
                true => "-light",
                false => ""
            }));
        self
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.builder = self.builder.child(icon.into_dom());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        let text = DomBuilder::<HtmlElement>::new_html("span")
            .text(text)
            .into_dom();
        self.builder = self.builder.child(text);
        self
    }

    pub fn into_dom(self) -> Dom {
        self.builder.into_dom()
    }
}
