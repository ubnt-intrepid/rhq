FROM ubuntu:xenial

ARG TOOLCHAIN=stable

RUN apt-get update
RUN apt-get install -yq build-essential cmake curl file git musl-tools sudo xutils-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $TOOLCHAIN
RUN /root/.cargo/bin/rustup target add x86_64-unknown-linux-musl
ADD cargo-config.toml /root/.cargo/config

ENV PATH /root/.cargo/bin:$PATH

WORKDIR /root/src
