#!/usr/bin/env bash

cargo rustc $1 $2 -- --emit=llvm-ir --emit=llvm-bc -C panic=abort 
