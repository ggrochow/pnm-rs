mod pbm;


use pbm::PBM;
use pbm::Pixel;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
//    println!("argv = {:?}", argv);
    let size = argv
        .get(1).expect("1st ARGV must be the desired scale");
    let x_index = size
        .find('x').expect("size must contain 'x'");

    let (width_str, height_str) = size.split_at(x_index);
    let width: i32 = width_str
        .parse().expect("scale must only be valid numbers and 'x'");

    let height: i32 = height_str
        .replace('x',"").
        parse().expect("scale must only be valid numbers and 'x'");

    let start_point = argv.
        get(2).expect("2nd ARGV be the start point");
    let start_split_index = start_point
        .find(',').expect("2nd point must contain ','");

    let (start_x_str, start_y_str) = start_point.split_at(start_split_index);

    let start_y: i32 = start_y_str
        .replace(',', "")
        .parse().expect("1 points must only be #s and ,");

    let start_x: i32 = start_x_str
        .parse().expect("2 points must only be #s and ,");

    let end_point = argv
        .get(3).expect("3rd ARGV must be the end point");
    let end_split_index = end_point
        .find(',').expect("point must contain ','");;

    let (end_x_str, end_y_str) = end_point.split_at(end_split_index);
    let end_y: i32 = end_y_str
        .replace(',', "")
        .parse().expect("4 points must only be #s and ,");

    let end_x: i32 = end_x_str
        .parse().expect("5 points must only be #s and ,");

    let out_file_name = argv.get(4).expect("4th argv must be the output filename");

    let mut pbm = PBM::new_blank_pbm(height, width);
    pbm.draw_line(start_x, start_y, end_x, end_y);

    let out_string = format!("{}", pbm);
    let mut out_file = File::create(out_file_name).expect("out-file path must be a valid writeable filename");
    out_file.write_all(out_string.as_bytes()).expect("Write failed");
}

// 00 - Scaling PNM
//fn main() {
//    let argv: Vec<String> = std::env::args().collect();
//    //    println!("argv = {:?}", argv);
//    let scale = argv
//        .get(1).expect("1st ARGV must be the desired scale")
//        .parse::<i32>().expect("1st ARGV must be a valid positive number designating the desired scale");
//
//    let input_file_path = argv.
//        get(2).expect("2nd ARGV must be the path to the input PNM");
//
//    let output_file_path = argv
//        .get(3).expect("3rd ARGV must be the path to the output PNM");
//
//    // Get file as string
//    let pbm_string = get_file_as_string(input_file_path);
//
//
//    let mut pbm = PBM::from_string(&pbm_string);
//    pbm.scale_up(scale);
//    let out_string = format!("{}", pbm);
//    let mut out_file = File::create(output_file_path).expect("out-file path must be a valid writeable filename");
//    out_file.write_all(out_string.as_bytes()).expect("Write failed");
//}
//
//fn get_file_as_string(path: &str) -> String {
//    let mut file = File::open(path).expect("The file path must be a valid, open-able file");
//    let mut file_as_string = String::new();
//    file.read_to_string(&mut file_as_string).expect("The file must be valid utf-8");
//
//    file_as_string
//}
