#!/bin/sh
cargo rustc -- --emit=llvm-ir -emit=llvm-bc -C panic=abort 
