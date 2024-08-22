// use actix_web::{App, HttpServer};

// mod services;
// mod schema;
// mod db_models;
// mod db_utils;
// use services::{create_user_article, fetch_user_articles, fetch_users};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(move || {
//         App::new()
//             .service(fetch_users)
//             .service(fetch_user_articles)
//             .service(create_user_article)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

use actix_web::{web, App, HttpServer, HttpResponse};
use crate::db::PgPool;

mod db;

async fn test_connection(pool: web::Data<PgPool>) -> HttpResponse {
    let pool = pool.clone();
    let result = web::block(move || {
        let conn = pool.get().expect("Failed to get connection from pool");
        Ok::<_, diesel::result::Error>("Database connection successful")
    }).await;

    match result {
        Ok(Ok(message)) => HttpResponse::Ok().body(message),
        Ok(Err(err)) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Blocking error: {:?}", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/test_connection", web::get().to(test_connection))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

