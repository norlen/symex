#!/bin/env bash

clang -c -emit-llvm $1 -o ${1%.*}.bc && clang -S -emit-llvm $1 -o ${1%.*}.ll
