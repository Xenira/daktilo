use daktilo_shared::event::Event;

impl From<rdev::EventType> for Event {
    fn from(value: rdev::EventType) -> Self {
        match value {
            rdev::EventType::KeyPress(key) => Event::KeyPress(format!("{:?}", key)),
            rdev::EventType::KeyRelease(key) => Event::KeyRelease(format!("{:?}", key)),
            _ => Event::Unknown,
        }
    }
}
