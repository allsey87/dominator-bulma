use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Variant {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Color {
    White,
    Light,
    Grey,
    Dark,
    Black,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger
}

impl Color {
    pub fn as_class(&self) -> &'static str {
        match self {
            Color::White => "is-white",
            Color::Light => "is-light",
            Color::Grey => "is-grey",
            Color::Dark => "is-dark",
            Color::Black => "is-black",
            Color::Primary => "is-primary",
            Color::Link => "is-link",
            Color::Info => "is-info",
            Color::Success => "is-success",
            Color::Warning => "is-warning",
            Color::Danger => "is-danger",
        }
    }
}