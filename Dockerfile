FROM phusion/baseimage:0.11 as builder
LABEL maintainer "team@cere.io"
LABEL description="This is the build stage to create the binary."

ARG PROFILE=release
WORKDIR /cereio

COPY . /cereio

RUN apt-get update && \
	apt-get upgrade -y && \
	apt-get install -y cmake pkg-config libssl-dev git clang
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
        export PATH=$PATH:$HOME/.cargo/bin && \
        scripts/init.sh && \
        cargo +nightly-2020-10-06 build --$PROFILE

# ===== SECOND STAGE ======
FROM phusion/baseimage:0.11
LABEL maintainer "team@cere.io"
LABEL description="This is the optimization to create a small image."
ARG PROFILE=release
COPY --from=builder /cereio/target/$PROFILE/node-template /usr/local/bin

RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	rm -rf /usr/lib/python* && \
	useradd -m -u 1000 -U -s /bin/sh -d /cereio cereio && \
	mkdir -p /cereio/.local/share/cereio && \
	chown -R cereio:cereio /cereio/.local && \
	ln -s /cereio/.local/share/cereio /data && \
	rm -rf /usr/bin /usr/sbin

USER cereio
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/node-template"]