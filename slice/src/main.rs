fn main() {
    let mut arr : [i32;5] = [1,2,3,4,5];
    let slice = &arr[3..5];
    println!("{:?}",slice);
    let mut slice3 = &mut arr[..];
    slice3[0] = 6;
    println!("{:?}", slice3);
}
