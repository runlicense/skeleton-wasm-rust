#!/usr/bin/env bash
set -e

# Read product name from runlicense.json and convert to snake_case crate name
PRODUCT_NAME=$(python3 -c "import json; print(json.load(open('runlicense.json'))['product_name'])")
CRATE_NAME=$(echo "$PRODUCT_NAME" | tr '[:upper:]' '[:lower:]' | tr ' -' '__')

# Update Cargo.toml package name to match the product
sed -i '' "s/^name = .*/name = \"${CRATE_NAME}\"/" Cargo.toml

wasm-pack build --target web

cargo run --manifest-path ../runlicense/sdk-webassembly-rust/Cargo.toml --features tools --bin generate_manifest -- \
  "$(pwd)/pkg/${CRATE_NAME}_bg.wasm" \
  --src "$(pwd)/src"

echo "Done — pkg/ is ready"
