
#[macro_export]
macro_rules! field {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("div" => web_sys::HtmlDivElement, {
            .class("field")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! label {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("label" => web_sys::HtmlLabelElement, {
            .class("label")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! control {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("div" => web_sys::HtmlDivElement, {
            .class("control")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! input {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("input" => web_sys::HtmlInputElement, {
            .class("input")
            $(.class($classes))*
            $($methods)*
        })
    };
}

#[macro_export]
macro_rules! textarea {
    ($($classes:literal,)* { $($methods:tt)* }) => {
        dominator::html!("textarea" => web_sys::HtmlTextAreaElement, {
            .class("textarea")
            $(.class($classes))*
            $($methods)*
        })
    };
}