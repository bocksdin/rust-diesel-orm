use crate::db_models::{User, Article};
use crate::db_utils::DbActor;
use crate::schema::users::dsl::*;
use crate::schema::articles::{dsl::*, id as article_id};
use crate::messages::{FetchUser, FetchUserArticles, CreateArticle};
use crate::insertables::NewArticle;
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchUser> for DbActor {
  type Result = QueryResult<Vec<User>>;

  fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

    users.get_results::<User>(&mut conn)
  }
}

impl Handler<FetchUserArticles> for DbActor {
  type Result = QueryResult<Vec<Article>>;

  fn handle(&mut self, msg: FetchUserArticles, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch User Articles: Unable to establish connection");

    articles.filter(created_by.eq(msg.user_id)).get_results::<Article>(&mut conn)
  }
}

impl Handler<CreateArticle> for DbActor {
  type Result = QueryResult<Article>;

  fn handle(&mut self, msg: CreateArticle, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Create User Article: Unable to establish connection");

    let new_article = NewArticle {
      title: msg.title,
      content: msg.content,
      created_by: msg.created_by,
    };

    diesel::insert_into(articles)
      .values(new_article)
      .returning((
        article_id,
        title,
        content,
        created_by,
        created_on.nullable(),
      ))
      .get_result::<Article>(&mut conn)
  }
}