fn main() {
    let first = 1;
    println!("immutable variable:  {}", first);

    // `_` - suppress `unused` warning
    let mut _second = 2;
    println!("mutable variable: {}", _second);


    let (x, y, z) = (1, 2, 3);
    let (x, y, z) = (z, y, x);
    println!("parallel assigning - x: {}, y: {}, z: {}", x, y, z);


    // `let` inside `if` for destruction object
    let number = Some(7);
    if let Some(i) = number {
        println!("Some - {}", i);
    }


    // `let` inside block(new scope)
    let mut main_scope_var = "main";
    {
        println!("vars from {} scope are available", main_scope_var);
        main_scope_var = "inner scope";

        let inner_variable = "inner scope vars are available just in current scope";
        let main_scope_var = "main scope vars could be shadowed (become inaccessible)";
        println!("{} ", inner_variable);
        println!("{}", main_scope_var);
    }
    println!("main scope vars could be changed from {}", main_scope_var);
}
