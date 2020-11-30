#!/bin/bash

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
IMAGE_NAME=customfed:latest

cd ${DIR}

docker rm -f ${IMAGE_NAME}
docker build -t ${IMAGE_NAME} .
