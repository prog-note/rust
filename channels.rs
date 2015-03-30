use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    for id in 0..10 {
        let clone_sender = sender.clone();
        thread::spawn(move || {
            clone_sender.send(id);
        });
    }

    let mut ids: Vec<Result<i32, mpsc::RecvError>> = vec![];
    for _ in 0..10 {
        ids.push(receiver.recv());
    }

    println!("results: {:?}", ids);
}
