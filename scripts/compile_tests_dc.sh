#!/usr/bin/env bash

SCRIPT_DIR="$(dirname $BASH_SOURCE[0])"

# We need clang-14 to generate some files, but we need libpolly-14-dev to compile llvm-sys. And
# installing libpolly-14-dev removes clang-14...
#
# So temporarily install clang-14 here, and install libpolly-14-dev again later.
apt remove libpolly-14-dev -y && apt install clang-14 -y

# Compile all .c files.
for file in $(find $SCRIPT_DIR/../tests -name '*.c'); do
    echo "generating files for $file"
    clang-14 -c -emit-llvm $file -o ${file%.*}.bc && clang-14 -S -emit-llvm $file -o ${file%.*}.ll
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
    llvm-as-14 $file
done

apt remove clang-14 -y && apt install libpolly-14-dev -y
