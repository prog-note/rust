use std::mem;

fn main() {
    let first = 1;
    println!("immutable:  {}", first as int);

    let second = 2i;
    println!("immutable with type inference: {}", second);

    let big_int = 1i64;
    println!("size of `big_int` in bytes: {}", std::mem::size_of_val(&big_int));
    println!("size of `big_int` in bites: {}", std::mem::size_of_val(&big_int)*8);

    let third: int = 4;
    println!("immutable with defined type: {}", third);

    let mut four: int = 2;
    println!("mutable with defined type: {}", four);


    let (x, y, z) = (1, 2, 3);
    println!("parallel assign - x: {}, y: {}, z: {}", x, y, z);

    let (x, y, z) = (z, y, x);
    println!("parallel reassign - x: {}, y: {}, z: {}", x, y, z);
}
