fn main() {
    let mut sum = 0;
    loop {
        sum = sum + 1;
        if sum == 100 {
            break;
        }
    }
    println!("{:?}", sum)
}
