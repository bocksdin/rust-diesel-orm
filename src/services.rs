use actix_web::{
    get, post,
    web::{Json, Path},
    Responder,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn fetch_users() -> impl Responder {
    "GET /users".to_string()
}

#[get("/users/{id}/articles")]
pub async fn fetch_user_articles(path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    format!("GET /users/{id}/articles")
}

#[post("/users/{id}/articles")]
pub async fn create_user_article(path: Path<i32>, body: Json<CreateArticleBody>) -> impl Responder {
    let id: i32 = path.into_inner();
    format!("POST /users/{id}/articles")
}
