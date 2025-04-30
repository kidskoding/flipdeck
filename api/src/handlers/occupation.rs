use axum::{extract::Path, http::StatusCode, Extension, Json};
use sqlx::PgPool;

use crate::models::occupation::Occupation;

pub async fn get_occupations_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>
) -> Result<Json<Vec<Occupation>>, StatusCode> {
    let id_int: i32 = id.parse().unwrap();

    let occupation_sql = r#"
        SELECT
            o.occupation AS name
        FROM occupations o
        WHERE o.character_id = $1
    "#;

    let occupations: Vec<Occupation> = sqlx::query_as(occupation_sql)
        .bind(&id_int)
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            eprintln!("{}", err);
            match err {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    Ok(Json(occupations))
}

pub async fn get_occupations_by_name(
    Extension(pool): Extension<PgPool>,
    Path(name): Path<String>
) -> Result<Json<Vec<Occupation>>, StatusCode> {
    let occupation_sql = r#"
        SELECT
            o.occupation AS name
        FROM occupations o
        WHERE o.character_id = $1
    "#;

    let occupations: Vec<Occupation> = sqlx::query_as(occupation_sql)
        .bind(&name)
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            eprintln!("{}", err);
            match err {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    Ok(Json(occupations))
}