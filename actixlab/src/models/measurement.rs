use serde::Serialize;

#[derive(Serialize)]
pub struct Measurement {
    pub temperature: f32,
}
