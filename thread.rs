use std::thread;

fn main() {
    for num in 0..20i8 {
        thread::spawn(move || {
            println!("-> {}", num);
        });
    }
}
