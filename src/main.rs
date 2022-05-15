mod point;
use point::Point;

struct Square {
    points: [Point; 4],
}

fn main() {
    let square: Square = Square {
        points: [
            (0.0, 0.0).into(),
            (0.0, 1.0).into(),
            (1.0, 1.0).into(),
            (1.0, 0.0).into(),
        ],
    };

    for (i, point) in square.points.iter().enumerate() {
        println!("The {}th point of square is: {}", i, point);
    }
}
