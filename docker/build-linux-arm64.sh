#!/bin/bash

docker build -f ./docker/Dockerfile.linux-arm64 -t linux-arm64 .
docker run --rm -v "$(pwd):/app" linux-arm64
