use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Mutable borrowing via a closure
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // Moving ownership into the closure
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // Even more fun with closures
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // This works
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    let mut sort_operations: Vec<String> = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        // This doesn't because we're moving `value` out of the closure environment which we can't do in an `FnMut`
        // sort_operations.push(value);
        r.width
    });
}
