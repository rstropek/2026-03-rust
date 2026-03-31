#![allow(dead_code, unused_variables, unused)]

struct Vector2D {
    x: f64,
    y: f64,
}

fn get_longer_vector<'a>(v1: &'a Vector2D, v2: &'a Vector2D) -> &'a Vector2D {
    let v1_length = v1.x.powi(2) + v1.y.powi(2);
    let v2_length = v2.x.powi(2) + v2.y.powi(2);
    
    if v1_length > v2_length {
        v1
    } else {
        v2
    }
}

fn get_longer_vector_3<'a, 'b, 'c>(v1: &'a Vector2D, v2: &'b Vector2D) -> &'c Vector2D {
    &Vector2D { x: 0.0, y: 0.0 }
}

fn get_longer_vector_2<'a, 'b>(v1: &'a Vector2D, v2: &'a Vector2D, v3: &'b Vector2D, v4: &'b Vector2D) 
    -> (&'a Vector2D, &'b Vector2D) {
    let longer_v1_v2 = get_longer_vector(v1, v2);
    let longer_v3_v4 = get_longer_vector(v3, v4);
    (longer_v1_v2, longer_v3_v4)
}

fn main() {
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    
    let longer;
    {
        let v2 = Vector2D { x: 3.0, y: 4.0 };
        longer = get_longer_vector(&v1, &v2);
        println!("The longer vector is: ({}, {})", longer.x, longer.y); // works
    }
    //println!("The longer vector is: ({}, {})", longer.x, longer.y); // does not work
    
    let v3 = get_longer_vector_3(&Vector2D { x: 5.0, y: 6.0 }, &Vector2D { x: 7.0, y: 8.0 });
    println!("The longer vector is: ({}, {})", v3.x, v3.y);
}
