fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

struct Point<T>{
    x:T,
    y:T,
}

struct Line<T>{
    x:Point<T>,
    y:Point<T>
}

fn main() {
    let x = largest(32, 64);
    println!("{}", x);
    let point1:Point<u32> = Point{
        x:32,
        y:64,
    };
    let point2:Point<u32> = Point{
        x:8,
        y:16,
    };
    let line = Line{
        x:point1,
        y:point2,
    };
    println!("{:?}",line.y.y);
    println!("{:?}",line.x.y);
}
