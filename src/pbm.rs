use std::fmt;

#[derive(Debug)]
pub struct PBM {
    height: u64,
    width: u64,
    raster: Vec<bool>
}

impl PBM {
    pub fn from_string(pbm_string: &str) -> PBM {
        let filtered_str = pbm_string
            .split("\n")
            .filter(|line| !line.starts_with('#'))
            .fold(String::new(), |acc, line| acc + line + "\n");


        let mut whitespace_iter = filtered_str.split_whitespace();

        let _ = whitespace_iter.next().expect("1st number is pbm type");
        let width = whitespace_iter.next().expect("2nd number is width")
            .parse::<u64>().expect("2nd set must be a valid number");

        let height = whitespace_iter.next().expect("3rd number is height")
            .parse::<u64>().expect("3rd set must be a valid number");

        let str_raster: String = whitespace_iter.collect();
        let mut raster = Vec::new();

        for char in str_raster.chars() {
            raster.push(char == '1');
        }

        PBM {
            height: height,
            width: width,
            raster: raster
        }
    }

    pub fn scale_up(&self, scale: u32) {

    }
}

impl fmt::Display for PBM {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut raster_iter = self.raster.iter();
        let mut out_string = String::new();
        for x in 0..self.height {
            for y in 0..self.width {
                let pixel = raster_iter.next().expect("raster too short");
                if !!pixel {
                    out_string += "▓";
                } else {
                    out_string += "░";
                }
            }
            out_string += "\n";
        }

        write!(f, "{}", out_string)
    }

}