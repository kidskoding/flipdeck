use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Occupation {
    #[sqlx(skip)]
    pub id: Option<i32>,
    pub name: String,
}