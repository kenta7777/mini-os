// reduce the warning for unused Color enum
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// each enum variant is stored as an u8
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// To ensure that the layout of background is the same as foreground 
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground) as u8)
    }
}

// Represent a screen character and the text buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// To order the fields of struct
#[repr(C)]
struct ScreenChar {
    ascii_character: u8, 
    color_code: ColorCode,
}

// The rows and columns of VGA text buffer
const BUFFER_HEIGHT: usize = 25
const BUFFER_WIDTH: usize = 80

// To ensure that this struct has the same memory layout as its single field 
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}