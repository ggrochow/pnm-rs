extern crate rustc_serialize;

mod pbm;
mod pixel;
mod vec2;

use pbm::PBM;
use vec2::Vec2;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;

// 03 - Woof3d
fn main() {
    let argv: Vec<String> = std::env::args().collect();
    println!("argv = {:?}", argv);
    let file_name = argv
        .get(1).expect("1st ARGV must be the desired input file");

    let out_file_name = argv.get(2).expect("2nd ARGV must be output filename");

    let json_string = get_file_as_string(file_name);
    let json: InJSON = json::decode(&json_string).expect("Input file must be valid json");
    println!("json = {:?}", json);

    let mut pnm = PBM::new_blank_pnm(json.camera.height as i32, json.camera.width as i32);

    let sky_pixel = pixel::Pixel::from_rgb(
        json.sky_color[0],
        json.sky_color[1],
        json.sky_color[2],
        255
    );

    pnm.fill_top_half(sky_pixel);

    let ground_pixel = pixel::Pixel::from_rgb(
        json.ground_color[0],
        json.ground_color[1],
        json.ground_color[2],
        255
    );

    pnm.fill_bottom_half(ground_pixel);

    let pixel_radian_width = json.camera.h_fov / json.camera.width;
    let left_radians = json.camera.theta + json.camera.h_fov / 2.0;
    let left_with_offset = left_radians + pixel_radian_width / 2.0;
    let v_fov = json.camera.h_fov * json.camera.height / json.camera.width;
    let pixel_radian_height = v_fov / json.camera.height;

//    println!("left_radians = {:?}", left_radians);
//    println!("left_with_offset = {:?}", left_with_offset);
//    println!("pixel_radian_width = {:?}", pixel_radian_width);
//    println!("pixel_radian_height= {:?}", pixel_radian_height);
//    println!("v_fov = {:?}", v_fov);

    let camera = Vec2 { x: json.camera.x, y: json.camera.y };

    for i in 1..(json.camera.width + 1.0) as usize {

        let center_radians = left_with_offset - pixel_radian_width * i as f64;
        let camera_point_1_length_away = Vec2 {
        // hacky rounding, input doesn't have actual PI values for perfect 90 deg angles
        x: camera.x + 1.0 * center_radians.cos(),
        y: camera.y + 1.0 * center_radians.sin()
    };
        let one_length_camera_vec = camera_point_1_length_away.minus(&camera);

//        println!("\npixel {} radians {}", i, center_radians);
        let mut closest_dist = std::f64::MAX;
        let mut closest_wall: Option<&Wall> = None;

        for wall in &json.walls {
            let p0 = Vec2 { x: wall.x0.clone(), y: wall.y0.clone() };
            let p1 = Vec2 { x: wall.x1.clone(), y: wall.y1.clone() };

            if let Some(distance) = get_distance_to_ray_line_intersection(&camera, &one_length_camera_vec, &p0, &p1) {
                if distance < closest_dist {
                    closest_dist = distance;
                    closest_wall = Some(wall);
                }
            }
        }

        if let Some(wall) = closest_wall {
            // Draw line
            let wall_radian_height = (1.0 / closest_dist).atan();
//            println!("wall_radian_height = {:?}", wall_radian_height);
//            println!("dist = {:?}", closest_dist);
            let wall_pixel_height = wall_radian_height / pixel_radian_height;
//            println!("wall_pixel_height = {:?}", wall_pixel_height);
            let wall_color = pixel::Pixel::from_rgb(
                wall.color[0],
                wall.color[1],
                wall.color[2],
                255
            );

            pnm.draw_wall(wall_pixel_height as i32, (i as i32 - 1), wall_color)
        }

    }
    // figure out FOV / Theta stuff to figure out camera rays + scaling distance
    // for each width, cast a ray down its path, do ray-wall collsion.
    // closest wall gets drawn to the screen, calc height based off of perspective view distances.
    // color as well

    let out_string = format!("{}", pnm);
    //    println!("{}", &out_string);
    let mut out_file = File::create(out_file_name).expect("out-file path must be a valid writeable filename");
    out_file.write_all(out_string.as_bytes()).expect("Write failed");

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

#[derive(Debug)]
#[derive(RustcDecodable)]
struct InJSON {
    walls: Vec<Wall>,
    ground_color: Vec<i32>,
    sky_color: Vec<i32>,
    camera: Camera
}

#[derive(Debug, RustcDecodable)]
struct Wall {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    color: Vec<i32>
}

#[derive(Debug)]
#[derive(RustcDecodable)]
struct Camera {
    x: f64,
    y: f64,
    theta: f64,
    h_fov: f64,
    width: f64,
    height: f64,
}