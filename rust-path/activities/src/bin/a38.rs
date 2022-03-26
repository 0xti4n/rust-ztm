// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish
use std::thread;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let msg_one = std::thread::spawn(|| msg_hello());

    let msg_two = std::thread::spawn(|| msg_thread());

    let msg_three = std::thread::spawn(|| msg_excited());

    let a = msg_one.join().unwrap();
    let b = msg_two.join().unwrap();
    let c = msg_three.join().unwrap();

    let fmt = format!("{}{}{}", a, b, c);
    println!("{:?}", fmt);
}
