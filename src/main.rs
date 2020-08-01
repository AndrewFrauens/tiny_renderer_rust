extern crate image;
extern crate obj;
//extern crate wavefront_obj;

// using all the stuff from the tinyrenderer... since the whole Point of what I'm doing is to
// build that file up.

mod tinyrenderer;
use crate::tinyrenderer::*;

fn main() {
    let model = obj::Obj::load("input/obj/african_head.obj").unwrap();
    //let model = wavefront_obj::obj::parse("input/obj/african_head.obj").unwrap();
    let mut img = image::RgbImage::new(500, 500);

    let mut count = 0;
    for object in &model.data.objects {
        for group in &object.groups {
            for poly in &group.polys {
                if count < 5 {
                    let mut pos_idxs: [usize; 3] = [0, 0, 0];
                    let mut tex_idxs: [usize; 3] = [0, 0, 0];
                    let mut nor_idxs: [usize; 3] = [0, 0, 0];
                    let P = Point { x: 0, y: 0, z: 0 };
                    let mut geo_tri: Triangle = Triangle::new([P, P, P]);

                    let mut pos_arr = [[0.,0.,0.],[0.,0.,0.],[0.,0.,0.]];

                    for (p_idxs, i) in poly.0.iter().zip(0..3) {
                        pos_idxs[i] = p_idxs.0;

                        tex_idxs[i] = p_idxs.1.unwrap();
                        nor_idxs[i] = p_idxs.2.unwrap();

                        pos_arr[i] = model.data.position[p_idxs.0];
                    }

                    let geo_tri = Triangle::new_scale([[pos_arr[0], pos_arr[1], pos_arr[2]]], img.width(), img.height());

                    println!("p:{:?}\tt:{:?}\tn:{:?}", pos_idxs, tex_idxs, nor_idxs);
                    println!(
                        "P1:{:?}\tp2:{:?}\tp3:{:?}",
                        model.data.position[pos_idxs[0]],
                        model.data.position[pos_idxs[1]],
                        model.data.position[pos_idxs[2]]
                    );

                    /*
                    let pos_idxs = [(poly.0)[0], (poly.1)[0], (poly.2)[0]];
                    let tex_idxs = [poly.0.1.unwrap(), poly.1.1.unwrap(), poly.2.1.unwrap()];
                    let nor_idxs = [poly.0.2.unwrap(), poly.1.2.unwrap(), poly.2.2unweap()];

                    */
                }

                /*
                for idx_tuple in poly {
                    if count < 5 {
                        count += 1;
                        println!("i: {:#?}", idx_tuple);

                    let position = idx_tuple.0;
                    let texture = idx_tuple.1.unwrap();
                    let normal = idx_tuple.2.unwrap();
                    println!("pos:{:3}\ttex:{:3}\tnor:{:3}", position, texture, normal);
                    //println!(" o: {}", group.position[p1]);
                    }
                */
            }
        }
    }

    //println!("model {:?}",model);
    /*

    let mut img = image::RgbImage::new(200, 200);

    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 10, y: 0 };
    let p3 = Point { x: 10, y: 10 };

    let t1 = Triangle {
        pt0: p1,
        pt1: p2,
        pt2: p3,
    };

    draw_triangle(t1, &mut img, RED);

    let p1 = Point { x: 10, y: 15 };
    let p2 = Point { x: 40, y: 150 };
    let p4 = Point { x: 230, y: 94 };
    let t2 = Triangle {
        pt0: p1,
        pt1: p4,
        pt2: p2,
    };

    draw_triangle(t2, &mut img, CYAN);
    /*
        draw_line(10, 10, 178, 25, &mut img, RED);
        draw_line(50, 78, 73, 9, &mut img, RED);
        draw_line(178, 9, 10, 24, &mut img, BLUE);
        draw_line(15, 13, 156, 167, &mut img, GREEN);
        draw_line(15, 166, 156, 12, &mut img, CYAN);
        draw_line(8, 175, 3, 6, &mut img, YELLOW);
    */

    */
    let img = image::imageops::flip_vertical(&img);

    img.save("./output/pixel16.tga").unwrap();
}
