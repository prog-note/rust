fn main() {
    let first = 1;
    println!("immutable variable:  {}", first);

    let mut second = 2;
    println!("mutable variable: {}", second);

    let (x, y, z) = (1, 2, 3);
    let (x, y, z) = (z, y, x);
    println!("parallel assigning - x: {}, y: {}, z: {}", x, y, z);

    // `let` iside `if` for destruction object
    let number = Some(7);
    if let Some(i) = number {
        println!("Some - {}", i);
    }
}
