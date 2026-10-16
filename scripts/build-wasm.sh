#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "wasm-bindgen-cli not found. Install: cargo install wasm-bindgen-cli --version 0.2.121 --locked" >&2
  exit 1
fi

# GPUI Web requires a real font family at initial text measurement time.
# Pin the official GPUI Kit gallery subset (SIL Open Font License).
FONT="$ROOT/crates/gallery/fonts/IBMPlexSans-Regular.ttf"
if [[ ! -s "$FONT" ]]; then
  mkdir -p "$(dirname "$FONT")"
  curl --fail --location --retry 3 --silent --show-error \
    "https://raw.githubusercontent.com/longbridge/gpui-kit/f8cf0bd964f9ef2e5b7be79f3a8fc391371feb4a/crates/story-web/fonts/IBMPlexSans-Regular.ttf" \
    --output "$FONT"
fi

echo "Compiling GPUI Ant Design gallery to wasm32-unknown-unknown ..."
cargo rustc --release -p gpui-antd-gallery --lib --target wasm32-unknown-unknown -- \
  -C link-arg=-zstack-size=8388608

SOURCE="$ROOT/target/wasm32-unknown-unknown/release/gpui_antd_gallery.wasm"
OUTPUT="$ROOT/website/src/wasm"
if [[ ! -s "$SOURCE" ]]; then
  echo "Missing compiled WASM module: $SOURCE" >&2
  exit 1
fi
mkdir -p "$OUTPUT"
wasm-bindgen "$SOURCE" --out-dir "$OUTPUT" --target web --no-typescript

echo "Generated real GPUI browser module under website/src/wasm/"
