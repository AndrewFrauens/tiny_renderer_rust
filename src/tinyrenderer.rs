#![allow(dead_code)]
extern crate image;

use std::cmp;

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

#[derive(Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub pt0: Point,
    pub pt1: Point,
    pub pt2: Point,
}

pub fn barycentric(tri: Triangle, p: Point) -> [f32; 3] {
    let determinant: i128 = (tri.pt1.y as i128 - tri.pt2.y as i128)
        * (tri.pt0.x as i128 - tri.pt2.x as i128)
        + (tri.pt2.x as i128 - tri.pt1.x as i128) * (tri.pt0.y as i128 - tri.pt2.y as i128);

    if determinant == 0 {
        panic!("try to divide by zero while finding barycentric coordinates");
    }

    let u: f32 = ((tri.pt1.y as i128 - tri.pt2.y as i128) * (p.x as i128 - tri.pt2.x as i128)
        + (tri.pt2.x as i128 - tri.pt1.x as i128) * (p.y as i128 - tri.pt2.y as i128))
        as f32
        / (determinant) as f32;

    let v: f32 = ((tri.pt2.y as i128 - tri.pt0.y as i128) * (p.x as i128 - tri.pt2.x as i128)
        + (tri.pt0.x as i128 - tri.pt2.x as i128) * (p.y as i128 - tri.pt2.y as i128))
        as f32
        / (determinant) as f32;

    let w = 1.0 - u - v;

    [u, v, w]
}

pub fn pt_is_in_triangle(tri: Triangle, p: Point) -> bool {
    let bary_coord = barycentric(tri, p);
    //This will probably lead to errors at some point... but so will leaving the comparison as
    //being to 0 exactly...
    let tol = -0.000005;

    // if any coord is less than 0, then pt is outside tri
    let inside_truthyness: bool =
        !(bary_coord[0] < tol || bary_coord[1] < tol || bary_coord[2] < tol);

    inside_truthyness
}

/// Finds the max and min values to create bounding box
pub fn get_bounding_box(tri: Triangle) -> [Point; 2] {
    let x_max = cmp::max(tri.pt0.x, cmp::max(tri.pt1.x, tri.pt2.x));
    let y_max = cmp::max(tri.pt0.y, cmp::max(tri.pt1.y, tri.pt2.y));

    let x_min = cmp::min(tri.pt0.x, cmp::min(tri.pt1.x, tri.pt2.x));
    let y_min = cmp::min(tri.pt0.y, cmp::min(tri.pt1.y, tri.pt2.y));

    let pt_max = Point { x: x_max, y: y_max };
    let pt_min = Point { x: x_min, y: y_min };

    [pt_max, pt_min]
}

pub fn draw_triangle(tri: Triangle, img: &mut image::RgbImage, _color: image::Rgb<u8>) {
    let bounding_box = get_bounding_box(tri);
    /*println!(
        "boundingbox: ({}, {}) ({}, {})",
        bounding_box[0].x, bounding_box[0].y, bounding_box[1].x, bounding_box[1].y
    );*/

    for col in (bounding_box[1].x)..(bounding_box[0].x) {
        for row in (bounding_box[1].y)..(bounding_box[0].y) {
            //println!("row: {}, col: {}", row, col);
            if pt_is_in_triangle(tri, Point { x: col, y: row }) {
                img.put_pixel(col, row, _color);
            }
        }
    }
}

pub fn draw_line(
    mut x0: u32,
    mut y0: u32,
    mut x1: u32,
    mut y1: u32,
    img: &mut image::RgbImage,
    color: image::Rgb<u8>,
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
