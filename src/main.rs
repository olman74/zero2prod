use zero2prod::startup::run;
use std::net::TcpListener;
use env_logger::Env;
use sqlx::{PgPool};
use zero2prod::configuration::get_configuration;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(),std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");

    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let connection_poool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let listener = TcpListener::bind(address)?;
    run(listener, connection_poool)?.await
}