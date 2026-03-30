mod calculation;

use calculation::tower_calculation;

fn main() {
    let calculations = tower_calculation();
    for (current, op, i, r) in calculations {
        println!("{:>10} {} {} = {}", current, op, i, r);
    }
}
