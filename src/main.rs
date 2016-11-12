mod pbm;


use pbm::PBM;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
//    println!("argv = {:?}", argv);
    let scale = argv
        .get(1).expect("1st ARGV must be the desired scale")
        .parse::<u32>().expect("1st ARGV must be a valid positive number designating the desired scale");

    let input_file_path = argv.
        get(2).expect("2nd ARGV must be the path to the input PNM");

    let output_file_path = argv
        .get(3).expect("3rd ARGV must be the path to the output PNM");

    // Get file as string
    let pbm_string = get_file_as_string(input_file_path);


    let mut pbm = PBM::from_string(&pbm_string);
    pbm.scale_up(scale);
    let out_string = format!("{}", pbm);
//    println!("{}", out_string);
    let mut out_file = File::create(output_file_path).expect("out-file path must be a valid writeable filename");
    out_file.write_all(out_string.as_bytes()).expect("Write failed");
}


fn get_file_as_string(path: &str) -> String {
    let mut file = File::open(path).expect("The file path must be a valid, open-able file");
    let mut file_as_string = String::new();
    file.read_to_string(&mut file_as_string).expect("The file must be valid utf-8");

    file_as_string
}