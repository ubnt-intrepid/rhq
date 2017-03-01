FROM tomaka/rust-android-docker

ARG TARGET=${TARGET:-arm-linux-androideabi}
ARG ARCH=${ARCH:-arm}
ARG API=${API:-23}

RUN apt-get install -y python
RUN $NDK_HOME/build/tools/make_standalone_toolchain.py --arch $ARCH --api $API --install-dir $HOME/my-android-toolchain
RUN echo "[target.$TARGET]" > $HOME/.cargo/config
RUN echo "linker = \"$HOME/my-android-toolchain/bin/$TARGET-gcc\"" >> $HOME/.cargo/config
ENV PATH $PATH:/root/my-android-toolchain/bin

CMD /bin/bash
