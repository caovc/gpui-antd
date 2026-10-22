# Withdraw unusable public component placeholders

User direction: remove components that assume unsupported React behavior and
avoid exposing unusable component code. The source audit found four concrete
failures: ConfigProvider does not propagate configuration; App only wraps that
surface; Affix does not observe scrolling; Image has no image source.

Remove their implementations and related configuration-only data types,
exports, Gallery state/demos and documentation entries. Publish migration
guidance and mark these four entries as withdrawn, with 68 remaining alpha
components. Old links must explain unavailability, not display another demo.

Retain components with real native behavior and documented limits. Native
equivalents may be designed later; do not claim GPUI makes these behaviors
fundamentally impossible. Keep React-specific API parity out of the contract.

Keep public routes synchronized across Rust, docs, Studio and smoke, while
preserving the existing supported token namespaces and theme JSON validation.
Add a catalog contract test and native route rejection checks. Run Rust
format/check/tests, JS checks, actual WASM build and Chromium smoke; review the
patch and require successful CI for its exact head before merging. After merge,
verify main CI and Pages deploy, then update issues #25 and #3 with evidence.
