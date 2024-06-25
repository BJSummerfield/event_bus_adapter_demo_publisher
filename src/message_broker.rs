use std::error::Error;
use std::future::Future;
use std::pin::Pin;

use crate::message_bus::MessageBus;
use crate::rabbitmq_bus::RabbitMQBus;

pub struct MessageBroker {
    bus: Box<dyn for<'a> MessageBus<'a> + Send + Sync>,
}

impl MessageBroker {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let bus = Box::new(RabbitMQBus::new().await?);
        Ok(Self { bus })
    }

    pub fn publish<'a>(
        &'a self,
        exchange: &'a str,
        routing_key: &'a str,
        message: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        self.bus.publish(exchange, routing_key, message)
    }

    pub fn close<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>> {
        self.bus.close()
    }
}
