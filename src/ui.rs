use core::time::Duration;

use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_6X10, iso_8859_10::FONT_10X20},
    pixelcolor::Bgr888,
    prelude::{Point, RgbColor},
    text::Text,
};
use uefi::{boot::stall, proto::console::text::Key, system::with_stdin};

use crate::{
    get_num_string,
    memalloc::{free, malloc_lit},
    screen::Screen,
    strutils::strcat,
};

const STYLE: MonoTextStyle<'_, Bgr888> = MonoTextStyle::new(&FONT_10X20, Bgr888::GREEN);

pub struct GameState<'a> {
    screen: Screen<'a>,
    curr_pos: usize,
    num: u64,
}

impl<'a> GameState<'a> {
    pub fn new<'b: 'a>(screen: Screen<'b>) -> Self {
        Self {
            screen,
            curr_pos: 50,
            num: 10,
        }
    }
    pub fn reset(&mut self) {
        self.screen.reset();
        self.curr_pos = 50;
    }
    pub fn run_game(&mut self) -> ! {
        loop {
            // let key = with_stdin(|x| x.read_key()).unwrap();
            // if let Some(k) = key
            //     && let Key::Printable(ch) = k
            // {
            //     self.newline_write_literal("HI");
            //     (ch as char).as_ascii().unwrap().
            // }
            self.do_frame();
            self.num += 1;
            stall(Duration::from_millis(50));
        }
    }
    pub fn do_frame(&mut self) {
        self.reset();
        self.newline_write_literal("WhyAmIDoingThis ver 1");
        self.newline_write(strcat(malloc_lit("You have:"), get_num_string(self.num)));
    }
    //frees the string
    pub fn newline_write(&mut self, string: &'static str) {
        Text::new(string, Point::new(50, self.curr_pos as i32), STYLE).draw(&mut self.screen);
        free(string.as_bytes());
        self.curr_pos += 25;
    }
    pub fn newline_write_literal(&mut self, string: &'static str) {
        Text::new(string, Point::new(50, self.curr_pos as i32), STYLE).draw(&mut self.screen);
        self.curr_pos += 25;
    }
}
