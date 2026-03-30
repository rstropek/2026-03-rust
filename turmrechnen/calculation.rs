const START: i32 = 5; // Must also be the result

/// Calculate a tower
/// 
/// A **tower** is a math exercise where you start with a number and then perform a series 
/// of operations (multiplication and division) in a specific order.
/// 
/// ## Example
/// 
/// ```
/// let calculations = tower_calculation();
/// for (current, op, i, r) in calculations {
///    // Print the calculation in a readable format
/// }
/// ```
pub fn tower_calculation() -> [(i32, char, i32, i32); 16] {
    let mut result = [(0, ' ', 0, 0); 16];
    
    let mut current = START;
    for i in 2..=9 {
        let r = current * i;
        result[i as usize - 2] = (current, '*', i, r);
        current = r;
    }
    
    for i in 2..=9 {
        let r = current / i;
        result[i as usize - 2 + 8] = (current, '/', i, r);
        current = r;
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_start_must_be_equal_last_result() {
        let calculations = tower_calculation();
        let last_result = calculations.last().unwrap().3; // Get the last result
        assert_eq!(START, last_result, "The START constant must be equal to the last result of the calculations.");
    }
}
