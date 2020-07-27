extern crate image;

//use std::fs::File;
//use std::path::Path;
//use std::vec::Vec;

use image::ImageBuffer;
//use image::Pixel;


const WHITE:[u8; 3]   = [255, 255, 255];
const BLACK:[u8; 3] = [  0,   0,   0];

const RED:[u8; 3]     = [255,   0,   0];
const GREEN:[u8; 3] = [  0, 255,   0];
const BLUE:[u8; 3]  = [  0,   0, 255];

const YELLOW:[u8; 3]  = [255, 255,   0];
const CYAN:[u8; 3]    = [  0, 255, 255];
const MAGENTA:[u8; 3] = [255,   0, 255];


fn main() {
    let mut img = ImageBuffer::new(100,100);

    img.put_pixel( 10, 10, image::Rgb(WHITE));
    img.put_pixel( 20, 20, image::Rgb(RED));
    img.put_pixel( 30, 30, image::Rgb(GREEN));
    img.put_pixel( 40, 40, image::Rgb(BLUE));
    img.put_pixel( 50, 50, image::Rgb(YELLOW));
    img.put_pixel( 60, 60, image::Rgb(CYAN));
    img.put_pixel( 70, 70, image::Rgb(MAGENTA));

    //make white square to test black
    img.put_pixel( 81, 80, image::Rgb(WHITE));
    img.put_pixel( 80, 80, image::Rgb(WHITE));
    img.put_pixel( 82, 80, image::Rgb(WHITE));

    img.put_pixel( 81, 81, image::Rgb(WHITE));
    img.put_pixel( 80, 81, image::Rgb(WHITE));
    img.put_pixel( 82, 81, image::Rgb(WHITE));

    img.put_pixel( 81, 82, image::Rgb(WHITE));
    img.put_pixel( 80, 82, image::Rgb(WHITE));
    img.put_pixel( 82, 82, image::Rgb(WHITE));

    img.put_pixel( 81, 81, image::Rgb(BLACK));




    img.save("./output/pixel2.tga").unwrap();
}
