//
//// 01 - Drawing PNM Lines
//fn main() {
//    let argv: Vec<String> = std::env::args().collect();
//    println!("argv = {:?}", argv);
//    let size = argv
//        .get(1).expect("1st ARGV must be the desired scale");
//    let x_index = size
//        .find('x').expect("size must contain 'x'");
//
//    let (width_str, height_str) = size.split_at(x_index);
//    let width: i32 = width_str
//        .parse().expect("scale must only be valid numbers and 'x'");
//
//    let height: i32 = height_str
//        .replace('x',"").
//        parse().expect("scale must only be valid numbers and 'x'");
//
//    let start_point = argv.
//        get(2).expect("2nd ARGV be the start point");
//    let start_split_index = start_point
//        .find(',').expect("2nd point must contain ','");
//
//    let (start_x_str, start_y_str) = start_point.split_at(start_split_index);
//
//    let start_y: i32 = start_y_str
//        .replace(',', "")
//        .parse().expect("1 points must only be #s and ,");
//
//    let start_x: i32 = start_x_str
//        .parse().expect("2 points must only be #s and ,");
//
//    let end_point = argv
//        .get(3).expect("3rd ARGV must be the end point");
//    let end_split_index = end_point
//        .find(',').expect("point must contain ','");;
//
//    let (end_x_str, end_y_str) = end_point.split_at(end_split_index);
//    let end_y: i32 = end_y_str
//        .replace(',', "")
//        .parse().expect("4 points must only be #s and ,");
//
//    let end_x: i32 = end_x_str
//        .parse().expect("5 points must only be #s and ,");
//
//    let out_file_name = argv.get(4).expect("4th argv must be the output filename");
//
//    let mut pbm = PBM::new_blank_pbm(height, width);
//    pbm.draw_line(start_x, start_y, end_x, end_y);
//
//    let out_string = format!("{}", pbm);
////    println!("out_ = {}", out_string);
//    let mut out_file = File::create(out_file_name).expect("out-file path must be a valid writeable filename");
//    out_file.write_all(out_string.as_bytes()).expect("Write failed");
//}
