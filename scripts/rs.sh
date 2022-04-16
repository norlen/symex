#!/bin/sh
rustc --emit=llvm-ir -C panic=abort $1 && rustc --emit=llvm-bc -C panic=abort $1
