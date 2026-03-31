#![allow(dead_code, unused_variables, unused)]

struct Customer {
    name: String,
}

struct Order<'a> {
    customer: &'a Box<Customer>,
    product: String,
    amount: f64,    
}

impl Customer {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }    
}

fn main() {
    let o2;
    let c = Box::new(Customer::new("Alice")); // Works because lifetime of c is long enough
    {
        //let c = Box::new(Customer::new("Alice")); // Does not work because c does NOT live long enough
        
        let o1 = Order {
            customer: &c,
            product: "Laptop".to_string(),
            amount: 999.99,
        };
        
        o2 = Order {
            customer: &c,
            product: "Mouse".to_string(),
            amount: 25.50,
        };
    }
    
    println!("The customer {} ordered a laptop", o2.customer.name);
}
