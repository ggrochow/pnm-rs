use super::vec2::Vec2;

#[derive(Debug, Clone)]
pub struct Wall {
    pub p0: Vec2,
    pub p1: Vec2,
}

impl Wall {
    pub fn new(p0: Vec2, p1: Vec2) -> Self {
        Wall {
            p0: p0,
            p1: p1
        }
    }

    pub fn max_x(&self) -> f64 {
        if self.p0.x > self.p1.x {
            self.p0.x
        } else {
            self.p1.x
        }
    }

    pub fn min_x(&self) -> f64 {
        if self.p0.x > self.p1.x {
            self.p1.x
        } else {
            self.p0.x
        }
    }

    pub fn lowest_max_x(&self, other: &Wall) -> f64 {
        if self.max_x() > other.max_x() {
            other.max_x()
        } else {
            self.max_x()
        }
    }

    pub fn greatest_max_x(&self, other: &Wall) -> f64 {
        if self.max_x() > other.max_x() {
            self.max_x()
        } else {
            other.max_x()
        }
    }

    pub fn lowest_min_x(&self, other: &Wall) -> f64 {
        if self.min_x() > other.min_x() {
            other.min_x()
        } else {
            self.min_x()
        }
    }

    pub fn greatest_min_x(&self, other: &Wall) -> f64 {
        if self.min_x() > other.min_x() {
            self.min_x()
        } else {
            other.min_x()
        }
    }

    pub fn max_y(&self) -> f64 {
        if self.p0.y > self.p1.y {
            self.p0.y
        } else {
            self.p1.y
        }
    }

    pub fn min_y(&self) -> f64 {
        if self.p0.y > self.p1.y {
            self.p1.y
        } else {
            self.p0.y
        }
    }

    pub fn lowest_max_y(&self, other: &Wall) -> f64 {
        if self.max_y() > other.max_y() {
            other.max_y()
        } else {
            self.max_y()
        }
    }

    pub fn greatest_max_y(&self, other: &Wall) -> f64 {
        if self.max_y() > other.max_y() {
            self.max_y()
        } else {
            other.max_y()
        }
    }

    pub fn lowest_min_y(&self, other: &Wall) -> f64 {
        if self.min_y() > other.min_y() {
            other.min_y()
        } else {
            self.min_y()
        }
    }

    pub fn greatest_min_y(&self, other: &Wall) -> f64 {
        if self.min_y() > other.min_y() {
            self.min_y()
        } else {
            other.min_y()
        }
    }

    pub fn point_at_y(&self, y: f64) -> f64 {
        let mut p0: Vec2;
        let mut p1: Vec2;

        if self.p0.y > self.p1.y {
            p0 = self.p0.clone();
            p1 = self.p1.clone();
        } else {
            p0 = self.p1.clone();
            p1 = self.p0.clone();
        }

        let v0 = p1.minus(&p0);
        let diff = (y - p0.y) / v0.y;



        let scaled = v0.multiply(diff);

        let pos = p0.plus(&scaled);
        let neg = p0.minus(&scaled);



        if pos.y == y {
            pos.x
        } else if neg.y == y {
            neg.x
        } else if (pos.y * 100.0).round() / 100.0 == y {
            pos.x
        } else if (neg.y * 100.0).round() / 100.0 == y {
            neg.x
        } else {
            println!("p0 = {:?}", p0);
            println!("p1 = {:?}", p1);
            println!("v0 = {:?}", v0);
            println!("y = {:?}", y);
            println!("diff = {:?}", diff);
            println!("scaled = {:?}", scaled);
            println!("pos = {:?}", pos);
            println!("neg = {:?}", neg);
            panic!()
        }
    }

//    pub fn multiply(&self, multiplier: f64) -> Vec2 {
//        Vec2 {
//            x: self.x * multiplier,
//            y: self.y * multiplier
//        }
//    }
}
