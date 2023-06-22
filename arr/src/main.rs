fn main() {
    let mut my_arr: [u32; 5] = [1, 2, 3, 4, 5];
    my_arr[2] = 100;
    println!("console{:?}", my_arr);
    let mut my_arr2: [u32; 1024 * 5] = [0; 1024 * 5];
    println!("console{:?}", my_arr2)
}
