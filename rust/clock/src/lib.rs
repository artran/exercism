use std::fmt;

const MINS_IN_HOUR: i32 = 60;
const HRS_IN_DAY: i32 = 24;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        hours += minutes.div_euclid(MINS_IN_HOUR);
        hours = hours.rem_euclid(HRS_IN_DAY);
        minutes = minutes.rem_euclid(MINS_IN_HOUR);

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
