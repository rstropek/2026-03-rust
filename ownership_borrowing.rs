#![allow(dead_code, unused_variables)]

struct Vector2D {
    x: f64,
    y: f64,
}

fn main() {
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = v1; // Transfer of ownership (move semantics)
    // "Highlander" - each value has exactly one owner
    consume(v2); // Another move
    let v3 = produce(); // Ownership of the produced vector is transferred to v3
    // The owner is responsible for cleaning up the resource when it goes out of scope
    drop(v3); // Explicitly dropping v3, though it would be dropped automatically at the end of scope
    
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = &v1; // Move? No, this is a reference (read-only borrowing)
    let v3 = &v1;
    print(&v2);
    print(&v3);
    // Yes, we can have multiple read-only borrows
    
    let mut v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = &mut v1; // Mutable borrow of v1
    v2.x = 10.0; // Modifying through the mutable reference
    let v3 = &mut v1;
    v3.x = 20.0;
    // v2.x = 40.0; // Would not work because v2 is still borrowed as mutable
    
    let mut my_numbers = vec![1, 2, 3];
    let my_numbers_ref = &mut my_numbers; // Borrowing the vector
    for num in my_numbers_ref {
        *num += 1;
        println!("Number: {}", num);
    }
    
    let my_vectors = vec![Vector2D { x: 1.0, y: 2.0 }, Vector2D { x: 3.0, y: 4.0 }];
    let my_vector = &my_vectors[1];
}

fn consume(v: Vector2D) {
    println!("Consuming vector: ({}, {})", v.x, v.y);
}

fn print(v: &Vector2D) {
    println!("Borrowing vector: ({}, {})", v.x, v.y);
}

fn produce() -> Vector2D {
    Vector2D { x: 3.0, y: 4.0 }
}
