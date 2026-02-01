#!/usr/bin/env bash

# Rebuild the binary
cargo clean
cargo build --profile asm

# Output file
OUT_FILE="fns.asm"

# Clear previous output
> "$OUT_FILE"

# Starting and ending indices for the output of "cargo asm --lib"
START_INDEX=4
LAST_INDEX=$(cargo asm --lib | awk '/^[[:space:]]*[0-9]+ "/ {i=$1} END {print i}')

# Loop through all indices and save outputs to file
for i in $(seq "$START_INDEX" "$LAST_INDEX"); do
    cargo asm --profile asm --lib "$i" >> "$OUT_FILE"
    echo -e "\n\n" >> "$OUT_FILE"
done

# filter out certain liens from the output file
grep -v -e ".section" -e ".globl" -e ".p2align" -e ".type" "$OUT_FILE" > "$OUT_FILE.tmp"
mv "$OUT_FILE.tmp" "$OUT_FILE"

echo "Assembly written to $OUT_FILE"
