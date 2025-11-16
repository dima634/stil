# /bin/bash

pushd core; cargo fmt; popd
pushd eleven; clang-format -i -- **.cpp **.h; popd
