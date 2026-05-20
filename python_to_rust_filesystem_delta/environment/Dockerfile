FROM rust:1.76

# Python + pytest
RUN apt-get update && apt-get install -y python3 python3-pip && \
    pip3 install pytest

# Create required directories
RUN mkdir -p /source /target /logs/verifier

WORKDIR /workspace
