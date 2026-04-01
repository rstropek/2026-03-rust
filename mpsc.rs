use std::sync::mpsc;
use std::thread;

use rand::RngExt;

const SAMPLES: u64 = 100_000_000;
const THREADS: u64 = 4;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    for _ in 0..THREADS {
        // Spawn a thread
        let tx = tx.clone();
        thread::spawn(move || {
            let hits = monte_carlo_pi();
            tx.send(hits).unwrap();
        });
    }
    
    drop(tx);
    
    let total_hits: u64 = rx.iter().sum();
    let pi = 4.0 * (total_hits as f64) / (SAMPLES * THREADS) as f64;
    println!("Estimated value of pi: {}", pi);
}

// Monte Carlo method to estimate the value of pi
// 
// Returns the number of points that fall inside the unit circle
fn monte_carlo_pi() -> u64 {
    let mut rng = rand::rng();
    let mut hits = 0;
    
    for _ in 0..SAMPLES {
        let x: f64 = rng.random();
        let y: f64 = rng.random();
        if x * x + y * y <= 1.0 {
            hits += 1;
        }
    }
    
    hits
}
