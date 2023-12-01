use zero2prod::startup::run;
use std::net::TcpListener;
use sqlx::{PgPool};
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("zero2Prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_poool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let listener = TcpListener::bind(address)?;
    run(listener, connection_poool)?.await;
    Ok(())
}