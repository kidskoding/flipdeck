use axum::{extract::Path, http::StatusCode, Extension, Json};
use sqlx::PgPool;
use crate::models::{occupation::Occupation, relation::Relation, Character};

pub async fn get_character_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<String>
) -> Result<Json<Character>, StatusCode> {
    let id_int: i32 = id.parse().map_err(|_| StatusCode::BAD_REQUEST)?;
    let sql = r#"
        SELECT id, name, gender, debut, description 
        FROM characters
        WHERE id = $1
    "#;

    let mut character: Character = sqlx::query_as(sql)
        .bind(&id_int)
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            match err {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

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

    let relation_sql = r#"
        SELECT
          c2.name AS name,
          r.title AS title
        FROM relations r
        JOIN characters c1 ON c1.id = r.character_id
        JOIN characters c2 ON c2.id = r.relation_id
        WHERE c1.id = $1
    "#;

    let relations: Vec<Relation> = sqlx::query_as(relation_sql)
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
    
    character.occupations = Some(occupations);
    character.relations = Some(relations);

    Ok(Json(character))
}

pub async fn get_character_by_name(
    Extension(pool): Extension<PgPool>,
    Path(name): Path<String>
) -> Result<Json<Character>, StatusCode> {
    let sql = r#"
        SELECT id, name, gender, debut, description 
        FROM characters
        WHERE name = $1
    "#;

    let mut character: Character = sqlx::query_as(sql)
        .bind(&name)
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            eprintln!("{}", err);
            match err {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;

    let relation_sql = r#"
        SELECT
          c2.name AS name,
          r.title AS title
        FROM relations r
        JOIN characters c1 ON c1.id = r.character_id
        JOIN characters c2 ON c2.id = r.relation_id
        WHERE c1.name = $1
    "#;

    let relations: Vec<Relation> = sqlx::query_as(relation_sql)
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

    character.relations = Some(relations);

    Ok(Json(character))
}