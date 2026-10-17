# Development, testing, CI and release guide

## Supported toolchain and prerequisites

At the root:

- Rust nightly as resolved by `rust-toolchain.toml`, edition 2024, with
  `rustfmt`, `clippy` and target `wasm32-unknown-unknown`.
- `gpui-kit = "=0.7.3"` in root `Cargo.toml`.
- `wasm-bindgen-cli` **0.2.121** (must match the Rust WASM bindings).
- Node.js 22+ with npm for the Vite docs.
- Native GPUI platform libraries and GPU backend. On Linux copy the
  packages from `.github/workflows/native.yml` rather than guessing.
- A browser with WebGPU or WebGL2 for GPUI Web; CI enables Chromium
  SwiftShader software rendering.

```bash
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install wasm-bindgen-cli --version 0.2.121 --locked

# Rust quick loop
cargo fmt --all
cargo check --workspace
cargo test -p gpui-antd --lib

# Desktop gallery
cargo run -p gpui-antd-gallery

# Actual GPU/canvas browser gallery
bash scripts/build-wasm.sh
cd website
npm install --no-audit --no-fund
npm run build
npx playwright install chromium
npm run smoke
npm run dev
```

The browser `build-wasm.sh` may download a pinned IBM Plex Sans subset on
first use and generates ignored `website/src/wasm/` bindings. Don't commit
binary fonts or generated `*.wasm` artifacts. `Cargo.lock` is not currently
committed; changes to the resolver or upstream dependencies can affect a
future nightly build, so examine CI before depending on unverified behavior.

## Typical component development change

1. Check GitHub Issues/roadmap, the current branch SHA and recent Actions
   runs. Read `AGENTS.md` and `docs/COMPONENT_PLAYBOOK.md`.
2. Inspect the installed `gpui-kit` 0.7.3 source for existing behavior
   before designing a new widget. Favor `gpui_kit::base` semantic primitives
   over custom keyboard event code unless upstream lacks the necessary
   behavior. Keep the project pinned dependency intact.
3. Add one public `AntXxx` in `crates/antd/src/<feature>.rs`; export it from
   `crates/antd/src/lib.rs`. Use doc comments, theme/size builders,
   controlled callbacks and tests of important logic.
4. Add `Gallery::<feature>_demo`, controlled state on `Gallery`, a name in
   `COMPONENTS` and a `match` arm in `Gallery::render`. The demo must
   render the Rust component, never a matching React/DOM lookalike.
5. Add a page in `website/src/main.js` `ITEMS` with summary, description,
   Rust code and public API prop table. Use `General`, `Navigation`,
   `Data Entry`, `Data Display`, `Feedback`, or `Layout` group.
   The static sidebar count in `website/index.html` and overview count in
   `website/src/main.js` must match `ITEMS.length`.
6. Add to `website/scripts/smoke.mjs`. Optionally enlarge the gallery iframe
   height in `website/src/style.css` for long demos.
7. Test: compile, run Rust unit tests, native preview, real WASM demo and
   browser smoke. Compare states against Ant Design references manually
   until screenshot parity tests exist.
8. Commit atomically where possible, and **check the Actions status for that
   exact commit SHA**. Correct CI failures and update docs if behavior changes.

## TwoTone icon color regressions

GPUI `svg()` and Kit `Icon` paint alpha masks, so TwoTone must use GPUI
`img()` to retain both filled layers. Keep the original SVG assets intact.
Palette selection and validation live in `crates/antd/src/icon_palette.rs`.
The generator matches the base `asset_path`, even though the GPUI image cache
key may include a validated `?primary=...&secondary=...` palette suffix.
Preserve this generator marker in `scripts/sync_antd_icons.py`.

Rust unit tests validate all 150 TwoTone assets in light, dark, and custom
palettes; browser smoke checks screenshot **pixels**, not only canvas startup,
for representative icon shapes and themes. Full pixel-by-pixel baseline
comparison of 848 icons remains future parity work.

## Debugging common problems

