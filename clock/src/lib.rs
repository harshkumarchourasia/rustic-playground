use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Clock{
    minutes: i32,
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { 
            minutes: ((hours * 60) + minutes).rem_euclid(24*60)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: (self.minutes + minutes).rem_euclid(24*60)
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result{
        write!(
            f,
            "{:02}:{:02}",
            self.minutes/60, 
            self.minutes%60
        )
    }
}