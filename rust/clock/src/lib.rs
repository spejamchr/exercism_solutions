#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    day_minute: i32,
}

impl Clock {
    const HOUR_MINUTES: i32 = 60;
    const DAY_MINUTES: i32 = Self::HOUR_MINUTES * 24;

    pub fn new(hours: i32, minutes: i32) -> Self {
        let day_minute = (minutes + hours * Self::HOUR_MINUTES).rem_euclid(Self::DAY_MINUTES);
        Self { day_minute }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.day_minute + minutes)
    }

    pub fn hour(&self) -> i32 {
        self.day_minute.div_euclid(Self::HOUR_MINUTES)
    }

    pub fn minute(&self) -> i32 {
        self.day_minute.rem_euclid(Self::HOUR_MINUTES)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour(), self.minute())
    }
}
