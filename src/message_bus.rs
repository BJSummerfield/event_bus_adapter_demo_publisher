use std::{error::Error, future::Future, pin::Pin};

pub trait MessageBus<'a> {
    fn publish(
        &'a self,
        exchange: &'a str,
        routing_key: &'a str,
        message: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>>;
    fn close(&'a self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + 'a>>;
}
