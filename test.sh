#!/bin/bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

IMAGE_NAME=${IMAGE_NAME:-customfed:latest}
CONTAINER_NAME=${CONTAINER_NAME:-customfed}

cd ${DIR};

PROJECT=opimps

# The directory where the project will be inside the container
PROJECT_DIR=/home/user/${PROJECT}

HOST_CARGO_CACHE=${DIR}/cargo_cache
CONTAINER_CARGO_CACHE=/home/user/cargo_cache

# Store the cache on the host to reduce unnecessary downloads of dependencies
mkdir -p ${HOST_CARGO_CACHE}

docker run -u 1000:1000 -it --rm \
        -e CARGO_HOME=${CONTAINER_CARGO_CACHE} \
        -v ${DIR}/../${PROJECT}:${PROJECT_DIR}:Z \
        -v ${HOST_CARGO_CACHE}:${CONTAINER_CARGO_CACHE}:Z \
        --name ${CONTAINER_NAME} ${IMAGE_NAME} \
        /bin/bash -c "source .profile && cd ${PROJECT_DIR} && cargo test";
