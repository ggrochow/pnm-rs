use std::fmt;
use pixel::{ Pixel, PnmPixel };

#[derive(Debug)]
pub struct PBM {
    magic_number: String,
    height: i32,
    width: i32,
    size: i32,
    max_val: i32,
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
            .parse::<i32>().expect("2nd set must be a valid number");

        let height = whitespace_iter.next().expect("3rd number is height")
            .parse::<i32>().expect("3rd set must be a valid number");

        let mut raster: Vec<Pixel> = Vec::new();
        let mut max_val:i32 = 255;

        match magic_number {
            "P1" => {
                let str_raster: String = whitespace_iter.collect();
                for char in str_raster.chars() {
                    match char {
                        '0' => raster.push(Pixel::black()),
                        '1' => raster.push(Pixel::white()),
                        _ => panic!("un-expected bitmap raster value")
                    }
                }
            },
            "P2" => {
                max_val = whitespace_iter.next().expect("4th number must be the max greyscale value")
                    .parse().expect("4th must be a valid number");

                for char in whitespace_iter {
                    let value: i32 = char.parse().expect("raster must only contain valid integers");
                    raster.push(Pixel::from_greyscale(value, max_val));
                }
            },

            "P3" => {
                max_val = whitespace_iter.next().expect("4th number must be the max value")
                    .parse().expect("4th must be a valid number");

                let mut rgb_vec: Vec<i32> = Vec::new();
                for char in whitespace_iter {
                    let value: i32 = char.parse().expect("raster must only contain valid integers");
                    rgb_vec.push(value);

                    if rgb_vec.len() == 3 {
                        raster.push(Pixel::from_rgb(rgb_vec[0], rgb_vec[1], rgb_vec[2], max_val));
                        rgb_vec.clear();
                    }
                }
            },
            _ => panic!("Invalid magic number")
        };

        PBM {
            magic_number: magic_number.to_string(),
            height: height,
            width: width,
            size: height * width,
            max_val: max_val,
            raster: raster
        }
    }

    pub fn new_blank_pbm(height: i32, width: i32) -> PBM {
        let mut raster: Vec<Pixel> = Vec::new();
        let size = height * width;
        raster.resize(size as usize, Pixel::black());

        PBM {
            magic_number: "P1".to_string(),
            height: height,
            width: width,
            size: size,
            max_val: 1,
            raster: raster
        }
    }

    pub fn draw_line(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32) {
        self.set_pixel(x_start, y_start, Pixel::white());

        let mut cursor_x = x_start;
        let mut cursor_y = y_start;

        let mut x_diff = 1;
        let mut y_diff = 1;

        while x_diff != 0 || y_diff != 0 {
            x_diff = i32::signum(cursor_x- x_end);
            y_diff = i32::signum(cursor_y - y_end);

            cursor_x -= x_diff;
            cursor_y -= y_diff;

            self.set_pixel(cursor_x, cursor_y, Pixel::white());
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        let pixel_index = x + (y * self.width);

        self.raster[pixel_index as usize]
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, pixel: Pixel) {
        let pixel_index = (x + (y * self.width)) as usize;
        if pixel_index < 1 || pixel_index > self.size as usize {
            println!("h{} w{}", self.height, self.width);
            println!("{}, s {},  x{} y{}", pixel_index, self.size, x, y);
            // TODO: better handle panic, possible do option / bool return
            // TODO: check against x - 1 / width | y - 1 / height
            panic!("set_pixel index out of bounds");
        }

        self.raster.remove(pixel_index);

        self.raster.insert(pixel_index, pixel);
    }

    pub fn scale_up(&mut self, scale: i32) {

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

    pub fn convert_to(&mut self, magic_number: &str) {
        self.magic_number = magic_number.to_string();
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
            match self.magic_number.as_ref() {
                "P1" => out_string += &pixel.to_pbm(),
                "P2" => out_string += &pixel.to_pgm(self.max_val),
                "P3" => out_string += &pixel.to_ppm(self.max_val),
                _ => panic!()
            };

            if (i+1) as i32 % self.width == 0 {
                out_string += "\n";
            } else {
                out_string += " ";
            }
        }

        write!(f, "{}", out_string)
    }

}
