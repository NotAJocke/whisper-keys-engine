#!/bin/bash

docker build -f ./docker/Dockerfile.linux-amd64 -t linux-amd64 .
docker run --rm -v "$(pwd):/app" linux-amd64
