use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use sqlx::{PgPool};
use crate::routes::subscribe;
use crate::routes::health_check;
use actix_web::dev::Server;
use actix_web::web::Data;
use tracing_actix_web::TracingLogger;


pub fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<Server> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}