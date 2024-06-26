use std::error::Error;

mod message_broker;
mod message_bus;
mod rabbitmq_bus;

use message_broker::MessageBroker;
use message_bus::{MessageBrokerExchanges, MessageBrokerRoutingKeys};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let broker = MessageBroker::new().await?;

    publish_messages(&broker).await?;

    broker.close().await?;

    println!("Messages published");

    Ok(())
}

async fn publish_messages(broker: &MessageBroker) -> Result<(), Box<dyn Error>> {
    for i in 1..=24 {
        let message = format!("Test message {}", i);

        match (i % 6) == 0 {
            true => {
                broker
                    .publish(
                        &MessageBrokerExchanges::TestExchange,
                        &MessageBrokerRoutingKeys::TestTopicTwo,
                        message.as_bytes(),
                    )
                    .await?;
            }
            false => {
                broker
                    .publish(
                        &MessageBrokerExchanges::TestExchange,
                        &MessageBrokerRoutingKeys::TestTopic,
                        message.as_bytes(),
                    )
                    .await?;
            }
        }
    }

    broker
        .publish(
            &MessageBrokerExchanges::TestExchange,
            &MessageBrokerRoutingKeys::TestTopic,
            b"Messages are done",
        )
        .await?;
    Ok(())
}
