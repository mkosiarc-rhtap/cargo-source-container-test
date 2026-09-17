FROM registry.access.redhat.com/ubi9/ubi-micro:latest

COPY Cargo.toml Cargo.lock requirements.txt /src/
COPY src/ /src/src/