**Rust trait method unavailable:** Import `gpui_kit::Styled as _` for
`.flex()`, `.w_full()`, etc.;
`gpui_kit::prelude::FluentBuilder as _` for `.when` / `.when_some`;
`gpui_kit::{InteractiveElement as _, StatefulInteractiveElement as _}`
for pointer/focus handlers; or `gpui_kit::base::StyledExt as _` for
unstyled Base wrapper styling. This is a frequent CI failure.

**Moved builder value:** A chained method consumes its `self`. Read
`child.disabled`, `child.selected`, etc. into variables before consuming
the builder.

**WASM starts but looks blank:** Look for `#status` and
`document.querySelector('canvas')` in `website/demo.html`. GPUI
initialization is asynchronous, so `wasm.run` returning is not enough.
Use real console/page errors. Check the browser's WebGPU/WebGL2 support and
font bootstrap. Do not fake the iframe UI to make smoke green.

**Transient console error:** GPUI Kit's WASM icon loader may emit exactly
`Wasm assets loading, will be available soon...` during deferred fetches.
Only this known transient string is filtered in the browser smoke; all other
errors should be investigated.

**Artifact upload invalid path:** Screenshot names must be portable across
filesystems. `website/scripts/smoke.mjs` sanitizes names such as
`Icon:SearchOutlined` before writing a PNG. Never use colon/pipe/slash in
artifact basenames.

**Pages job fails while WASM succeeded:** Inspect separately named Actions
jobs `wasm-and-docs` and `deploy`. The former uploads a standalone artifact
even if Pages is not enabled. Pages is currently enabled in this repository.
`actions/configure-pages` or permissions may fail independently.

**Lots of canceled Pages runs:** The Pages workflow uses GitHub Actions
`concurrency` and `cancel-in-progress: true` to keep the newest deployment.
Batch changes before pushing rather than creating a rapid series of commits
that repeatedly cancels expensive WASM jobs.

**WASM package size:** All 848 MIT icon SVGs are embedded in the Rust binary.
Avoid adding a second icon registry or rendering every icon at once in a
canvas. The names are searchable in the docs; a single icon's preview loads
the GPUI gallery using `component=Icon:SearchOutlined`.

## CI requirements and how to inspect

- Native workflow: <https://github.com/caovc/gpui-antd/actions/workflows/native.yml>
- Web/docs workflow: <https://github.com/caovc/gpui-antd/actions/workflows/pages.yml>
- Icons sync: <https://github.com/caovc/gpui-antd/actions/workflows/icons-sync.yml>
- Published docs: <https://caovc.github.io/gpui-antd/>

Native CI:
`cargo check --workspace` followed by `cargo test -p gpui-antd --lib`.
The command label in the workflow might say "Unit tests (theme tokens)" but
the command actually runs **all library unit tests**.

Pages CI:
`bash scripts/build-wasm.sh`, Vite build, confirm `*.wasm` in dist,
Playwright browser smoke, screenshot Artifact, website Artifact, Pages Artifact,
separate `deploy` job.

A basic smoke test validates presence of a nonzero visible canvas and no
unhandled errors; it does **not** certify pixels are correct, focus order or
all component transitions. New components should also receive explicit
behavioral tests when possible.

## Documentation quality bar

- Public Rust usage samples should be type-correct for the pinned upstream.
- Document what is **controlled**, what the callback receives, disabled
  semantics, sizes, variants, theme tokens, focus/keyboard behavior and
  accessibility labels.
- Avoid inaccurate claims about parity: distinguish "implemented API",
  "compiles", "smoke-tested in browser" and "pixel-comparison certified".
- When a feature needs an application runtime or a browser-only capability,
  explain those constraints without hiding them from consumers.

## Security and licensing

Treat GPUI Kit/GPUI and the imported icon catalog as third-party code.
Preserve their upstream licenses and `THIRD_PARTY_NOTICES.md`.
No CDN/request to inject runtime executable scripts into native components.
Never commit secrets, download font files, release WASM blobs or node_modules.
Don't disable tests or ignore non-transient browser errors just to get a green
badge.
