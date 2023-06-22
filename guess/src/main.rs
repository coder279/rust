use std::io;
use rand::Rng;

fn main() {
    let guess_numver = rand::thread_rng().gen_range(0..101);
    loop {
        println!("Input the number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => continue
        };
        println!("Your guess number: {}", guess_number);
        if guess_number > guess_numver {
            println!("Too big");
        } else if guess_number < guess_numver {
            println!("Too small")
        } else {
            println!("bingo")
        }
    }
}
