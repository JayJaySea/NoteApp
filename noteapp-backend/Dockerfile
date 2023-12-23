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
CMD ["noteapp-backend"]

#FROM alpine
#RUN apk update; apk add libgcc; apk add gcompat; apk add postgresql-client
