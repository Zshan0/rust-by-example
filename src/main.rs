mod point;
use point::Point;

fn main() {
    let origin: Point = (0.0, 0.0).into();
    let one = Point::from((1.0, 1.0));
    println!("The origin is: {}", origin);
    println!(
        "The distance of {} from the origin is: {}",
        one,
        one.distance(origin)
    );
}
