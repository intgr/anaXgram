#!/bin/sh
cargo build --release
(for i in `seq 1 500`; do ./target/release/analgram lemmad.txt era ; done) | sort -n 2>/dev/null | head -3


