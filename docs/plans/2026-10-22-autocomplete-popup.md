# AutoComplete native popup and editor focus

The current builder puts suggestions in normal layout. Opening it moves later
content, and there is no outside dismissal. Keep the real InputState editor and
the parent-owned open/active/value contract; replace the inline list with Kit
Base Popup, the measured/deferred/clamped native overlay primitive.

Use Base Select (the semantic foundation of Base Combobox) for the named
combobox/expanded/value surface, without adding another editor Tab stop. Keep
editor keyboard handling scoped to the InputState focus handle and guard IME
and modified keys. Space/caret clicks keep editing. Arrow keys navigate;
Enter selects once; Escape and Tab dismiss; click-away and focus loss request
closure. Option buttons do not take typing focus. The parent can reject any
request. Retained internal state is limited to scroll position, reveal caching,
and the current focus subscription/callback, not text, selection or open state.

Expose the existing four OverlayPlacement corners and a configurable semantic
label. Kit owns measurement, deferred paint, pointer occlusion and edge snapping.
Pinned Kit 0.7.3 corner placement clamps rather than automatically flipping;
document this limit and test explicit above placement/window boundaries.
Keyboard navigation reveals offscreen options using the native ScrollHandle.

The official [AutoComplete documentation](https://ant.design/components/auto-complete/)
was consulted on 2026-10-22 (site currently 6.6.7; repository catalog reference
remains 6.6.6). Preserve free-text entry, controlled open/selection requests,
hidden suggestions when no options match, clear and disabled behavior. Do not
add React props/DOM containers or claim full screen-reader/reference parity.

First reproduce layout shift and missing outside dismissal through actual
native window dispatch. Add native tests for placement/clipping, typing/caret,
keyboard/IME, focus traversal/loss, selection, disabled/empty states, controlled
rejection, independent instances and scroll reveal. Extend actual-WASM smoke
with light/dark and boundary cases. Keep the public catalog at 68, update docs
and issue #1, and require exact-head CI/review plus final-main CI/Pages proof.
