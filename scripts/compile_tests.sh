#!/usr/bin/env bash

SCRIPT_DIR="$(dirname $BASH_SOURCE[0])"

# Compile all .c files.
for file in $(find $SCRIPT_DIR/../tests -name '*.c'); do
    echo "generating files for $file"
    clang -c -emit-llvm $file -o ${file%.*}.bc && clang -S -emit-llvm $file -o ${file%.*}.ll
done

opts="-C panic=abort -C link-dead-code=yes" # -C debuginfo=2"

# Compile all .rs files used in tests.
for file in $(find $SCRIPT_DIR/../tests/samples -name '*.rs'); do
    echo "generating files for $file"
    rustc --emit=llvm-bc --edition=2021 -o ${file%.*}.bc $opts $file
    rustc --emit=llvm-ir --edition=2021 -o ${file%.*}.ll $opts $file
done

# Compile all .rs files used in doctests.
for file in $(find $SCRIPT_DIR/../tests/doc_tests -name '*.rs'); do
    echo "generating files for $file"
    rustc --emit=llvm-bc --edition=2021 -o ${file%.*}.bc $opts $file
    rustc --emit=llvm-ir --edition=2021 -o ${file%.*}.ll $opts $file
done

# Compile .ll files used by unit tests
for file in $(find $SCRIPT_DIR/../tests/unit_tests -name '*.ll'); do
    echo "generating files for $file"
    llvm-as $file
done
