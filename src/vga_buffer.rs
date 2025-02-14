use lazy_static::lazy_static;
use core::fmt::{self};
use volatile::Volatile;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
#[repr(u8)]
enum Color {
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
    White = 15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorDesc(u8);
impl ColorDesc {
    fn new(foreground: Color, background: Color) -> ColorDesc {
        ColorDesc((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Char {
    character: u8,
    color_desc: ColorDesc,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<Char>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
    row_pos: usize,
    column_pos: usize,
    color_desc: ColorDesc,
    buff: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_pos;
                let col = self.column_pos;
                let color = self.color_desc;

                self.buff.chars[row][col].write(Char {
                    character: byte,
                    color_desc: color,
                });

                self.column_pos += 1;
            }
        }
    }

    fn is_print(&self, byte: u8) -> bool {
        match byte {
            0x20..=0x7e | b'\n' => true, // valid ascii
            _ => false
        }
    } 

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            if self.is_print(byte) {
                self.write_byte(byte);
            } else {
                self.write_byte(0xfe);
            }
        }
    }

    pub fn new_line(&mut self) {
        self.row_pos+=1;
        self.column_pos = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        row_pos: 0,
        column_pos: 0,
        color_desc: ColorDesc::new(Color::Yellow, Color::Black),
        buff: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

