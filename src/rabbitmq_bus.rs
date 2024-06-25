use lapin::{
    options::{BasicPublishOptions, ConfirmSelectOptions, ExchangeDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties,
};

use std::{error::Error, future::Future, pin::Pin};

use crate::message_bus::MessageBus;

pub struct RabbitMQBus {
    channel: Channel,
}

impl RabbitMQBus {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://localhost:5672".into());
        let options = ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);

        let connection = Connection::connect(&addr, options).await?;
        let channel = connection.create_channel().await?;
        channel
            .confirm_select(ConfirmSelectOptions::default())
            .await?;

        Ok(Self { channel })
    }
}

impl<'a> MessageBus<'a> for RabbitMQBus {
    fn publish(
        &'a self,
        exchange: &'a str,
        routing_key: &'a str,
        message: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        let channel = &self.channel;
        Box::pin(async move {
            channel
                .exchange_declare(
                    exchange,
                    lapin::ExchangeKind::Topic,
                    ExchangeDeclareOptions::default(),
                    FieldTable::default(),
                )
                .await?;

            channel
                .basic_publish(
                    exchange,
                    routing_key,
                    BasicPublishOptions::default(),
                    message,
                    BasicProperties::default(),
                )
                .await?
                .await?;

            Ok(())
        })
    }

    fn close(&'a self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        let channel = &self.channel;
        Box::pin(async move {
            channel.close(200, "Bye").await?;
            Ok(())
        })
    }
}
