use std::{cell::RefCell, rc::Rc};

struct MyPreciousRing {
    engraving: String
}

fn main() {
    // inner mutability
    // 
    // RefCell => runtime implementation of ownership/borrowing rules
    // Mutex => semaphore for thread synchronization
    // RwLock => read/write lock for thread synchronization
    // 
    // Rc => ref counting for freeing memory when no longer used
    // Arc => atomic ref counting for thread safe reference counting
    
    let saurons_ring = Rc::new(MyPreciousRing {
        engraving: "One Ring to rule them all, One Ring to find them, One Ring to bring them all and in the darkness bind them".to_string()
    });
    println!("Saurons ring says: {}", saurons_ring.engraving);
    println!("Saurons ring (Rc): {:p}", &saurons_ring);
    println!("Saurons ring: {:p}", saurons_ring.engraving.as_str());
    println!("We have {} references to Saurons ring", Rc::strong_count(&saurons_ring));

    // a lot of things happen...
    
    let frodos_ring = saurons_ring.clone();
    println!("Frodos ring (Rc): {:p}", &frodos_ring);
    println!("Frodos ring: {:p}", frodos_ring.engraving.as_str());
    println!("We have {} references to Saurons ring", Rc::strong_count(&saurons_ring));
    
    drop(frodos_ring);
    
}
