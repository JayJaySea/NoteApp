FROM rust as builder
WORKDIR /usr/src/myapp
COPY . .

RUN \
    --mount=type=cache,target=./target,sharing=locked\
    --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked\
        cargo install --path . --profile=release-with-debug

FROM debian:bookworm-slim
RUN apt-get update && apt-get update  && apt-get install -y postgresql-client libc6
COPY --from=builder /usr/local/cargo/bin/noteapp-backend\
                            /usr/local/bin/noteapp-backend
COPY dist /dist/
CMD ["noteapp-backend"]
