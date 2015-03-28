fn main() {
    let first = 1i;

    let closure = |arg: int| -> int {
        println!("argument {}", arg);
        println!("out varibale {}", first);
        arg + first
    };

    println!("closure result - {}", closure(2));
}
