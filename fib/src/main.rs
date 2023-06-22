struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Point {
        Point { x, y }
    }
    fn get_x(&self) -> u64 {
        self.x
    }
}

fn aficionado(n: u64) -> u64 {
    if n < 2 {
        n
    } else {
        aficionado(n - 1) + aficionado(n - 2)
    }
}

fn main() {
    let p = Point::new(10, 20);
    let x = p.get_x();
    println!("{}", x)
}
