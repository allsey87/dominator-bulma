use std::borrow::BorrowMut;

use dominator::{DomBuilder, events, Dom};
use futures_signals::{signal::Signal, signal_vec::SignalVec};
use web_sys::{HtmlElement, HtmlSelectElement};

pub struct Select {
    builder_outer: DomBuilder<HtmlElement>,
    builder_inner: DomBuilder<HtmlSelectElement>,
}

impl Into<dominator::Dom> for Select {
    fn into(self) -> dominator::Dom {
        self.builder_outer
            .child(self.builder_inner.into_dom())
            .into_dom()
    }
}

impl Select {
    pub fn new() -> Self {
        Self {
            builder_outer: DomBuilder::new_html("div")
                .class("select"),
            builder_inner: DomBuilder::new_html("select"),
        }
    }

    // escape hatch: this should be present on all classes
    // will need to be added manually since components can have multiple builders
    pub fn apply_outer(
        mut self,
        mut apply: impl FnMut(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>
    ) -> Self {
        self.builder_outer = apply(self.builder_outer);
        self
    }

    pub fn apply_inner(
        mut self,
        mut apply: impl FnMut(DomBuilder<HtmlSelectElement>) -> DomBuilder<HtmlSelectElement>
    ) -> Self {
        self.builder_inner = apply(self.builder_inner);
        self
    }

    pub fn fullwidth(mut self) -> Self {
        self.builder_outer = self.builder_outer
            .class("is-fullwidth");
        self
    }

    pub fn rounded(mut self) -> Self {
        self.builder_outer = self.builder_outer
            .class("is-rounded");
        self
    }

    pub fn multiple(mut self, size: u32) -> Self {
        self.builder_outer = self.builder_outer
            .class("is-multiple");
        self.builder_inner = self.builder_inner
            .attr("multiple", "")
            .attr("size", &size.to_string());
        self
    }

    pub fn option<B>(mut self, option: B) -> Self
    where
        B: BorrowMut<Dom> {
        self.builder_inner = self.builder_inner
            .child(option);
        self
    }

    pub fn option_signal<B>(mut self, option: B) -> Self
    where
        B: Signal<Item = Option<Dom>> + 'static {
        self.builder_inner = self.builder_inner
            .child_signal(option);
        self
    }

    pub fn options<B, C>(mut self, options: C) -> Self
    where
        B: BorrowMut<Dom>,
        C: IntoIterator<Item = B> {
        self.builder_inner = self.builder_inner
            .children(options);
        self
    }

    pub fn options_signal<B>(mut self, options: B) -> Self
    where
        B: SignalVec<Item = Dom> + 'static {
        self.builder_inner = self.builder_inner
            .children_signal_vec(options);
        self
    }

    pub fn disable_signal<S: Signal<Item = bool> + 'static>(mut self, disable: S) -> Self {
        self.builder_inner = self.builder_inner
            .prop_signal("disabled", disable);
        self
    }

    pub fn on_change<F>(mut self, mut handler: F) -> Self
    where F: FnMut(events::Change, &HtmlSelectElement) + 'static {
        let element = DomBuilder::__internal_element(&self.builder_inner);
        self.builder_inner = self.builder_inner.event(move |event| handler(event, &element));
        self
    }
}