use serde::{Serialize, Deserialize};

pub mod message;
#[cfg(feature = "ssr")]
pub mod server;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Option<Self> {
        match name {
            "" => None,
            name => Some(Name(name.to_string())),
        }
    }

    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}
