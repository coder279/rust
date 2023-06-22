fn main() {
    let mut my_arr = [1,2,3];
    for my in my_arr.iter_mut() {
        *my *= 2;
    }
    println!("{:?}",my_arr)
}
