use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Car {
    pub id: String,
    pub make: String,
    pub model: String,
    pub year: u16,
    pub color: String,
    pub price: f32,
}
