use crate::db_models::{User, Article};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUser;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Article>>")]
pub struct FetchUserArticles {
  pub user_id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Article>")]
pub struct CreateArticle {
  pub title: String,
  pub content: String,
  pub created_by: i32,
}