//
// #[item_attribute] - attribute for module or file
//
// #![crate_attribute] - attribute for crate
//
//   - #[attribute = "value"]
//   - #[attribute(key = "value")]
//   - #[attribute(value)]
//
// #![crate_type = "lib"] - crate as library (instead `--crate-type` compile param)
// #![crate_name = "something"] - library is named "something"
//
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("you are running linux!")
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("you are *not* running linux!")
}

#[cfg(not(some_key))]
fn conditional_function() {
    println!("you don't pass `--cfg some_key`!")
}

#[cfg(some_key)]
fn conditional_function() {
    println!("you just pass `--cfg some_key`!")
}

// allow unused code
#[allow(dead_code)]

// allow camel case type names
#[allow(non_camel_case_types)]

// add default Show trait implementation to struct
#[derive(Show)]
struct SomeStructure (int, int);

fn main() {
    are_you_on_linux();
    conditional_function();
    println!("test derive Show - {:?}", SomeStructure(1,2));
}
