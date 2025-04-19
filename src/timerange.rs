use chrono::{DateTime, TimeDelta, TimeZone};
use std::ops::{Add, AddAssign};

pub struct TimeRange<Tz: TimeZone> {
    start: DateTime<Tz>,
    duration: TimeDelta,
}

impl<Tz: TimeZone> TimeRange<Tz> {
    pub fn new(start: DateTime<Tz>, duration: TimeDelta) -> Self {
        Self { start, duration }
    }
}

impl<Tz: TimeZone> Add<TimeDelta> for TimeRange<Tz> {
    type Output = TimeRange<Tz>;
    fn add(self, rhs: TimeDelta) -> Self::Output {
        TimeRange {
            duration: self.duration + rhs,
            ..self
        }
    }
}

impl<Tz: TimeZone> AddAssign<TimeDelta> for TimeRange<Tz> {
    #[inline]
    fn add_assign(&mut self, rhs: TimeDelta) {
        self.duration += rhs;
    }
}
