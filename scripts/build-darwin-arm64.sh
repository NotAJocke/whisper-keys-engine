#!/bin/bash

cargo build -r --target aarch64-apple-darwin

mkdir target/aarch64-apple-darwin/release/whisper-keys-engine-darwin-arm64
cp target/aarch64-apple-darwin/release/whisper-keys-engine target/aarch64-apple-darwin/release/whisper-keys-engine-darwin-arm64/
zip -r -9 ./target/aarch64-apple-darwin/release/whisper-keys-engine-darwin-arm64.zip ./target/aarch64-apple-darwin/release/whisper-keys-engine-darwin-arm64
