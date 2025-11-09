# /bin/bash

set -e

echo "Building the project..."

cargo build
cmake --preset debug
sudo cmake --build --preset debug --target install # TODO: remove sudo
