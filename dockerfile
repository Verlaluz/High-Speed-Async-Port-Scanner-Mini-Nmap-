FROM rust:latest
WORKDIR /usr/src/port_scanner
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/port_scanner"]