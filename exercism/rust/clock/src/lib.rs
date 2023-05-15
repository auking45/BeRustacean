use std::fmt;

const MIN_AN_HOUR: i32 = 60;
const MINUTES_A_DAY: i32 = 24 * MIN_AN_HOUR;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_min = hours * MIN_AN_HOUR + minutes;

        if total_min < 0 {
            total_min = MINUTES_A_DAY + (total_min % MINUTES_A_DAY);
        }

        let hours = (total_min / MIN_AN_HOUR) % 24;
        let minutes = total_min % MIN_AN_HOUR;

        Self {
            hours,
            minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut total_min = self.minutes + minutes;

        if total_min < 0 {
            total_min = MINUTES_A_DAY + (total_min % MINUTES_A_DAY);
        }

        let hours = (self.hours + total_min / MIN_AN_HOUR) % 24;
        let minutes = total_min % MIN_AN_HOUR;

        Self {
            hours,
            minutes
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
