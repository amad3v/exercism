use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
use std::fmt;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ResistorColor> for String {
    fn from(value: ResistorColor) -> Self {
        format!("{}", value)
    }
}
pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(v) => v.into(),
        _ => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
