extern crate image;

//use std::fs::File;
//use std::path::Path;
//use std::vec::Vec;

//use image::ImageBuffer;
//use image::Pixel;

const _WHITE: image::Rgb<u8> = image::Rgb([255, 255, 255]);
const _BLACK: image::Rgb<u8> = image::Rgb([0, 0, 0]);

const RED: image::Rgb<u8> = image::Rgb([255, 0, 0]);
const _GREEN: image::Rgb<u8> = image::Rgb([0, 255, 0]);
const _BLUE: image::Rgb<u8> = image::Rgb([0, 0, 255]);

const _YELLOW: image::Rgb<u8> = image::Rgb([255, 255, 0]);
const _MAGENTA: image::Rgb<u8> = image::Rgb([255, 0, 255]);
const _CYAN: image::Rgb<u8> = image::Rgb([0, 255, 255]);

fn draw_line<T: image::GenericImage>(
    x0: &u32,
    y0: &u32,
    x1: &u32,
    y1: &u32,
    img: &mut T,
    color: T::Pixel,
) {
    let steps = 10; //actually 1 more step than this
    for t in 0..=steps {
        let x = x0 + (((x1 - x0) as f32 * (t as f32) / (steps as f32)) as u32);
        let y = y0 + (((y1 - y0) as f32 * (t as f32) / (steps as f32)) as u32);
        img.put_pixel(x, y, color);
    }
}

fn main() {
    let mut img = image::ImageBuffer::new(100, 100);

    draw_line(&10, &10, &60, &25, &mut img, RED);

    let img = image::imageops::flip_vertical(&img);

    img.save("./output/pixel5.tga").unwrap();
}
