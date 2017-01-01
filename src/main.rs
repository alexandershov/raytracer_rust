extern crate bmp;

use bmp::{Image, Pixel};

fn main() {
    let mut image = Image::new(256, 256);
    draw_line(&mut image, 0, 200);
    image.save("/Users/aershov182/tmp/raytracer.bmp").expect("oops");
}

fn draw_line(image: &mut Image, start: u32, end: u32) {
    for i in start..end {
        image.set_pixel(i, i, Pixel { r: 255, g: 255, b: 255 });
    }
}
