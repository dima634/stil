# /bin/bash

set -e

BUILD_TYPE="${1:-debug}"

source $(dirname "$0")/build.sh ${BUILD_TYPE}

sudo cmake --build --preset ${BUILD_TYPE} --target install
pkill quickshell || true
hyprctl dispatch exec quickshell
