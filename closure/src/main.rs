use std::thread;

fn main() {
    let hello = "hello";
    thread::spawn(move || println!("{}", hello));
}
