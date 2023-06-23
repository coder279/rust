use std::fmt::{Formatter};
use std::net::Shutdown::Write;

struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{},{}",self.x,self.y)
    }
}

fn show<T: std::fmt::Display>(a: T) {
    println!("{}", a)
}

fn main() {
    let p = Point {
        x: 20,
        y: 30,
    };
    show(p)
}
