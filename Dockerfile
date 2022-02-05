FROM rust:1.58 as build-env

WORKDIR /nt_app
COPY . /nt_app
RUN cargo build 

FROM gcr.io/distroless/cc
WORKDIR /nt_container_app
COPY --from=build-env /nt_app/target/debug/container_cli ./

CMD ["/nt_container_app/container_cli"]