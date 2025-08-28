// Assignment: Implement a Custom Iterator in Rust

trait Iterator {
    type Item;
    fn new() -> Self;
    fn next(&mut self) -> Option<Self::Item>;
}

// Problem Statement
// You are required to implement a simple counter using Rust that behaves
// like an iterator. Your task is to define a Counter struct and implement
// the Iterator trait for it so that it returns the numbers from 1 to 5,
// one by one.
struct Counter {
    count: u32,
}
// Requirements
// Create a struct named Counter with a single field count of type u32.
// Implement a method new() that initializes Counter with count = 0.
// Implement the Iterator trait for Counter:

// Set the associated Item type to u32.
// Implement the next() method to:
// Increment count each time it is called.
// Return Some(count) while count <= 5.
// Return None once the counter exceeds 5.
impl Iterator for Counter {
    type Item = u32;

    fn new() -> Self {
        Counter { count: 0 }
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}

// In the main function, use the iterator to print numbers 1 through 5 to
// the console.

fn main() {
    println!("Inside the Assignment 2 Block !");
    let mut counter = Counter::new();
    while let Some(value) = counter.next() {
        println!("{}", value);
    }
}
