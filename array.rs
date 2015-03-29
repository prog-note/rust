fn main() {
    let array = [1,2,3];
    println!("simple array: {:?}", array);

    let slice_array: &[i32] = array.as_slice();
    println!("slice array first val: {}", slice_array[0]);

    let direct_slice_array = &[1,2,3];
    println!("first el of direct array slice: {:?}", direct_slice_array[0] as i32);
    println!("direct array slice length: {:?}", direct_slice_array.len());
}
