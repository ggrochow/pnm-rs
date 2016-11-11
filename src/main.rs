mod pbm;

use pbm::PBM;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let argv: Vec<String> = std::env::args().collect();

    let scale = argv
        .get(1).expect("1st ARGV must be the desired scale")
        .parse::<u64>().expect("1st ARGV must be a valid positive number designating the desired scale");

    let input_file_path = argv.
        get(2).expect("2nd ARGV must be the path to the input PNM");

    let output_file_path = argv
        .get(2).expect("3rd ARGV must be the path to the output PNM");

    // Get file as string
    let pbm_string = get_file_as_string(input_file_path);

    let magic_number = pbm_string
        .split_whitespace()
        .next()
        .expect("Must have magic number");

    let pbm = match magic_number {
        // P1 -> ASCII Bitmap
        "P1" => {
            let pbm = PBM::from_string(&pbm_string);
            pbm.scale(scale);
            println!("{}", pbm);

        }, //.scale(scale),
        // P2 -> ASCII Greyscale
        // P3 -> ASCII RGB
        _ => panic!("invalid magic number")
    };
}


fn get_file_as_string(path: &str) -> String {
    let mut file = File::open(path).expect("The file path must be a valid, open-able file");
    let mut file_as_string = String::new();
    file.read_to_string(&mut file_as_string).expect("The file must be valid utf-8");

    file_as_string
}