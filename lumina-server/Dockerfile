FROM rust:1.94.0 as lumina-server

WORKDIR /app

COPY . .

RUN cargo install --path .

FROM debian:trixie
RUN apt-get update && apt-get install curl gcc ca-certificates libssl-dev -y && rm -rf /var/lib/apt/lists/*
COPY --from=lumina-server /usr/local/cargo/bin/lumina-server /usr/local/bin/lumina-server
CMD ["lumina-server"]
