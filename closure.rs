fn main() {
    let first = 1isize;

    let closure = |arg: isize| -> isize {
        println!("argument {}", arg);
        println!("out varibale {}", first);
        arg + first
    };

    println!("closure result - {}", closure(2));
}
