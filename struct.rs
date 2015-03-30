struct PointTuple (i32, i32);
struct PointStruct { x: i32, y: i32 }

struct PointUnit;
impl PointUnit {
    fn about() {
        println!("Struct without attributes");
    }
}

struct Generic<T> { bar: T }
impl<T> Generic<T> {
    fn bar(self) -> T {
        self.bar
    }
}

struct Car {
    speed: i32,
    stamp: &'static str
}
impl Car {
    fn new(stamp: &'static str, speed: i32) -> Car {
        println!("Initialize");
        Car { stamp: stamp, speed: speed }
    }
    fn print(&self) {
        println!("Car instance method - stamp: {}, speed: {}", self.stamp, self.speed);
    }
    fn about() {
        println!("Car class method - Car: stamp, speed");
    }
}

fn main() {
    let point_tuple: PointTuple = PointTuple(1, 2);
    let PointTuple(x, y) = point_tuple;
    println!("Extracted PointTuple values: x: {}, y: {}", x, y);

    let point_struct = PointStruct { x: 1, y: 2 };
    println!("Point struct - x: {}, y: {}", point_struct.x, point_struct.y);

    let generic_string = Generic { bar: "something" };
    let generic_int = Generic { bar: 1 };
    println!("Generic string: {}, int: {}", generic_string.bar(), generic_int.bar());

    let car: Car = Car { stamp: "ЗАПОР", speed: 120 };
    car.print();

    Car::new("BMW", 320).print();
    Car::about();
    PointUnit::about();
}
