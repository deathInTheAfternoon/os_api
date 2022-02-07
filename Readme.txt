The cargo command 'cargo generate-lockfile' will clear and regenerate your lock file. Useful to tidy up after many changes/deletions to your deps in your Cargo.toml.
The Cargo command 'cargo clean' will delete your 'target' folder resulting in the next build being a full compile of all deps.
To run this daemon outside of Docker: cargo run --release
    If you need to pass an argument through cargo to the daemon use '--' e.g.:
        cargo run --release -- --bind 0.0.0.0

This Dockerfile only packages the Cargo release build.
It uses the larger debian:buster-slim (69MB). This way you have access to a shell and can install network tools for debugging (e.g. net-tools, procps, curl, sudo). However, distroless gcr.distroless/cc (23MB) can be used.
To build this docker image: docker build -t nt .
To run this Docker image:
    docker run --rm -p 8080:8080 nt --address 0.0.0.0
    It's critical that the daemon does not listen on 127.0.0.1. This causes daemon to bind to that ip address and start listening for local traffic. But this is not the external ip address assigned to the container.
    Which means traffic from the host machine is forwarded to the external ip address where no one is listening. So external browser will not be able to connect. 
    Hence, use 0.0.0.0 and the daemon will bind and listen to incoming traffic from all its containers IP addresses.