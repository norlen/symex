#!/bin/sh

# Compile all .c files.
for file in $(find tests -name '*.c'); do
    echo "generating files for $file"
    clang -c -emit-llvm $file -o ${file%.*}.bc && clang -S -emit-llvm $file -o ${file%.*}.ll
done

# Compile all .rs files used in tests.
for file in $(find tests/samples -name '*.rs'); do
    echo "generating files for $file"
    rustc --emit=llvm-bc --edition=2021 -o ${file%.*}.bc -C panic=abort $file
    rustc --emit=llvm-ir --edition=2021 -o ${file%.*}.ll -C panic=abort $file
done

# Compile all .rs files used in doctests.
for file in $(find tests/doc_tests -name '*.rs'); do
    echo "generating files for $file"
    rustc --emit=llvm-bc --edition=2021 -o ${file%.*}.bc -C panic=abort $file
    rustc --emit=llvm-ir --edition=2021 -o ${file%.*}.ll -C panic=abort $file
done
