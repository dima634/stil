# /bin/bash

set -e

BUILD_TYPE="${1:-dev}"

if [ "$BUILD_TYPE" = "release" ]; then
    cargo build --release
else
    cargo build
fi
cmake --preset ${BUILD_TYPE}
cmake --build --preset ${BUILD_TYPE}
