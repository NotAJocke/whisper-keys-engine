use rdev::Key;
use std::fmt::Display;

pub struct KeyWrapper(pub Key);

impl Display for KeyWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl KeyWrapper {
    pub fn to_lowercase(&self) -> String {
        match self.0 {
            Key::Unknown(_) => "unknown".to_string(),
            _ => format!("{}", self).to_lowercase(),
        }
    }
}
