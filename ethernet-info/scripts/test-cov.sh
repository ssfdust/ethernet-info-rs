#!/bin/sh
set -e
rm -rf profiles/*
module_name=${1:-ethernet-info}
export LLVM_PROFILE_FILE="profiles/${module_name}-%p-%m.profraw"
files=$(RUSTFLAGS="-C instrument-coverage"  cargo test --tests --no-run --message-format=json 2>/dev/null | jq -r "select(.profile.test == true) | .filenames[]" | grep -v dSYM -)
RUSTFLAGS="-C instrument-coverage" cargo test --tests
grcov profiles --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html


# llvm-profdata merge -sparse profiles/*.profraw -o "profiles/${module_name}.profdata"
# llvm-cov report $( \
#     for file in $files; \
#     do \
#         printf "%s %s " -object $file; \
#     done \
# ) \
# --use-color --ignore-filename-regex='packages/cargo' --ignore-filename-regex='/.cargo/registry' --ignore-filename-regex='/.cargo/git' --ignore-filename-regex='/rustc/' \
# --instr-profile="profiles/${module_name}.profdata"
# llvm-cov show --format html $( \
#     for file in $files; \
#     do \
#         printf "%s %s " -object $file; \
#     done \
# ) \
# --use-color --ignore-filename-regex='packages/cargo' --ignore-filename-regex='/.cargo/registry' --ignore-filename-regex='/.cargo/git' --ignore-filename-regex='/rustc/' \
# --instr-profile="profiles/${module_name}.profdata" --show-instantiations --show-line-counts-or-regions > coverage.html
# rm -rf profiles
