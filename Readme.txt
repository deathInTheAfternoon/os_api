To run this daemon outside of Docker: cargo run --release
    If you need to pass an argument through cargo to the daemon use '--':
    cargo run --release -- --bind 0.0.0.0

This Dockerfile only packages the Cargo release build.
To build this docker image: docker build -t nt .
To run this Docker image: docker run --rm -p 8080:8080 nt --address 0.0.0.0
    It's critical that you do not use 127.0.0.1. This causes daemon to bind to that ip address. But this is not the external ip address assigned to the container. 
    So external browser will not be able to connect. Hence, use 0.0.0.0 and the daemon will bind to all IP addresses.