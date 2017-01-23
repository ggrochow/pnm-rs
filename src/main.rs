extern crate rustc_serialize;
extern crate rayon;

mod pbm;
mod pixel;
mod vec2;

use pbm::PBM;
use vec2::Vec2;
use pixel::Pixel;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use rustc_serialize::json;
use rayon::prelude::*;
// 03 - Woof3d
fn main() {
    let file_name = env::args().nth(1).unwrap();
    let world: World = json::decode::<InJSON>(&get_file_as_string(&file_name)).unwrap().into();
    let mut pbm = PBM::new_blank_pnm(world.height, world.width, world.background);
    for triangle in &world.triangles {
        println!("triangle.get_walls() = {:?}", triangle.get_walls());
    }
    for base_y in 0..world.height as usize {
        for base_x in 0..world.height as usize {
            let x = base_x as f64 + 0.5;
            let y = base_y as f64 + 0.5;

            for triangle in &world.triangles {
                let mut hit_count = 0;
                for (p0, p1) in triangle.get_walls() {
                    if let Some(distance) =
                    get_distance_to_ray_line_intersection(&Vec2{ x: x, y: y }, &Vec2 { x: 5.0, y: 5.0 }, &p0, &p1) {
                        hit_count += 1;
                    }
                }

                if hit_count == 1 {
                    pbm.set_pixel(x as i32, y as i32, triangle.color)
                }
            }
        }
    }


    let mut file = File::create("out.pnm").unwrap();
    file.write_all(format!("{}", pbm).as_bytes()).unwrap();
}

fn get_distance_to_ray_line_intersection(p0: &Vec2, v0: &Vec2, p1: &Vec2, p2: &Vec2) -> Option<f64> {
    let v1 = p2.minus(&p1);

    let v0_cross_v1 = v0.cross(&v1);

//    println!("p0 = {:?}, v0 = {:?}, p1 = {:?}, v1 = {:?}", p0, v0, p1, v1);
    if v0_cross_v1 == 0.0 {
        //    if (v0_cross_v1 * 100000.0).round() / 100000.0 == 0.0 {
        // rounding to deal with the fact that our angles aren't perfect due to input-radians-accuracy
        // Segments are parallel / co-linear
        // in our case we don't care about co-linear collisions
        return None;
    }

    let p1_minus_p0 = p1.minus(&p0);

    let s0 = p1_minus_p0.cross(&v1) / v0_cross_v1;
    let s1 = p1_minus_p0.cross(&v0) / v0_cross_v1;

    if s0 >= 0.0 && s1 <= 1.0 && s1 >= 0.0 {
        // because v0 is of 1 distance,
        // s0 = distance to collision
//        println!("{:?}", p0.plus(&v0.multiply(s0)));
        Some(s0)
    } else {
        None
    }
}

fn get_file_as_string(path: &str) -> String {
    let mut file = File::open(path).expect("The file path must be a valid, open-able file");
    let mut file_as_string = String::new();
    file.read_to_string(&mut file_as_string).expect("The file must be valid utf-8");

    file_as_string
}

#[derive(Debug, RustcDecodable)]
struct InJSON {
    height: i32,
    width: i32,
    background: Vec<i32>,
    triangles: Vec<TriangleJSON>,
}

#[derive(Debug)]
struct World {
    height: i32,
    width: i32,
    background: Pixel,
    triangles: Vec<Triangle>,
}

impl From<InJSON> for World {
    fn from(json: InJSON) -> Self {
        let mut triangles: Vec<Triangle> = Vec::with_capacity(json.triangles.len());
        for triangleJson in json.triangles {
            triangles.push(triangleJson.into());
        }

        World {
            height: json.height,
            width: json.width,
            background: Pixel::from_rgb(json.background[0], json.background[1], json.background[2], 255),
            triangles: triangles
        }
    }
}
#[derive(Debug, RustcDecodable)]
struct TriangleJSON {
    points: Vec<Vec<f64>>,
    color: Vec<i32>
}

#[derive(Debug)]
struct Triangle {
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    color: Pixel
}

impl Triangle {
    fn get_walls(&self) -> Vec<(Vec2, Vec2)> {
        vec![
            (Vec2{ x: self.p0.x, y: self.p0.y }, Vec2 { x: self.p1.x, y: self.p1.y}),
            (Vec2{ x: self.p0.x, y: self.p0.y }, Vec2 { x: self.p2.x, y: self.p2.y}),
            (Vec2{ x: self.p2.x, y: self.p2.y }, Vec2 { x: self.p1.x, y: self.p1.y}),
        ]
    }
}

impl From<TriangleJSON> for Triangle {
    fn from(json: TriangleJSON) -> Self {
        Triangle {
            p0: Vec2 { x: json.points[0][0], y: json.points[0][1] },
            p1: Vec2 { x: json.points[1][0], y: json.points[1][1] },
            p2: Vec2 { x: json.points[2][0], y: json.points[2][1] },
            color: Pixel::from_rgb(json.color[0], json.color[1], json.color[2], 255)
        }
    }
}