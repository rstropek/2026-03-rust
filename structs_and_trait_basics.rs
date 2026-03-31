#![allow(unused)]

mod my_time {
    #[derive(Default)]
    pub struct Clock {
        hours: i32,
        minutes: i32,
    }

    impl Clock {
        pub fn new(hours: i32, minutes: i32) -> Self {
            Self { hours, minutes }
        }

        pub fn get_hours(&self) -> i32 {
            self.hours
        }

        pub fn get_minutes(&self) -> i32 {
            self.minutes
        }

        // Normalizes minutes and hours
        // 
        // E.g. 10:65 -> 11:05, 25:30 -> 01:30, etc.
        // fn normalize(&self) -> Self // That would mean that we create a NEW normalized Clock
        // fn normalize(self) -> Self // Similar to above, but consumes the original Clock
        // fn normalize(&mut self) // That would mean that we modify the original Clock in place
        pub fn normalize(&self) -> Self {
            let total_minutes = self.hours * 60 + self.minutes;
            let normalized_hours = (total_minutes / 60) % 24; // Modulo 24 to wrap around the clock
            let normalized_minutes = total_minutes % 60;
            Self {
                hours: normalized_hours,
                minutes: normalized_minutes,
            }
        }
        
        pub fn normalize_self(&mut self) {
            let total_minutes = self.hours * 60 + self.minutes;
            self.hours = (total_minutes / 60) % 24; // Modulo 24 to wrap around the clock
            self.minutes = total_minutes % 60;
        }
        
        pub fn print(self) {
            println!("The time is {}:{}", self.hours, self.minutes);
        }
    }
}

mod my_time_with_copy {
    #[derive(Copy, Clone, Default)]
    pub struct Clock {
        hours: i32,
        minutes: i32,
    }
    
    impl Clock {
        pub fn new(hours: i32, minutes: i32) -> Self {
            Self { hours, minutes }
        }
        
        pub fn get_hours(&self) -> i32 {
            self.hours
        }
        
        pub fn get_minutes(&self) -> i32 {
            self.minutes
        }
        
        pub fn normalize(&self) -> Self {
            let total_minutes = self.hours * 60 + self.minutes;
            let normalized_hours = (total_minutes / 60) % 24; // Modulo 24 to wrap around the clock
            let normalized_minutes = total_minutes % 60;
            Self {
                hours: normalized_hours,
                minutes: normalized_minutes,
            }
        }
        
        pub fn print(self) {
            println!("The time is {}:{}", self.hours, self.minutes);
        }
    }
}

fn main() {
    // let clock = my_time::Clock::new(12, 30);
    let clock = my_time_with_copy::Clock::new(12, 30);
    // let clock: my_time_with_copy::Clock = Default::default();
    println!("The time is {}:{}", clock.get_hours(), clock.get_minutes());
    // println!("The time is {}:{}", clock.hours, clock.minutes); // Does not work because hours and minutes are private
    clock.print();
    clock.print(); // This only works with "copy" variant
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use pretty_assertions::assert_eq;

    #[rstest]
    #[case(10, 65, 11, 5)]
    #[case(12, 80, 13, 20)]
    #[case(25, 00, 01, 05)]
    fn test_normalize(#[case] hours: i32, #[case] minutes: i32, #[case] expected_hours: i32, #[case] expected_minutes: i32) {
        let clock = my_time::Clock::new(hours, minutes);
        let normalized_clock = clock.normalize();
        assert_eq!(normalized_clock.get_hours(), expected_hours);
        assert_eq!(normalized_clock.get_minutes(), expected_minutes);
    }

    #[test]
    fn test_normalize_self() {
        let mut clock = my_time::Clock::new(25, 30);
        clock.normalize_self();
        assert_eq!(clock.get_hours(), 1);
        assert_eq!(clock.get_minutes(), 30);
    }    
}
