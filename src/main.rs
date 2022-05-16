mod point;
use point::Point;

struct Square {
    points: [Point; 4],
}
// let mut pr = 0.0;
// for (i, cur_point) in self.points.iter().enumerate() {
//     pr += cur_point.distance(self.points[(i + 1) % 4]);
// }

impl Square {
    pub fn new(bottom_left: Point, size: f32) -> Self {
        let points = [
            bottom_left,
            bottom_left.translate(0.0, size),
            bottom_left.translate(size, size),
            bottom_left.translate(size, 0.0),
        ];
        Square { points }
    }

    pub fn perimeter(&self) -> f32 {
        4.0 * self.points[0].distance(self.points[1])
    }

    pub fn area(&self) -> f32 {
        self.points[0].distance(self.points[1]).powi(2)
    }
}

fn main() {
    let origin: Point = Point::new(0.0, 0.0);
    let square: Square = Square::new(origin, 2.0);

    for (i, point) in square.points.iter().enumerate() {
        println!("The {}th point of square is: {}", i, point);
    }
    println!(
        "The perimeter of the above square is {}",
        square.perimeter()
    );
    println!("The area of the above square is {}", square.area());
}
