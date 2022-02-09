Build and run
    cargo build --release
    docker build -t nt .    // I noticed docker-compose doesn't build
    docker-compose up -d
    docker-compose down

Run using Cargo?
To run this daemon outside of Docker (directly on the host): 
    cargo run --release
If you need to pass an argument through cargo to the daemon use '--' e.g.:
    cargo run --release -- --bind 0.0.0.0

How to tidy up cargo lockfile and target folder?
The cargo command 'cargo generate-lockfile' will clear and regenerate your lock file. Useful to tidy up after many changes/deletions to your deps in your Cargo.toml.
The Cargo command 'cargo clean' will delete your 'target' folder resulting in the next build being a full compile of all deps.

Build and run using Docker?
This Dockerfile only packages the Cargo release build.
It uses the larger debian:buster-slim (69MB). This way you have access to a shell and can install network tools for debugging (e.g. net-tools, procps, curl, sudo). However, distroless gcr.distroless/cc (23MB) can be used.
To build this docker image: docker build -t nt .
To run this Docker image:
    docker run --rm -p 8080:8080 nt --address 0.0.0.0
    It's critical that the daemon does not listen on 127.0.0.1. This causes daemon to bind to that ip address and start listening for local traffic.
    But this is not the external ip address assigned to the container.
    Which means traffic from the host machine is forwarded to the external ip address where no one is listening. So external browser will not be able to connect. 
    Hence, use 0.0.0.0 and the daemon will bind and listen to incoming traffic from all its containers IP addresses.

Tips and tricks
To start all three services use 'docker-compose up -d' 
To stop AND remove all containers use 'docker-compose down'.
To start envoy with full debug output (trace) add to docker-compose.yaml [under envoy service] :
    command: ["-c", "/etc/envoy/envoy.yaml", "-l", "trace"]
To login into any container shell just use the Docker panel on the right, R-click over running container and 'connect with shell'.
During emergency...to install networking tools on container, login to shell then run:
    apt update
    apt install net-tools
    apt install inetutils-ping

You can control the container's dns name using the 'container_name:' attribute in docker-compose.yaml.
NOTE: 'hostname' attribute just sets what the container hallucinates as its own hostname. Nothing to do with its DNS name.
    In the event you start a shell inside the container, 'hostname' might show up in the prompt.
    It has no effect on anything outside, and there’s usually no point in setting it.
By default Docker-compose generates a DNS name '<docker_compose_folder>_<service_name>_<incrementing_number_of_containers_with_same_name>'
    For example 'project-name_daemon1_1'.
