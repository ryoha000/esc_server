FROM rust:latest

RUN git clone https://github.com/ryoha000/esc_server.git
RUN apt update
RUN apt install libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
WORKDIR /esc_server
RUN cargo build
