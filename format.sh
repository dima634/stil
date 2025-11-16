# /bin/bash

pushd stil_core; cargo fmt; popd
pushd eleven; clang-format -i -- **.cpp **.h; popd
