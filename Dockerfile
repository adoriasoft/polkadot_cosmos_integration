FROM alpine:3.12

RUN apk add --no-cache ca-certificates gcc g++ \
        make alpine-sdk git musl-dev cmake wget \
        build-base python3-dev

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.44.1 \
    RUST_BACKTRACE=full

RUN set -eux; \
    url="https://static.rust-lang.org/rustup/archive/1.21.1/x86_64-unknown-linux-musl/rustup-init"; \
    wget "$url"; \
    echo "0c86d467982bdf5c4b8d844bf8c3f7fc602cc4ac30b29262b8941d6d8b363d7e *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

COPY . ./

RUN rustup default stable; \
    rustup update nightly; \
    rustup update stable; \
    rustup target add wasm32-unknown-unknown --toolchain nightly; \
    rustup update

WORKDIR  ./substrate
RUN cargo clean; \
    cargo build --release

CMD ["cargo","test","--all"]
