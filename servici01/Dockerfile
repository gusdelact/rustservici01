FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/servici01 /usr/local/bin/servici01
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["servici01"]
