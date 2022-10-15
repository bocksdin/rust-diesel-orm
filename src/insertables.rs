use crate::schema::articles;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=articles)]
pub struct NewArticle {
  pub title: String,
  pub content: String,
  pub created_by: i32,
}