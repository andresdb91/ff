ARG BUILD_TARGET=x86_64-unknown-linux-musl
ARG BUILD_MSG_FORMAT=human
ARG BUILD_VERBOSITY=-q

FROM rust:alpine as chef
RUN cargo install --locked cargo-chef

FROM chef as planner
ARG BUILD_TARGET
WORKDIR /chef
COPY . .
RUN cargo chef prepare --target ${BUILD_TARGET} --recipe-path recipe.json

FROM chef as builder
ARG BUILD_TARGET
ARG BUILD_MSG_FORMAT
ARG BUILD_VERBOSITY

RUN addgroup -S -g 1000 app && adduser -S -H -s /sbin/nologin -u 1000 app -G app
WORKDIR /build

COPY --from=planner /chef/recipe.json recipe.json
RUN cargo chef cook --target ${BUILD_TARGET} --recipe-path recipe.json --release

COPY . .
RUN cargo build --workspace --target ${BUILD_TARGET} --release --message-format ${BUILD_MSG_FORMAT} ${BUILD_VERBOSITY} --locked

FROM scratch
ARG BUILD_TARGET
COPY --from=builder /etc/passwd /etc/passwd
WORKDIR /app
COPY --from=builder --chown=1000:1000 /build/target/${BUILD_TARGET}/release/ff .
USER app
ENTRYPOINT [ "/app/ff" ]
