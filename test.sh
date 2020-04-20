#!/bin/bash

PROJECT="crate-transparency"
IMAGE="rust-trillian"
TAG=$(git rev-parse HEAD)

# Uses ./src to avoid uploading (much) context
# No context is used by the Dockerfile
# Dockerfile installs protoc and git clones the repo
docker build \
--tag=gcr.io/${PROJECT}/${IMAGE}:${TAG} \
--file=./Dockerfile.test \
./src