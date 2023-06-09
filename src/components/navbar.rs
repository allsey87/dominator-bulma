use std::sync::Arc;

use dominator::{html, Dom, clone, events, DomBuilder};
use futures_signals::signal::Mutable;

// TODO create a builder for this component
/* fixed_top and fixed_bottom methods take dominator::body() so that we can 
   add the appropiate classes to the body
*/ 
#[derive(Default)]
pub struct Component;

impl Component {

    pub fn render(
        mixin: impl FnOnce(DomBuilder<web_sys::HtmlElement>) -> DomBuilder<web_sys::HtmlElement>,
        brand_mixin: impl FnOnce(DomBuilder<web_sys::HtmlElement>) -> DomBuilder<web_sys::HtmlElement>,
        start_mixin: impl FnOnce(DomBuilder<web_sys::HtmlElement>) -> DomBuilder<web_sys::HtmlElement>,
        end_mixin: impl FnOnce(DomBuilder<web_sys::HtmlElement>) -> DomBuilder<web_sys::HtmlElement>
    ) -> Dom {
        let show_menu: Mutable<bool> = Default::default();

        html!("nav", {
            .class("navbar")
            .apply(mixin)
            .attr("role", "navigation")
            .attr("aria-label", "main navigation")
            .child(Self::render_brand(brand_mixin, &show_menu))
            .child(Self::render_menu(start_mixin, end_mixin, &show_menu))
        })
    }

    fn render_brand(
        mixin: impl FnOnce(DomBuilder<web_sys::HtmlElement>) -> DomBuilder<web_sys::HtmlElement>,
        show_menu: &Mutable<bool>
    ) -> Dom {
        html!("div", {
            .class("navbar-brand")
            .apply(mixin)
            .child(html!("a", {
                .class("navbar-burger")
                .attr("role", "button")
                .attr("aria-label", "menu")
                .attr("aria-expanded","false")
                .child(html!("span", { .attr("aria-hidden", "true") }))
                .child(html!("span", { .attr("aria-hidden", "true") }))
                .child(html!("span", { .attr("aria-hidden", "true") }))
                .class_signal("is-active", show_menu.signal())
                .event(clone!(show_menu => move |_: events::Click| {
                    show_menu.replace_with(|&mut show| !show);
                }))
            }))
        })
    }

    fn render_menu(
        start_mixin: impl FnOnce(DomBuilder<web_sys::HtmlElement>) -> DomBuilder<web_sys::HtmlElement>,
        end_mixin: impl FnOnce(DomBuilder<web_sys::HtmlElement>) -> DomBuilder<web_sys::HtmlElement>,
        show_menu: &Mutable<bool>
    ) -> Dom {
        html!("div", {
            .class("navbar-menu")
            .class_signal("is-active", show_menu.signal())
            .event(clone!(show_menu => move |_: events::Click| {
                show_menu.replace_with(|&mut show| !show);
            }))
            .child(html!("div", {
                .class("navbar-start")
                .apply(start_mixin)
            }))
            .child(html!("div", {
                .class("navbar-end")
                .apply(end_mixin)
            }))
        })
    }

}
