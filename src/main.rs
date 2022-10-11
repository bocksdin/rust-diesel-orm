use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod services;
use services::{create_user_article, fetch_user_articles, fetch_users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    HttpServer::new(move || {
        App::new()
            .service(fetch_users)
            .service(fetch_user_articles)
            .service(create_user_article)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
