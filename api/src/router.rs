use axum::{
    Extension, 
    Router,
    routing::get
};
use sqlx::PgPool;
use crate::handlers::{character::{get_character_by_id, get_character_by_name}, occupation::{get_occupations_by_id, get_occupations_by_name}, relation::{get_relation_by_id, get_relation_by_name}};

pub fn build_router(pool: PgPool) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { 
                "Hello, World!"
            })
        )
        .route(
            "/api/characters/id/{id}", 
            get(get_character_by_id)
        )
        .route(
            "/api/characters/name/{name}", 
            get(get_character_by_name)
        )
        .route(
            "/api/characters/id/{id}/relations",
            get(get_relation_by_id)
        )
        .route(
            "/api/characters/name/{name}/relations",
            get(get_relation_by_name)
        )
        .route(
            "/api/characters/id/{id}/occupations",
            get(get_occupations_by_id)
        )
        .route(
            "/api/characters/name/{name}/occupations",
            get(get_occupations_by_name)
        )
        .layer(Extension(pool))
}