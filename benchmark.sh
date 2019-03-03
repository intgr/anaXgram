#!/bin/sh
set -e
cargo build --release
LOOPS=${1:-500}
(for i in `seq 1 $LOOPS`; do ./target/release/analgram lemmad.txt era ; done) | sort -n 2>/dev/null | head -3
