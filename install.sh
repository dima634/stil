# /bin/bash

set -e

cargo build
cmake --preset debug
cmake --build --preset debug;
sudo cmake --build --preset debug --target install

pkill quickshell || true
hyprctl dispatch exec quickshell
