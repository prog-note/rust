use std::thread;

fn main() {
    for num in range(0, 20i) {
        thread::spawn(move || {
            println!("-> {}", num);
        });
    }
}
