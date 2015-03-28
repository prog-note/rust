mod my {
    // private function
    fn private_function() {
        println!("my::private_function()");
    }
    // public function
    pub fn function() {
        println!("my::function()");
    }

    // A private module
    mod private_mod {
        pub fn function() {
            println!("super::private_mod::function()");
        }
    }
    // A public module
    pub mod nested {
        pub fn function() {
            super::private_mod::function();
            println!("my::nested::function()");
        }
    }
}

// mod my;      -- load my mod from `my.rs` or `my/mod.rs`
// use my::mod; -- include module
// use something = my::nested::function; -- assign method

fn main() {
    my::function();
    my::nested::function();
}
