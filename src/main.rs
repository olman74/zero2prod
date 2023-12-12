use zero2prod::startup::run;
use std::net::TcpListener;
use sqlx::{PgPool};
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("zero2Prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}",configuration.application.host,  configuration.application.port);
    let connection_poool = PgPool::connect_lazy(&configuration.database.connection_string()
        .expose_secret())
        .expect("Failed to connect to Postgres");
    let listener = TcpListener::bind(address)?;
    run(listener, connection_poool)?.await;
    Ok(())
}