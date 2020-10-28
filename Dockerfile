FROM phusion/baseimage:0.10.2 as builder

ENV TERM=xterm
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
	apt-get --yes --force-yes -o Dpkg::Options::="--force-confdef" -o Dpkg::Options::="--force-confold" upgrade && \
	apt-get --yes --force-yes -o Dpkg::Options::="--force-confdef" -o Dpkg::Options::="--force-confold" dist-upgrade && \
	apt-get install -y cmake pkg-config libssl-dev git clang
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

COPY rust-toolchain rust-toolchain

RUN rustup toolchain install nightly && \
    rustup default nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly

ARG PROJECT=node-template
ARG PROFILE=release
WORKDIR /substrate

COPY . .

RUN cargo build --$PROFILE && \
	mv ./target/$PROFILE/$PROJECT /app

FROM phusion/baseimage:0.10.2
COPY --from=builder /app .
ENTRYPOINT ["/app"]
