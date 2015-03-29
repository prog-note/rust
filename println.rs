use std::fmt;

// add default Debug trait implementation to structure
#[derive(Debug)]
struct SomeStructure (isize, isize);

// implement Display for structure
impl fmt::Display for SomeStructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Awesome Structure ({}, {})", self.0, self.1)
    }
}



fn main() {
    println!("Simple Display - {}", 121);

    println!("Positional arguments {0}, {1}, {1}, {0}", 1, 2);

    println!("Named argument first: {first}, second: {second}",
              first=1,
              second=2);


    println!("Display: {}, Debug: {:?}, Specific format: {:b}", 1,2,3);
    // nothing ⇒ Display    ? ⇒ Debug       o ⇒ Octal
    // x ⇒ LowerHex         X ⇒ UpperHex    p ⇒ Pointer
    // b ⇒ Binary           e ⇒ LowerExp    E ⇒ UpperExp

    println!("test Debug - {:?}", SomeStructure(1,2));
    println!("test Display - {}", SomeStructure(3,4));


    println!("Integer - {:05}, Float - {:.2}, With sign - {:+}", 1, 2.023, 3);


    println!("align to right (with spaces) -  {:>10}",  "|");
    println!("align to center (with dots)  -  {:.^10}", "|");
    println!("align to left (with dashes)  -  {:-<10}", "|");
}
