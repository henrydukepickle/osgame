#![no_std]
#![no_main]

mod memalloc;
mod screen;
mod strutils;
mod ui;

use core::slice;

use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::{Bgr888, Rgb888},
    prelude::{Dimensions, Point, RgbColor, Size},
    primitives::Rectangle,
    text::{Text, TextStyle},
};
use uefi::{
    CStr16, Handle, Status,
    boot::{
        EventType, ScopedProtocol, Tpl, create_event, exit_boot_services, get_handle_for_protocol,
        open_protocol_exclusive, stall,
    },
    entry,
    proto::console::gop::{FrameBuffer, GraphicsOutput, Mode, PixelFormat},
    system::with_stdout,
    table::system_table_raw,
};

use crate::{
    memalloc::{MemAllocator, malloc},
    ui::GameState,
};

macro_rules! write_to_console {
    ($string:expr) => {
        let mut buf: [u16; 64] = [0; 64];
        with_stdout(|out| {
            out.output_string(CStr16::from_str_with_buf($string, buf.as_mut_slice()).unwrap())
        })
        .unwrap();
    };
    ($string:expr, $buf_size:literal) => {
        let mut buf: [u16; $buf_size] = [0; $buf_size];
        with_stdout(|out| {
            out.output_string(CStr16::from_str_with_buf($string, buf.as_mut_slice()).unwrap())
        })
        .unwrap();
    };
}
#[panic_handler]
fn panic<'a>(info: &core::panic::PanicInfo<'a>) -> ! {
    write_to_console!("panic!");
    write_to_console!(info.message().as_str().unwrap(), 600);
    loop {}
}

#[entry]
fn main() -> Status {
    write_to_console!("Starting..");
    write_to_console!("Allocator Created..");
    let gop_handle = get_handle_for_protocol::<GraphicsOutput>().unwrap();
    write_to_console!("GOP Handle Found..");
    let gop_result: Result<ScopedProtocol<GraphicsOutput>, uefi::Error> =
        open_protocol_exclusive(gop_handle);
    if let Err(ref e) = gop_result {
        match e.status() {
            Status::UNSUPPORTED => {
                write_to_console!("Unsupported.");
            }
            Status::ACCESS_DENIED => {
                write_to_console!("Access Denied.");
            }
            _ => {}
        }
    }
    let mut gop = gop_result.unwrap();
    write_to_console!("GOP Accessed..");
    let best_mode = get_best_mode(&gop).unwrap();
    gop.set_mode(&best_mode).unwrap();
    let mut fb = gop.frame_buffer();
    let ptr = fb.as_mut_ptr();
    let mut screen = screen::Screen {
        _buffer: fb,
        ptr: ptr as *mut Bgr888,
        size: best_mode.info().resolution(),
        stride: best_mode.info().stride(),
    };
    let mut game = GameState::new(screen);
    game.run_game()
}

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn get_best_mode(gop: &ScopedProtocol<GraphicsOutput>) -> Option<Mode> {
    let mut best = (0, 0);
    let mut curr_mode = None;
    for mode in gop.modes() {
        if mode.info().pixel_format() == PixelFormat::Bgr
            && mode.info().resolution().0 <= WIDTH
            && mode.info().resolution().1 <= HEIGHT
            && mode.info().resolution().0 > best.0
        {
            best = mode.info().resolution();
            curr_mode = Some(mode)
        }
    }
    curr_mode
}

fn get_num_string(num: u64) -> &'static str {
    if num == 0 {
        let ret = malloc(1);
        ret[0] = b'0';
        return str::from_utf8(ret).unwrap();
    }
    let mut running = num;
    let length = max_ten_pow(num) as usize;
    let mut ind = length - 1;
    let ret = malloc(length);
    loop {
        ret[ind] = ((running % 10) as u8) + b'0';
        running /= 10;
        if ind == 0 {
            break;
        }
        ind -= 1;
    }
    str::from_utf8(ret).unwrap()
}
//max power of ten strictly less
//number of digits in num
fn max_ten_pow(num: u64) -> u16 {
    let mut test: u16 = 0;
    if num == 0 {
        return 1;
    }
    loop {
        if num < 10_u64.pow(test.into()) {
            break test;
        }
        test += 1;
    }
}
