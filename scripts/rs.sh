#!/usr/bin/env bash

rustc --emit=llvm-bc --edition=2021 -o ${1%.*}.bc -C panic=abort $1
rustc --emit=llvm-ir --edition=2021 -o ${1%.*}.ll -C panic=abort $1
