/// Event type.
#[derive(Debug, Clone, PartialEq)]
pub enum DaktiloEvent {
    /// Key press.
    KeyPress(String),
    /// Key release.
    KeyRelease(String),
    /// Column change.
    ColumnChange(usize),
    /// Activate temoprary profile.
    ActivateTemp(String),
    /// Custom event.
    Custom {
        /// Event name.
        name: String,
        /// Event value.
        value: String,
    },
    /// Unknown event.
    Unknown,
}

impl From<rdev::EventType> for DaktiloEvent {
    fn from(value: rdev::EventType) -> Self {
        match value {
            rdev::EventType::KeyPress(key) => DaktiloEvent::KeyPress(format!("{:?}", key)),
            rdev::EventType::KeyRelease(key) => DaktiloEvent::KeyRelease(format!("{:?}", key)),
            _ => DaktiloEvent::Unknown,
        }
    }
}
