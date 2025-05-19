use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use super::{occupation::Occupation, relation::Relation};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Character {
    pub id: i64,
    pub name: String,
    pub gender: String,
    pub appearance: Option<String>,
    pub debut: String,
    pub description: Option<String>,
    pub image_url: String,
    #[sqlx(skip)]
    pub occupations: Option<Vec<Occupation>>,
    #[sqlx(skip)]
    pub relations: Option<Vec<Relation>>,
}