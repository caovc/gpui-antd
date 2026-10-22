# Ant theme to Kit application bridge

Follows #6 dark Tabs evidence and #3 theme infrastructure. Base: PR #33 final
head `6656704143acc58fe8ddfa642d2848af336f85ef`, rebased onto its merged
main `fabd7368d3b29a20bae7cdd974ab0b6df0f71933`.

## Problem

Gallery dark routes pass `AntTheme::dark()` to wrappers while Kit-backed
controls read the Kit application theme. Dark Tabs therefore mix bright
Card/Segmented backgrounds and near-black text/icons with dark Ant panels.
Token Studio's primary color similarly stops at wrapper-owned presentation.

## Decision

Expose `AntTheme::apply_to_kit(&self, cx)` as an explicit application-level
operation after `gpui_kit::init`, and call it once from Gallery construction
after resolving the existing Rust theme route. Use Kit `Theme::change` for
appearance and `Theme::update` for represented semantic colors and tab roles;
Kit reconciles color/background copies and synchronizes Base automatically.
Map existing Ant fields only. No token derivation, parser or JS model changes.

The operation is global to all windows in an App. Applications call it from
initialization/theme-change updates, with a single theme owner. Per-widget
`.theme(t)` remains presentation input; `RenderOnce` builders do not mutate
application theme globals. Fonts, spacing, shadows, motion and unrelated Kit
palette roles retain Kit ownership. Scoped differing Kit themes, locale,
direction, size and disabled propagation remain the ConfigProvider roadmap.

A Gallery-only mode switch would leave consumers and custom primary colors
unaddressed. A full provider would conflate application defaults with the
existing explicit configuration contract. The explicit bridge supplies the
missing seam while keeping state ownership visible.

## Verification

Context tests compare light/dark/custom palettes after repeated changes,
including Kit solid/token/Base copies. A real window retains an editor's
value and focus across theme changes and exercises Tabs afterward. Gallery
uses the same operation on native and WASM. Chromium screenshots/pixel checks
verify Tab text/background contrast and primary selection, and exercise other
Kit-backed controls, light/dark and Token Studio custom primary routes.
Formatting, workspace and Rust tests, actual WASM build, full browser suite,
exact-head CI/review and final-main deployment remain release gates.

No official screenshot parity or screen-reader completion is inferred from
this color/interaction verification. #6 and #3 retain their remaining scope.
