#!/bin/bash
set -euo pipefail

rm -rf "$ENV_DIR/tmp-ui"
mkdir -p "$ENV_DIR/tmp-ui"
cp -r tests/ui/. "$ENV_DIR/tmp-ui/"

cargo fmt --all -- --check
cargo expand --test tests > "$ENV_DIR/tests_expanded.rs"
cargo check --tests
cargo clippy --tests -- -D warnings
cargo sqlx prepare -- --tests
cargo test
bash /app/tests/checksum.sh > "$ENV_DIR/checksums.txt"

rm -rf "$ENV_DIR/ui-common"
mkdir -p "$ENV_DIR/ui-common"
cp -r "$ENV_DIR/tmp-ui/." "$ENV_DIR/ui-common/"
