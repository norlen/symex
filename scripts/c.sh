#!/bin/sh
clang -c -emit-llvm $1 -o $2.bc && clang -S -emit-llvm $1 -o $2.ll