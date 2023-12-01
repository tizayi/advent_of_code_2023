FROM docker.io/library/rust:1.73.0-bullseye

RUN rustup component add rustfmt clippy

RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
    libopencv-dev clang libclang-dev \
    && rm -rf /var/lib/apt/lists/*