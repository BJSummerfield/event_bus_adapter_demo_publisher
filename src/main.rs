use lapin::{
    options::{BasicPublishOptions, ConfirmSelectOptions, ExchangeDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use std::time::Instant;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://localhost:5672".into());

    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let connection = Connection::connect(&addr, options).await?;
    let channel = connection.create_channel().await?;

    channel
        .confirm_select(ConfirmSelectOptions::default())
        .await?;

    let exchange_name = "test_exchange";
    let healthcheck_routing_key = "test.healthcheck";
    let regular_routing_key = "test.topic";

    channel
        .exchange_declare(
            exchange_name,
            lapin::ExchangeKind::Topic,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let payload = b"Health check";

    let mut attempts = 0;
    let max_attempts = 10;
    let interval = Duration::from_secs(5);

    while attempts < max_attempts {
        let start_time = Instant::now();
        let confirm = channel
            .basic_publish(
                exchange_name,
                healthcheck_routing_key,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await?
            .await?;

        println!("Confirm: {:?}", confirm);
        if confirm.is_ack() {
            println!(
                "Health check message acknowledged on attempt {}.",
                attempts + 1
            );
            break;
        } else {
            println!(
                "Health check message not acknowledged on attempt {}. Retrying...",
                attempts + 1
            );
            attempts += 1;
            let elapsed = start_time.elapsed();
            if elapsed < interval {
                sleep(interval - elapsed).await;
            }
        }
    }

    if attempts == max_attempts {
        eprintln!(
            "Health check message was not acknowledged after {} attempts. Exiting.",
            max_attempts
        );
        std::process::exit(1);
    }

    println!("Subscriber is confirmed ready. Now publishing regular messages...");

    let payload = b"Hello, world!";
    channel
        .basic_publish(
            exchange_name,
            regular_routing_key,
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default(),
        )
        .await?
        .await?;

    println!(
        "Message published with routing key: {}",
        regular_routing_key
    );

    channel.close(200, "Bye").await?;

    Ok(())
}
