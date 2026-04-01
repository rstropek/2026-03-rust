#![allow(dead_code)]

mod billing {
    struct ConsultingWork {
        what: String,
        hours: f32,
        rate: f32,
    }

    pub trait Billable {
        fn total(&self) -> f32;

        fn total_in_000(&self) -> f32 {
            self.total() / 1000.0
        }
    }

    impl Billable for ConsultingWork {
        fn total(&self) -> f32 {
            self.hours * self.rate
        }
    }

    pub struct MaterialCost {
        pub description: String,
        pub cost: f32,
    }

    impl Billable for MaterialCost {
        fn total(&self) -> f32 {
            self.cost
        }
    }
    
    impl Billable for f32 {
        fn total(&self) -> f32 {
            *self
        }
    }

    pub fn create_consulting_work(what: &str, hours: f32, rate: f32) -> impl Billable {
        ConsultingWork {
            what: what.to_string(),
            hours,
            rate,
        }
    }

    pub fn print_billable(b: &impl Billable) {
        println!(
            "Total: ${:.2}, Total in thousands: ${:.2}",
            b.total(),
            b.total_in_000()
        );
    }
    
    pub fn print_billable_2(b: &(impl Billable + std::fmt::Display)) {
        println!(
            "{b}, Total: ${:.2}, Total in thousands: ${:.2}",
            b.total(),
            b.total_in_000()
        );
    }
    
    pub fn create_billable(material: Option<&str>, costs: f32) -> Box<dyn Billable> { // Trait object
        if let Some(desc) = material {
            Box::new(MaterialCost {
                description: desc.to_string(),
                cost: costs,
            })
        } else {
            Box::new(costs)
        }
    }
    
    pub fn create_billables() -> Vec<Box<dyn Billable>> {
        vec![
            Box::new(create_consulting_work("Rust consulting", 10.0, 150.0)),
            Box::new(MaterialCost {
                description: "Rust book".to_string(),
                cost: 39.99,
            }),
            Box::new(42.0),
        ]
    }
    
    // Loyalty system, where people get one point for every $1000 they spend.
    // 1000$ -> 1 point, 1005$ -> 1 point, 1999$ -> 1 point, 2000$ -> 2 points
    pub trait Pointworthy {
        fn points(&self) -> i32;
    }
    
    impl<T: Billable> Pointworthy for T {
        fn points(&self) -> i32 {
            self.total_in_000().floor() as i32
        }
    }
    
    // Implement Billable for an array of Billables
    impl<T: Billable> Billable for Vec<T> {
        fn total(&self) -> f32 {
            self.iter().map(|b| b.total()).sum()
        }
    }
}

use billing::{Billable, create_consulting_work, print_billable, print_billable_2, create_billable, create_billables, Pointworthy};

fn main() {
    let cw = create_consulting_work("Rust consulting", 10.0, 150.0);
    let total = cw.total();
    let total_in_000 = cw.total_in_000();
    println!("Total is ${:.2}", total);
    println!("Total in thousands is ${:.2}k", total_in_000);
    print_billable(&cw);
    
    let mc = billing::MaterialCost {
        description: "Rust book".to_string(),
        cost: 39.99,
    };
    print_billable(&mc);
    
    let some_other_cost = 42.0;
    print_billable(&some_other_cost);
    print_billable_2(&some_other_cost);
    
    // print_billable_2(&cw); // does not work as Display impl is missing
    
    let to = create_billable(Some("JavaScript book"), 19.9);
    let to = create_billable(None, 19.9);
    let tos = create_billables();
    for b in tos {
        println!("Billable total: ${:.2}", b.total());
    }
    
    println!("Consulting work gives us {} points", cw.points());
    println!("Material cost gives us {} points", mc.points());
    println!("Some other cost gives us {} points", some_other_cost.points());
}
