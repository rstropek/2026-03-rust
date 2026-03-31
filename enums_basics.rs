#![allow(unused_variables)]

#[derive(Clone)]
struct Customer {
    name: String
}

#[derive(Clone)]
enum HotelRoom {
    Vacant,
    Occupied(Customer),
    UnderMaintenance(i32 /* number of remaining days */),
}

impl HotelRoom {
    fn print_status(&self) {
        match self {
            HotelRoom::Vacant => println!("The hotel room is vacant."),
            HotelRoom::Occupied(c) => println!("The hotel room is occupied by {}.", c.name),
            HotelRoom::UnderMaintenance(days) => println!("The hotel room is under maintenance for {} more days.", days),
        }
    }
}

fn main() {
    let mut hr = HotelRoom::Vacant;
    hr = HotelRoom::Occupied(Customer { name: "John".to_string() });
    let hr = HotelRoom::UnderMaintenance(42);
    
    hr.print_status();
    
    let hr = HotelRoom::Occupied(Customer { name: "Alice".to_string() });
    
    // Check if hr is occupied and print the customer's name
    // Combine the if statement with a variable declaration + assignment
    if let HotelRoom::Occupied(c) = hr {
        println!("The hotel room is occupied by {}.", c.name);
    }
    
    let hr1 = HotelRoom::Occupied(Customer { name: "Jane".to_string() });
    let hr2 = HotelRoom::UnderMaintenance(42);
    
    if let HotelRoom::Occupied(c) = hr1
        && let HotelRoom::UnderMaintenance(days) = hr2
        && c.name == "Jane"
        && days > 30
    {
        println!("The hotel room is occupied by {} and will be under maintenance for {} more days.", c.name, days);
    }
}
