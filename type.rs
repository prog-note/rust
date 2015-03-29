// i8, i16, i32, i64, isize (pointer size)
// u8, u16, u32, u64, usize (pointer size)
// f32, f64

type Integer64 = i64;
type UInteger64 = u64;

fn main() {
    let default_int = 1;
    let default_float = 2.0;
    println!("Default int size: {}", std::mem::size_of_val(&default_int)*8);
    println!("Default float size: {}", std::mem::size_of_val(&default_float)*8);

    let int_64: i64 = 1;
    let float_32: f32 = 2.0;
    println!("Explicit - i64: {}, f32: {}", int_64, float_32);

    let integer: Integer64 = 1;
    let uinteger: UInteger64 = 2;
    println!("Alias - Integer64: {}, UInteger64: {}", integer, uinteger);

    let postfix_i64 = 1i64;
    let postfix_f64 = 2f64;
    println!("Inference - postfix i64: {}, postfix f64: {}", postfix_i64, postfix_f64);

    let decimal = 65.4321 as f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting - decimal: {}, int: {}, char: {}", decimal, integer, character);
}
