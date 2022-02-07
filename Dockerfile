FROM debian:buster-slim

WORKDIR /app
COPY ./target/release .

ENTRYPOINT [ "/app/container_cli" ]