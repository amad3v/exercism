use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn value(v: i32, offset: i32) -> i32 {
        match v < 0 {
            true => offset + v,
            false => v,
        }
    }

    fn zero_pad(v: i32) -> String {
        match v < 10 {
            true => format!("0{}", v),
            false => format!("{}", v),
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        const HRS_PER_DAY: i32 = 24;
        const MINS_PER_HR: i32 = 60;
        // .rem_euclid == %
        let h = (hours + minutes.div_euclid(MINS_PER_HR)) % HRS_PER_DAY;
        let m = minutes % MINS_PER_HR;

        Self {
            hours: Self::value(h, HRS_PER_DAY),
            minutes: Self::value(m, MINS_PER_HR),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, minutes + self.minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}:{}",
            Self::zero_pad(self.hours),
            Self::zero_pad(self.minutes)
        )
    }
}
