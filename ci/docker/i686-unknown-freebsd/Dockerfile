FROM japaric/i686-unknown-freebsd:latest

ARG TOOLCHAIN=stable

RUN apt-get update
RUN apt-get install -yq build-essential curl file git

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $TOOLCHAIN
ENV PATH /root/.cargo/bin:$PATH
RUN rustup target install i686-unknown-freebsd
ADD cargo-config.toml /root/.cargo/config

WORKDIR /root/src

CMD [ "/bin/bash" ]
