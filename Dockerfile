FROM rust:1.88-alpine as builder
WORKDIR /usr/src/undeepend
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/undeepend /usr/local/bin/undeepend
CMD ["undeepend"]

#&& apt-get install -y extra-runtime-dependencies