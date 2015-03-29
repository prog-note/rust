fn main() {
    let tuple: (i32, char) = (1, 't');

    println!("tuple with (int, char): {:?}", tuple);
    println!("first: {}, second: {}", tuple.0, tuple.1);
}
