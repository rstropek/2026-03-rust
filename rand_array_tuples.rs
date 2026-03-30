#![allow(unused)]

use rand::prelude::*;

fn main() {
    let mut rng = rand::rng();
    let my_number = rng.random_range(1..=10);
    
    // Numbers < 5 lose, 5 is "nearly", > 5 wins
    let msg = match my_number {
        5 => "So close!",
        1..=4 => "You lose!",
        _ => "You win!" // else
    };
    println!("The result is: {msg}");
    
    let my_numbers = [1, 2, 3, 4, 5];
    let mut my_numbers = [0u8; 500];
    
    // Fill my_numbers with random numbers between 0 and (excluding) 10
    for num in &mut my_numbers {
        *num = rng.random_range(0..10u8);
    }
    
    println!("Numbers: {my_numbers:?}");
    
    let my_tuple = (true, 42, "Hello World");
    println!("{} {} {}", my_tuple.0, my_tuple.1, my_tuple.2);
    let (mut a, b, _) = my_tuple;
    println!("{a} {b}");
    a = false;
    
    let my_tuple = ((1, 2), (3, 4));
    
    let mut my_tuple = (1, 2);
    (my_tuple.1, my_tuple.0) = (my_tuple.0, my_tuple.1);
}
