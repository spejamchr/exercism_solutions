#[derive(Debug)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

impl Clock {
    pub fn new(raw_hours: i32, raw_minutes: i32) -> Self {
        let minutes = modulo(raw_minutes, 60);
        let hours = modulo(raw_hours + (raw_minutes as f32 / 60.0).floor() as i32, 24);
        Self { minutes, hours }
    }

    pub fn add_minutes(&self, raw_minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + raw_minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, o: &Self) -> bool {
        self.minutes == o.minutes && self.hours == o.hours
    }
}
