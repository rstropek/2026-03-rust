#![allow(dead_code)]

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn div(a: i32, b: i32) -> i32 {
    if b == 0 { 0 } else { a / b }
    // return b == 0 ? 0 : a / b;
}

fn main() {
    let mut a = 32;
    let b = 10i16;
    let c = a + b;
    println!("The result of {} + {} is {}", a, b, c);

    let c = add(a as i32, b as i32);
    println!("The result of add({}, {}) is {}", a, b, c);

    a += 1;
    let c = a + b;
    println!("The result of {} + {} is {}", a, b, c);

    let user_input = "42";
    let user_input: i32 = user_input.parse().unwrap();
    println!("The user input is {}", user_input);

    let a = 42;
    let mut a = a; // Makes a mutable
    a += 1;
    let a = a; // "Freezes" a again
    println!("The value of a is {}", a);

    let d = 42;
    {
        let mut d = d;
        d += 1;
        println!("The value of d is {}", d);
    }
    println!("The value of d is {}", d);
    
}
