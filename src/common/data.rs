use std::fmt::{Debug, Formatter};
use smart_leds::{RGB, RGB8};
use smart_leds::hsv::Hsv;
pub const GREEN_STATE: Color = Color::Rgb(RGB8::new(0, 10, 0));
pub const YELLOW_STATE: Color = Color::Rgb(RGB8::new(10, 10, 0));
pub const BLUE_STATE: Color = Color::Rgb(RGB8::new(0, 0, 10));
pub const RED_STATE: Color = Color::Rgb(RGB8::new(10, 0, 0));
pub const BLACK_STATE: Color = Color::Rgb(RGB8::new(0, 0, 0));


#[derive(Copy, Clone)]
pub enum Color {
    Rgb(RGB<u8>),
    Hsv(Hsv),
}

#[derive(Copy, Clone, Debug)]
pub enum Gradient {
    Rgb(u8, u8, u8, u8),
    Hsv(u8, u8, u8, u8, u8),
}
#[derive(Copy, Clone)]
pub enum State {
    Color(Color),
    Gradient(Gradient),
    Message(&'static str)
}

impl Debug for State{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "State(")?;
        match self {
            State::Color(color) => {
                write!(f, "Color(")?;
                match color {
                    Color::Rgb(r) => {
                        write!(f, "{:?}", r)?;
                    }
                    Color::Hsv(h) => {
                        write!(f, "HSV({}, {}, {})", h.hue, h.val, h.sat)?;
                    }
                }
                write!(f, ")")?;
            }
            State::Gradient(g) => {
                write!(f, "Gradient({:?})", g)?;
            }
            State::Message(s) => {
                write!(f, "{}", s)?;
            }
        };
        write!(f, ")")
    }
}