fn main() {
    // &str (string slice) can't be muttable
    let as_slice = "world";
    let as_slice2: &str = "world";

    // convert &str to String is muttable (like: vector)
    let mut as_string = "hello ".to_string();
    let mut as_string2: String = "hello ".to_string();

    // String back to &str is not chip
    let back_to_slice = &as_string2;

    println!("String + &str = {}", as_string + as_slice);
    println!("String - {}", as_string2);
    println!("&str - {}", as_slice2);
    println!("&str - {}", back_to_slice);


    let mut something = String::new();
    something.push('H');
    something.push('i');
    println!("{}", something);
}
