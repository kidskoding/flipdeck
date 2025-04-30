use axum::{extract::Path, http::StatusCode, Extension, Json};
use sqlx::PgPool;
use crate::models::relation::Relation;

pub async fn get_relation_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Relation>>, StatusCode> {
    let id_int: i32 = id.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
    let sql = r#"
        SELECT
            c.name AS name,
            r.title AS title
        FROM relations r
        JOIN characters c ON c.id = r.relation_id
        WHERE r.character_id = $1
    "#;

    let row: Vec<Relation> = sqlx::query_as(sql)
        .bind(id_int)
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            eprintln!("{}", err);
            match err {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    Ok(Json(row))
}

pub async fn get_relation_by_name(
    Extension(pool): Extension<PgPool>,
    Path(name): Path<String>,
) -> Result<Json<Vec<Relation>>, StatusCode> {
    let sql = r#"
        SELECT
          c2.name AS name,
          r.title AS title
        FROM relations r
        JOIN characters c1 ON c1.id = r.character_id
        JOIN characters c2 ON c2.id = r.relation_id
        WHERE c1.name = $1
    "#;

    let relation: Vec<Relation> = sqlx::query_as(sql)
        .bind(name)
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            eprintln!("{}", err);
            match err {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    Ok(Json(relation))
}