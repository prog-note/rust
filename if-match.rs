fn main() {

    fn _if(x: isize) -> isize {
        if x < 0 { -1 }
        else if x > 0 { 1 }
        else { 0 }
    }
    println!("if, else: {}", _if(1));


    //-----------------------------------------------------------------
    fn _match_int(x: isize) -> &'static str {
        match x {
            0     => "zero",
            1 | 2 => "one or two",
            3...10 => "three to ten",
            _     => "something else"
        }
    }
    println!("match int: {}", _match_int(1));


    //-----------------------------------------------------------------
    fn _match_tuple(tuple: (i32, i32))  {
        match tuple {
            (0, y) => println!("first should be 0, second: {:?}", y),
            (x, 0) => println!("second should be 0, first: {:?}", x),
            _      => println!("doesn't matter"),
        }
    }
    _match_tuple((0,1));


    //-----------------------------------------------------------------
    enum Options {
        AsInt(isize),
        Nothing
    }
    fn _match_enum(x: Options) -> String {
        match x {
            Options::AsInt(n) => format!("it’s an int - {}", n),
            Options::Nothing  => "it’s nothing!".to_string(),
        }
    }
    println!("match enum: {}", _match_enum(Options::Nothing));
    println!("match enum: {}", _match_enum(Options::AsInt(2)));


    //-----------------------------------------------------------------
    struct Point {
        x: isize,
        y: isize
    }
    fn _match_struct(x: Point) -> String {
        match x {
            Point {x: 1, y: _} => "Point: X == 1, Y is ignored".to_string(),
            Point {x, y}       => format!("Point: X = {}, Y = {}", x, y)
        }
    }
    println!("match struct: {}", _match_struct(Point {x: 1, y: 2}));
    println!("match struct: {}", _match_struct(Point {x: 3, y: 2}));

    fn _match_with_if(point: Point) -> &'static str {
        match point {
            Point {x, y: _} if x == 1 => "Point: X == 1, Y is ignored",
            Point {x: _, y} if y > 1  => "Point: Y is more than 1, X is ignored",
            _                         => "Something else"
        }
    }
    println!("match with if: {}", _match_with_if(Point {x: 1, y: 2}));
    println!("match with if: {}", _match_with_if(Point {x: 3, y: 2}));
}
