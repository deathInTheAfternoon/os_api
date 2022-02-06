FROM gcr.io/distroless/cc

WORKDIR /app
COPY ./target/release .

CMD [ "/app/container_cli" ]