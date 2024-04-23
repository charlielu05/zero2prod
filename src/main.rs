use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read config");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    let connection_string: String = configuration
        .database
        .connection_string()
        .expose_secret()
        .to_string();
    let connection_pool =
        PgPool::connect_lazy(&connection_string).expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await?;
    Ok(())
}
