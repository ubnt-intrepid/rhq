FROM ubuntu:xenial

ARG TOOLCHAIN=stable

RUN apt-get update
RUN apt-get install -yq build-essential cmake curl file git g++-multilib libc6-dev-i386

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $TOOLCHAIN
RUN /root/.cargo/bin/rustup target add i686-unknown-linux-gnu
ADD cargo-config.toml /root/.cargo/config

ENV PATH /root/.cargo/bin:$PATH

WORKDIR /root/src
