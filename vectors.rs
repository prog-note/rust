fn main() {

    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("range(0, 10) collected into: {:?}", collected_iterator);

    let mut vector = vec![1i32, 2, 3];
    vector.push(4);

    println!("dynamically sized vector: {:?}", vector);
    println!("vector size: {:?}", vector.len());
    println!("item 0: {:?}", vector.get(0));
    println!("last: {:?}", vector.last());
    println!("pop: {:?}", vector.pop());
}
