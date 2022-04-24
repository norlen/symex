#!/bin/sh

# Compile all .c files.
for sample in $(find samples -name '*.c'); do
    echo "generating files for $sample"
    clang -c -emit-llvm $sample -o ${sample%.*}.bc && clang -S -emit-llvm $sample -o ${sample%.*}.ll
done
