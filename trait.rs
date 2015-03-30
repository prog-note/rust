struct Test {
    desc: &'static str
}
impl Test {
    fn print() {
        println!("Trait cant have class and instance method with the same name");
    }
}
trait Face {
    fn print(&self) { println!("default") }
    fn new(&'static str) -> Self;
}
impl Face for Test {
    fn new(v: &'static str) -> Test {
        Test { desc: v }
    }
    fn print(&self) {
        println!("this is {}", self.desc)
    }
}


struct Test2<T> {
    val: T
}
trait Generic<T> {
    fn val(self) -> T;
}
impl<T> Generic<T> for Test2<T> {
    fn val(self) -> T {
        self.val
    }
}


fn print<T: Face>(v: T) {
    println!("Generic `print` for structs with Face trait");
    v.print()
}

fn main() {
    let t: Test = Face::new("test");
    Test::print();
    t.print();
    print(t);

    let test_str: Test2<&str> = Test2 { val: "test" };
    let test_int: Test2<isize> = Test2 { val: 111 };
    println!("Test2<&str>: {:?}, Test2<isize>: {:?}", test_str.val(), test_int.val());
}
