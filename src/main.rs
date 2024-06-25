use std::error::Error;

mod message_broker;
mod message_bus;
mod rabbitmq_bus;

use message_broker::MessageBroker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let broker = MessageBroker::new().await?;

    broker
        .publish("test_exchange", "test.topic", b"Hello, world!")
        .await?;

    broker.close().await?;

    Ok(())
}
