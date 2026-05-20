FROM rust:1.76

# Python + pytest + required system libs
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    python3-setuptools \
    libssl-dev && \
    pip3 install pytest

# Create required directories
RUN mkdir -p /source /target /logs/verifier

WORKDIR /workspace
