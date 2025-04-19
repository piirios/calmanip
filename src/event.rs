use chrono::{DateTime, TimeDelta, TimeZone};
use std::ops::{Add, AddAssign};

use crate::timerange::TimeRange;

pub struct Event<'a, Tz: TimeZone> {
    range: TimeRange<Tz>,
    title: Option<&'a str>,
    description: Option<&'a str>,
}

impl<'a, Tz: TimeZone> Event<'a, Tz> {
    fn new(start: DateTime<Tz>, duration: TimeDelta) -> Event<'a, Tz> {
        Event {
            range: TimeRange::new(start, duration),
            title: None,
            description: None,
        }
    }

    fn add_title(&mut self, title: &'a str) {
        self.title = Some(title)
    }

    fn add_description(&mut self, description: &'a str) {
        self.description = Some(description)
    }
}
