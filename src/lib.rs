pub mod button;
pub mod label;
pub mod level;
pub mod form;
pub mod icon;
pub mod text;
pub mod color;
pub mod error;

pub use button::Button;
pub use level::Level;
pub use icon::{Icon, IconText};
pub use form::{checkbox::Checkbox, control::Control, field::Field, input::Input, select::Select};
pub use label::Label;
pub use text::Text;
pub use color::Color;

/* this helper should be available on all classes */
#[derive(Debug, Clone, Copy)]
pub enum Visibility {
    HiddenMobile,
    HiddenTabletOnly,
    HiddenDesktopOnly,
    HiddenWidescreenOnly,
    HiddenTouch,
    HiddenTablet,
    HiddenDesktop,
    HiddenWidescreen,
    HiddenFullHd
}

impl Visibility {
    fn as_class(&self) -> &'static str {
        match self {
            Visibility::HiddenMobile => "is-hidden-mobile",
            Visibility::HiddenTabletOnly => "is-hidden-tablet-only",
            Visibility::HiddenDesktopOnly => "is-hidden-desktop-only",
            Visibility::HiddenWidescreenOnly => "is-hidden-widescreen-only",
            Visibility::HiddenTouch => "is-hidden-touch",
            Visibility::HiddenTablet => "is-hidden-tablet",
            Visibility::HiddenDesktop => "is-hidden-desktop",
            Visibility::HiddenWidescreen => "is-hidden-widescreen",
            Visibility::HiddenFullHd => "is-hidden-fullhd",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl Size {
    fn as_class(&self) -> &'static str {
        match self {
            Size::Small => "is-small",
            Size::Normal => "is-normal",
            Size::Medium => "is-medium",
            Size::Large => "is-large",
        }
    }
}