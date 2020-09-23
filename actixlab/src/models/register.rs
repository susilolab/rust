use serde::Deserialize;

#[derive(Deserialize)]
pub struct Register {
    pub username: String,
    pub country: String,
}
