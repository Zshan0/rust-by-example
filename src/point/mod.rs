#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
    // Note that we have to now define the return value to be f32
    pub fn distance(&self, point: Point) -> f32 {
        // f32 is a type with methods described for it that can be called
        // in the same way as methods are called on other class instances.
        ((self.x - point.x).abs().powi(2) + (self.y - point.y).abs().powi(2)).sqrt()
    }

    // Returns the point translated by a given value.
    pub fn translate(&self, x: f32, y: f32) -> Point {
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(f32, f32)> for Point {
    fn from(t: (f32, f32)) -> Self {
        Point { x: t.0, y: t.1 }
    }
}
