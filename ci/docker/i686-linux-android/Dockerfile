FROM ubuntu:xenial

ARG TOOLCHAIN=stable
ARG API=23

# Install apt packages
RUN apt-get update
RUN apt-get install -yq \
  bison cmake curl file flex g++ git pkg-config python unzip libasound2-dev lib32stdc++6 lib32z1

# Install Android NDK
RUN curl -qL http://dl.google.com/android/repository/android-ndk-r14-linux-x86_64.zip -o /tmp/android-ndk.zip
RUN unzip -q /tmp/android-ndk.zip -d /usr/local
RUN rm -f /tmp/android-ndk.zip
ENV NDK_HOME /usr/local/android-ndk-r14

# Install NDK standalone toolchain for i686-linux-android
RUN $NDK_HOME/build/tools/make_standalone_toolchain.py --arch x86 --api $API \
      --install-dir /usr/local/android-ndk-toolchain
ENV NDK_TOOLCHAIN   /usr/local/android-ndk-toolchain
ENV PATH $PATH:$NDK_TOOLCHAIN/bin

# Install Rust toolchain
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $TOOLCHAIN
RUN /root/.cargo/bin/rustup target add i686-linux-android
ADD cargo-config.toml /root/.cargo/config
ENV PATH /root/.cargo/bin:$PATH

RUN mkdir -p /root/src
WORKDIR /root/src
