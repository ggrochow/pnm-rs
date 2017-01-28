use std::fmt;
use pixel::{ Pixel, PnmPixel };
use wall::Wall;
use rayon::prelude::*;
use super::Triangle;

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

    pub fn new_blank_pnm(height: i32, width: i32, fill_pixel: Pixel) -> Self {
        let mut raster: Vec<Pixel> = Vec::new();
        let size = height * width;
        raster.resize(size as usize, fill_pixel);

        PBM {
            magic_number: "P3".to_string(),
            height: height,
            width: width,
            size: size,
            max_val: 255,
            raster: raster
        }
    }

    fn get_half_height(&self) -> i32 {
        match self.height % 2 {
            0 => self.height / 2,
            _ => (self.height / 2) + 1
        }
    }

    pub fn fill_bottom_half(&mut self, fill_pixel: Pixel) {
        let half_size = self.get_half_height() * self.width;
        self.raster.truncate(half_size as usize);
        self.raster.resize(self.size as usize, fill_pixel)
    }

    pub fn draw_wall(&mut self, height: i32, x: i32, fill_pixel: Pixel) {
        let middle = self.get_half_height() - 1;

        for i in 0..height + 1 {
            self.set_pixel(x, middle + i, &fill_pixel);
            self.set_pixel(x, middle - i, &fill_pixel);
        }
    }

    pub fn draw_line(&mut self, x_start: i32, y_start: i32, x_end: i32, y_end: i32) {
        let mut x1 = x_start;
        let mut y1 = y_start;
        let mut x2 = x_end;
        let mut y2 = y_end;

        let is_steep = (y2 - y1).abs() > (x2 - x1).abs();

        if is_steep {
            let mut temp = 0;
            temp = x1;
            x1 = y1;
            y1 = temp;

            temp = x2;
            x2 = y2;
            y2 = temp;
        }

        if x1 > x2 {
            let mut temp = 0;
            temp = x1;
            x1 = x2;
            x2 = temp;

            temp = y1;
            y1 = y2;
            y2 = temp;
        }

        let delta_x = x2 - x1;
        let delta_y = (y2 - y1).abs();

        let mut error = delta_x / 2;
        let mut y_step = 1;
        if y1 >= y2 {
            y_step = -1;
        }

        let mut y = y1;
        for x in x1..(x2 + 1) {
            if is_steep {
                self.set_pixel(y, x, &Pixel::white());
            } else {
                self.set_pixel(x, y, &Pixel::white());
            }

            error -= delta_y;
            if error < 0 {
                y += y_step;
                error += delta_x;
            }
        }
    }

    pub fn draw_triangle(&mut self, triangle: &Triangle) {
        let top = triangle.get_top_walls();
        let bottom = triangle.get_bottom_walls();

        self.fill_horizontal_space_between_walls(&top.0, &top.1, &triangle.color);
        self.fill_horizontal_space_between_walls(&bottom.0, &bottom.1, &triangle.color);
    }

    pub fn fill_horizontal_space_between_walls(&mut self, wall1: &Wall, wall2: &Wall, pixel: &Pixel) {
        let mut start_y = wall1.greatest_min_y(wall2);
        let mut end_y = wall1.lowest_max_y(wall2);

        if start_y % 1.0 == 0.5 {
            println!("wall1 = {:?}, wall2 = {:?}", wall1, wall2);
            println!("start_y = {:?}", start_y);
        }
        if end_y % 1.0 == 0.5 {
            println!("wall1 = {:?}, wall2 = {:?}", wall1, wall2);
            println!("end_y = {:?}", end_y);
        }

        // if start_y OR end_y == 0.5 check?
        for base_y in start_y.round() as i32..end_y.round() as i32 {
             let y = base_y as f64 + 0.5;

             let wall_1_x = wall1.point_at_y(y);
             let wall_2_x = wall2.point_at_y(y);

             let mut start = wall_1_x;
             let mut end = wall_2_x;
             if start > end {
                 start = wall_2_x;
                 end = wall_1_x;
             }

             for base_x in start.round() as i32..end.round() as i32 {
                 self.set_pixel(base_x, base_y, &pixel);
             }
         }
        // offset y by 0.5, get lines X pos at that, draw to other lines X pos at that Y.
        // for 0.5 offset -> 0,5 offset of end, fill each pixel
        // don't include end-peices, check those to === 0.5, if so then do some edge checking
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        let pixel_index = x + (y * self.width);

        self.raster[pixel_index as usize]
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, pixel: &Pixel) {
        let pixel_index = (x + (y * self.width)) as usize;

        if let Some(px) = self.raster.get_mut(pixel_index) {
            px.update(&pixel);
        }
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

impl From<String> for PBM {
    fn from(target: String) -> Self {
        let filtered_str = target
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
