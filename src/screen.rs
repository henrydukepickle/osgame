use embedded_graphics::{
    pixelcolor::Bgr888,
    prelude::{Dimensions, Point, RgbColor, Size},
    primitives::Rectangle,
};
use uefi::proto::console::gop::FrameBuffer;

pub struct Screen<'a> {
    pub _buffer: FrameBuffer<'a>, //to ensure safety, so the lifetimes make sense
    pub ptr: *mut Bgr888,
    pub size: (usize, usize),
    pub stride: usize,
}

const LARGE_BLOCK_SIZE: usize = 1920;

impl<'a> Screen<'a> {
    pub fn in_bounds(&self, x: usize, y: usize, strict: bool) -> bool {
        if strict {
            x < self.size.0 && y < self.size.1
        } else {
            x <= self.size.0 && y <= self.size.1
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, pixel: Bgr888) -> Result<(), ()> {
        if self.in_bounds(x, y, true) {
            unsafe {
                self.ptr
                    .offset((x + (y * self.stride)) as isize)
                    .write(pixel);
            }
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn rectangle(
        &mut self,
        min: (usize, usize),
        max: (usize, usize),
        pixel: Bgr888,
        strict: bool,
    ) -> Result<(), ()> {
        if self.in_bounds(min.0, min.1, !strict)
            && self.in_bounds(max.0, max.1, !strict)
            && max.0 > min.0
            && max.1 > min.1
        {
            let pixel_data_block = [pixel; LARGE_BLOCK_SIZE];
            if strict {
                for y in (min.1)..(max.1) {
                    unsafe {
                        self.ptr
                            .offset((min.0 + (y * self.stride)) as isize)
                            .copy_from_nonoverlapping(
                                (&pixel_data_block) as *const Bgr888,
                                max.0 - min.0,
                            );
                    }
                }
            } else {
                for y in (min.1)..=(max.1) {
                    unsafe {
                        self.ptr
                            .offset((min.0 + (y * self.stride)) as isize)
                            .copy_from_nonoverlapping(
                                (&pixel_data_block) as *const Bgr888,
                                max.0 - min.0 + 1,
                            );
                    }
                }
            }
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn reset(&mut self) {
        self.rectangle((0, 0), self.size, Bgr888::BLACK, true);
    }
}

impl Dimensions for Screen<'_> {
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        Rectangle::new(
            Point::new(0, 0),
            Size::new(self.size.0 as u32, self.size.1 as u32),
        )
    }
}

impl embedded_graphics::draw_target::DrawTarget for Screen<'_> {
    type Color = embedded_graphics::pixelcolor::Bgr888;

    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for p in pixels {
            self.write_pixel(p.0.x as usize, p.0.y as usize, p.1)?;
        }
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        self.rectangle(
            (area.top_left.x as usize, area.top_left.y as usize),
            (
                area.bottom_right().ok_or(())?.x as usize,
                area.bottom_right().ok_or(())?.y as usize,
            ),
            color,
            false,
        )
    }
}
