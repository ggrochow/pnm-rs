const MAX_RGB_VALUE: i32 = 255;
const BIT_MAP_RGB_BREAKPOINT: i32 = 382; // (255+255+255) / 2

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    r: i32,
    g: i32,
    b: i32
}

impl Pixel {
    pub fn black() -> Pixel {
        Pixel { r: 0, g: 0, b: 0 }
    }

    pub fn white() -> Pixel {
        Pixel { r: 255, g: 255, b: 255 }
    }

    pub fn from_greyscale(value: i32, max_val: i32) -> Pixel {
        let rounded_rgb_grey = round_to_rgb(value, max_val);

        Pixel { r: rounded_rgb_grey, g: rounded_rgb_grey, b: rounded_rgb_grey }
    }

    pub fn from_rgb(r: i32, g: i32, b: i32, max_val: i32) -> Pixel {
        Pixel {
            r: round_to_rgb(r, max_val),
            g: round_to_rgb(g, max_val),
            b: round_to_rgb(b, max_val)
        }
    }

    fn total_rgb_val(&self) -> i32 {
        self.r + self.g + self.b
    }

    fn avg_rgb_val(&self) -> i32 {
        if self.r == self.g && self.r == self.b {
            return self.r;
        }

        return (self.total_rgb_val() as f64 / 3f64).round() as i32;
    }
}

fn round_to_rgb(value: i32, max_val: i32) -> i32 {
    if value == 0 { return 0 }

    let value_grey_percentage = value as f64 / max_val as f64;
    let rounded_rgb_value = (MAX_RGB_VALUE as f64 * value_grey_percentage).round();

    rounded_rgb_value as i32
}

fn rgb_to_val(value: i32, max_val: i32) -> i32 {
    if value == 0 { return 0 }

    let value_grey_percentage = value as f64 / MAX_RGB_VALUE as f64;
    let rounded_rgb_value = (max_val as f64 * value_grey_percentage).round();

    rounded_rgb_value as i32
}

pub trait PnmPixel {
    fn to_pbm(&self) -> String;

    fn to_pgm(&self, max_val: i32) -> String;

    fn to_ppm(&self, max_val: i32) -> String;
}

impl PnmPixel for Pixel {
    fn to_pbm(&self) -> String {
        let is_white = self.total_rgb_val() > BIT_MAP_RGB_BREAKPOINT;

        match is_white {
            true => String::from("1"),
            false => String::from("0")
        }
    }

    fn to_pgm(&self, max_val: i32) -> String {

        format!("{}", rgb_to_val(self.avg_rgb_val(), max_val))
    }

    fn to_ppm(&self, max_val: i32) -> String {

        format!(" {} {} {} ", rgb_to_val(self.r, max_val), rgb_to_val(self.g, max_val), rgb_to_val(self.b, max_val))
    }
}


