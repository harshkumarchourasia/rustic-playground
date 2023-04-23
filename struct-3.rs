/*
Define a struct named Point that has fields for x and y coordinates. 
Write a method for the struct that calculates the distance between two points.
 */
struct Point{
    x: f64,
    y: f64
}

fn dist(p1: &Point, p2: &Point) -> f64 {
    (p1.x - p2.x).abs() + (p1.y -p2.y).abs()
}
fn main() {
    let p1 = Point {
        x: 0.0,
        y: 0.0
    };
    let p2 = Point {
        x: 1.0,
        y: 1.0
    };
    let dist: f64 = dist(&p1, &p2);
    println!("{}", dist);
}