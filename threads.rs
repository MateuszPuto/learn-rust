use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            let fib = fibonacci(10 * i);
            println!("{}", fib);
        }
    });

    handle.join().unwrap();
    println!("Offf, all fib's calculated.");
}

fn fibonacci(seq_item: i32) -> i32 {
    let number = match seq_item {
        1 => 0,
        2 => 1,
        3.. => fibonacci(seq_item - 1) + fibonacci(seq_item - 2),
        _ => panic!("Fibonacci sequence starts at 1'st item."),
    };

    number
}
