use std::fs as stdFS;

fn main() {
    let data = stdFS::read("src/main.rs").unwrap();
    println!("{}", String::from_utf8(data).unwrap());
}
