use crate::event::Event;
use chrono::TimeZone;
use std::slice::{Iter, IterMut};

pub struct Calendar<'a, Tz: TimeZone> {
    content: Vec<Event<'a, Tz>>,
}

impl<'a, Tz: TimeZone> Calendar<'a, Tz> {
    fn iter(&self) -> Iter<'_, Event<'a, Tz>> {
        self.content.iter()
    }

    fn iter_mut(&mut self) -> IterMut<'_, Event<'a, Tz>> {
        self.content.iter_mut()
    }
}
