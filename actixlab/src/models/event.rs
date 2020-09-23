use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub id: Option<i32>,
    pub timestamp: f64,
    pub kind: String,
    pub tags: Vec<String>,
}
