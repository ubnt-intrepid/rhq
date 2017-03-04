FROM ubuntu:xenial

ARG TOOLCHAIN=stable

RUN apt-get update
RUN apt-get install -yq build-essential cmake curl file git gcc-aarch64-linux-gnu libc6-arm64-cross libc6-dev-arm64-cross

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $TOOLCHAIN
RUN /root/.cargo/bin/rustup target add aarch64-unknown-linux-gnu
ADD cargo-config.toml /root/.cargo/config

ENV PATH /root/.cargo/bin:$PATH

WORKDIR /root/src
