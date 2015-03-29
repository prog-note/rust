fn main() {
    for i in 0..3 {
        println!("{}", i)
    }

    for i in [4, 5, 6].iter() {
        println!("{}", i);
    }

    loop {
        println!("infinity loop!");
        break;
    }


    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }

    println!("happy end;)");
}
