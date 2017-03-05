FROM ubuntu:xenial

ARG ARCH
ARG API

# Install apt packages
RUN apt-get update && \
    apt-get install -yq \
      bison cmake curl file flex g++ \
      git pkg-config python unzip libasound2-dev \
      lib32stdc++6 lib32z1

# Install Android NDK and NDK standalone toolchain
RUN curl -qL http://dl.google.com/android/repository/android-ndk-r14-linux-x86_64.zip -o /tmp/android-ndk.zip && \
    unzip -q /tmp/android-ndk.zip -d /usr/local && \
    /usr/local/android-ndk-r14/build/tools/make_standalone_toolchain.py \
        --arch $ARCH \
        --api $API \
        --install-dir /usr/local/android-ndk-toolchain && \
    rm -rf /tmp/android-ndk.zip /usr/local/android-ndk-r14/

ENV NDK_TOOLCHAIN=/usr/local/android-ndk-toolchain
ENV PATH=$PATH:$NDK_TOOLCHAIN/bin
