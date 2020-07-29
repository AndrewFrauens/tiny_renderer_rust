extern crate image;
extern crate obj;

// using all the stuff from the tinyrenderer... since the whole point of what I'm doing is to
// build that file up.

mod tinyrenderer;
use crate::tinyrenderer::*;


fn main() {
    
    let model = obj::Obj::load("input/obj/african_head.obj");



    let mut img = image::ImageBuffer::new(200, 200);

    draw_line(10, 10, 178, 25, &mut img, RED);
    draw_line(50, 78, 73, 9, &mut img, RED);
    draw_line(178, 9, 10, 24, &mut img, BLUE);
    draw_line(15, 13, 156, 167, &mut img, GREEN);
    draw_line(15, 166, 156, 12, &mut img, CYAN);
    draw_line(8, 175, 3, 6, &mut img, YELLOW);

    let img = image::imageops::flip_vertical(&img);

    img.save("./output/pixel10.tga").unwrap();

}
