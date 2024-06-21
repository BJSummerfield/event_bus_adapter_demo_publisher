FROM rust:latest as builder

WORKDIR /usr/src/event_bus_adapter_demo_publisher

COPY . .

RUN cargo install --path .

# Use a smaller image to run the binary
FROM debian:stable-slim

COPY --from=builder /usr/local/cargo/bin/event_bus_adapter_demo_publisher /usr/local/bin/event_bus_adapter_demo_publisher

CMD ["event_bus_adapter_demo_publisher"]

