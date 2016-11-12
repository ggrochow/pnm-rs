use std::fmt;

#[derive(Debug)]
pub struct PBM {
    magic_number: String,
    height: u32,
    width: u32,
    max_val: u32,
    raster: Vec<Pixel>
}

impl PBM {
    pub fn from_string(pbm_string: &str) -> PBM {
        let filtered_str = pbm_string
            .split("\n")
            .filter(|line| !line.starts_with('#'))
            .fold(String::new(), |acc, line| acc + line + "\n");


        let mut whitespace_iter = filtered_str.split_whitespace();

        let magic_number = whitespace_iter.next().expect("1st number is pbm type");

        let width = whitespace_iter.next().expect("2nd number is width")
            .parse::<u32>().expect("2nd set must be a valid number");

        let height = whitespace_iter.next().expect("3rd number is height")
            .parse::<u32>().expect("3rd set must be a valid number");

        let mut raster: Vec<Pixel> = Vec::new();

        match magic_number {
            "P1" => {
                let str_raster: String = whitespace_iter.collect();
                for char in str_raster.chars() {
                    match char {
                        '0' => raster.push(Pixel::BitMap(0)),
                        '1' => raster.push(Pixel::BitMap(1)),
                        _ => panic!("un-expected bitmap raster value")
                    }
                }

                PBM {
                    magic_number: magic_number.to_string(),
                    height: height,
                    width: width,
                    max_val: 1,
                    raster: raster
                }
            },
            "P2" => {
                let max_val: u32 = whitespace_iter.next().expect("4th number must be the max greyscale value")
                    .parse().expect("4th must be a valid number");

                for char in whitespace_iter {
                    let value: u32 = char.parse().expect("raster must only contain valid integers");
                    raster.push(Pixel::GreyScale(value));
                }

                PBM {
                    magic_number: magic_number.to_string(),
                    height: height,
                    width: width,
                    max_val: max_val,
                    raster: raster
                }
            },

            "P3" => {
                let max_val: u32 = whitespace_iter.next().expect("4th number must be the max greyscale value")
                    .parse().expect("4th must be a valid number");

                let mut rgb_vec: Vec<u32> = Vec::new();
                for char in whitespace_iter {
                    let value: u32 = char.parse().expect("raster must only contain valid integers");
                    rgb_vec.push(value);

                    if rgb_vec.len() == 3 {
                        raster.push(Pixel::RGB(rgb_vec[0], rgb_vec[1], rgb_vec[2]));
                        rgb_vec.clear();
                    }
                }


                PBM {
                    magic_number: magic_number.to_string(),
                    height: height,
                    width: width,
                    max_val: max_val,
                    raster: raster
                }

            },
            _ => panic!("Invalid magic number")

        }
    }

    pub fn scale_up(&mut self, scale: u32) {

        let mut scaled_raster: Vec<Pixel> = Vec::new();
        {
            let mut raster_iter = self.raster.iter();

            for _ in 0..self.height {
                let mut row: Vec<Pixel> = Vec::new();

                for _ in 0..self.width {
                    let pixel = raster_iter.next().expect("raster is too short");
                    for _ in 0..scale {
                        row.push(pixel.clone());
                    }
                }

                for _ in 0..scale {
                    for px in &row {
                        scaled_raster.push(px.clone());
                    }
                }
            }
        }

        self.raster = scaled_raster;
        self.height = self.height * scale;
        self.width = self.width * scale;
    }
}

impl fmt::Display for PBM {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let raster_iter = self.raster.iter();
        let mut out_string = format!("{}\n{} {}\n",self.magic_number, self.width, self.height);
        if self.magic_number != "P1" {
            out_string += &format!("{}\n", self.max_val);
        }
        for (i, pixel) in raster_iter.enumerate() {
            out_string += &format!("{}", pixel);

            if (i+1) as u32 % 35 == 0 {
                out_string += "\n";
            } else {
                out_string += " ";
            }
        }

        write!(f, "{}", out_string)
    }

}

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
enum Pixel {
    BitMap(u32),
    GreyScale(u32),
    RGB(u32, u32, u32)
}

impl fmt::Display for Pixel {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Pixel::BitMap(i) => write!(f, "{}", i),
            &Pixel::GreyScale(i) => write!(f, "{}", i),
            &Pixel::RGB(r, g, b) => write!(f, "{} {} {}", r, g, b)
        }
    }

}