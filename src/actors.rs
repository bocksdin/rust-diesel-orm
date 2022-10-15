use crate::db_models::{User};
use crate::db_utils::DbActor;
use crate::schema::users::dsl::*;
use crate::messages::{FetchUser};
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchUser> for DbActor {
  type Result = QueryResult<Vec<User>>;

  fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

    users.get_results::<User>(&mut conn)
  }
}