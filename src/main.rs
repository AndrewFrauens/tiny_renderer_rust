extern crate image;
extern crate obj;

// using all the stuff from the tinyrenderer... since the whole Point of what I'm doing is to
// build that file up.

mod tinyrenderer;
use crate::tinyrenderer::*;

fn main() {
    //let model = obj::Obj::load("input/obj/african_head.obj").unwrap();

    //println!("model {:?}",model);

    let mut img = image::RgbImage::new(200, 200);

    let p1 = Point { x: 10, y: 15 };
    let p2 = Point { x: 40, y: 150 };
    let p3 = Point { x: 180, y: 34 };

    let t1 = Triangle {
        pt0: p1,
        pt1: p2,
        pt2: p3,
    };

    draw_triangle(t1, &mut img, RED);

    //let p1 = Point { x: 10, y: 15 };
    //let p2 = Point { x: 40, y: 150 };
    let p4 = Point { x: 130, y: 94 };
    let t2 = Triangle {
        pt0: p1,
        pt1: p4,
        pt2: p3,
    };

    draw_triangle(t2, &mut img, RED);
    /*
        draw_line(10, 10, 178, 25, &mut img, RED);
        draw_line(50, 78, 73, 9, &mut img, RED);
        draw_line(178, 9, 10, 24, &mut img, BLUE);
        draw_line(15, 13, 156, 167, &mut img, GREEN);
        draw_line(15, 166, 156, 12, &mut img, CYAN);
        draw_line(8, 175, 3, 6, &mut img, YELLOW);
    */

    let img = image::imageops::flip_vertical(&img);

    img.save("./output/pixel11.tga").unwrap();
}
