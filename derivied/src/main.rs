#[derive(Debug, Default, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let z = Point::default();
    println!("{:?}", z);
    let p = Point { x: 10, y: 20 };
    println!("{:?}", p);
    let p1 = Point { x: 10, y: 20 };
    println!("{:?}", p == p1);
}
