#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug)]
enum SomeErrors {
    DivisionByZero
}

fn div_with_result(x: i32, y: i32) -> Result<i32, SomeErrors> {
    if x == 0 { Err(SomeErrors::DivisionByZero) }
    else      { Ok(x / y) }
}

fn div_with_option(x: i32, y: i32) -> Option<i32> {
    if x == 0 { None }
    else      { Some(x/y) }
}

fn match_result(res: Result<i32, SomeErrors>) {
    match res {
        Err(why) => println!("bad: {:?}", why),
        Ok(res) => println!("good: {:?}", res)
    }
}

fn main() {
    println!("Direction Up as i32: {}", Direction::Up as isize);
    println!("Direction Up: {:?}", Direction::Up);

    println!("Option: 0/1 = {:?}", div_with_option(0, 1));
    println!("Option: 4/2 = {:?}", div_with_option(4, 2));

    println!("Result: 0/1 = {:?}", div_with_result(0, 1));
    println!("Result: 4/2 = {:?}", div_with_result(4, 2));

    match_result(div_with_result(0, 1));
    match_result(div_with_result(4, 2));

    div_with_option(0, 1).expect("fail! with None Option");
}
