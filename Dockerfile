FROM rust:1.79-slim

ENV ROCKET_ADDRESS=0.0.0.0

WORKDIR /app

COPY . .

RUN apt update && apt install pkg-config gcc musl-dev openssl libssl-dev -y && rm -rf /var/lib/apt/lists/*
RUN cargo build --release


EXPOSE 8000

CMD ["./target/release/muscle_booster"]