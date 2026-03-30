#![allow(unused)]

fn main() {
    let mut my_numbers = vec![1, 2, 3];
    my_numbers.push(4);
    
    let mut my_numbers = Vec::with_capacity(3);
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    
    let num: i32 = "42".parse().unwrap();
    let num = "42".parse::<i32>().unwrap();
    
    let mut my_numbers: Vec<i32> = (1..10).collect();
    let mut my_numbers = (1..10).collect::<Vec<_>>();
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    
    for i in my_numbers {
        println!("{}", i);
    }
    
    let mut my_array = Box::new([1, 2, 3]);
    println!("The length of my array is {}", my_array.len());
    println!("The first number in my array is {}", my_array[0]);
    my_array[0] = 10;
    println!("The first number in my array is {}", my_array[0]);
}
