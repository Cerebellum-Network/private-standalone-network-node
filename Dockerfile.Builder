FROM phusion/baseimage:0.11 as builder
LABEL maintainer "team@cere.io"
LABEL description="To create the builder image."

ARG PROFILE=release
WORKDIR /cereio
COPY ./scripts/init.sh /cereio/scripts/init.sh

RUN apt-get update && \
	apt-get upgrade -y && \
	apt-get install -y cmake pkg-config libssl-dev git clang
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
        export PATH=$PATH:$HOME/.cargo/bin && \
        scripts/init.sh