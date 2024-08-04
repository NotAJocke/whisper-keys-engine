#!/bin/bash

docker build -f ./docker/Dockerfile.linux-arm64 -t linux-arm64 .
docker run --rm -v "$(pwd):/app" linux-arm64

mkdir target/aarch64-unknown-linux-gnu/release/whisper-keys-engine-linux-arm64
cp target/aarch64-unknown-linux-gnu/release/whisper-keys-engine target/aarch64-unknown-linux-gnu/release/whisper-keys-engine-linux-arm64/
zip -r -9 ./target/aarch64-unknown-linux-gnu/release/whisper-keys-engine-linux-arm64.zip ./target/aarch64-unknown-linux-gnu/release/whisper-keys-engine-linux-arm64
