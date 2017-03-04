FROM ubuntu:xenial

ARG TOOLCHAIN=stable

RUN apt-get update
RUN apt-get install -yq build-essential cmake curl file git sudo

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $TOOLCHAIN
ENV PATH /root/.cargo/bin:$PATH

WORKDIR /root/src
