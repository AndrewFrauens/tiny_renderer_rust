#![allow(dead_code)]
extern crate image;

/// Tiny Renderer module
/// Created following ssloy's guide

pub const WHITE: image::Rgb<u8> = image::Rgb([255, 255, 255]);
pub const BLACK: image::Rgb<u8> = image::Rgb([0, 0, 0]);

pub const RED: image::Rgb<u8> = image::Rgb([255, 0, 0]);
pub const GREEN: image::Rgb<u8> = image::Rgb([0, 255, 0]);
pub const BLUE: image::Rgb<u8> = image::Rgb([0, 0, 255]);

pub const YELLOW: image::Rgb<u8> = image::Rgb([255, 255, 0]);
pub const MAGENTA: image::Rgb<u8> = image::Rgb([255, 0, 255]);
pub const CYAN: image::Rgb<u8> = image::Rgb([0, 255, 255]);

pub fn draw_line<T: image::GenericImage>(
    mut x0: u32,
    mut y0: u32,
    mut x1: u32,
    mut y1: u32,
    img: &mut T,
    color: T::Pixel,
) {
    let dx: i64 = (x1 as i64 - x0 as i64).abs();
    let dy: i64 = (y1 as i64 - y0 as i64).abs();
    let steep = dx < dy;

    let mut assigner = |x: u32, y: u32| -> () {
        match steep {
            false => img.put_pixel(x, y, color),
            true => img.put_pixel(y, x, color),
        };
    };

    if steep {
        // steep
        let temp = x0;
        x0 = y0;
        y0 = temp;

        let temp = x1;
        x1 = y1;
        y1 = temp;
    }

    if x0 > x1 {
        let temp = x0;
        x0 = x1;
        x1 = temp;

        let temp = y0;
        y0 = y1;
        y1 = temp;
    }

    let dx: i64 = (x1 as i64 - x0 as i64).abs();
    let dy: i64 = (y1 as i64 - y0 as i64).abs();

    let derror2 = 2 * dy;
    let mut error2 = 0;
    let mut y: i64 = y0 as i64;

    let inc_y = |y: i64| -> i64 {
        match y0 < y1 {
            true => y + 1,
            false => y - 1,
        }
    };
    for x in x0..=x1 {
        assigner(x as u32, y as u32);
        error2 = error2 + derror2;
        if error2 > dx {
            y = inc_y(y);
            error2 = error2 - dx * 2;
        }
    }
}
