# Tabs window interaction verification

Issue: #6. Base: `864480cb5a258434d59bb656a618e15adf7c9a3f`.

## Contract and approach

Keep the existing public controlled API and pinned Kit TabBar/Tab rendering.
The owner applies requested indices and notifies; stable per-tab focus handles
remain in keyed window state. Native tests must dispatch real pointer and key
events and assert owner callbacks, selected/disabled semantics and focus.

Helper-only tests cannot establish event routing. Screenshot-only browser
tests cannot establish callback counts or native focus. Use native window
integration tests for those facts, retain actual WASM browser checks for paint,
and extend the existing Gallery's real panel controls for end-to-end behavior.
Do not introduce a parallel DOM component, renderer bridge state or new GPUI
dependency.

## Verification

- Pointer selection and disabled rejection; selected-panel replacement.
- Left/Right, Home/End, wrap and disabled skipping, including parent-delayed
  selection. Moving focus and requesting selection are independent operations:
  Home/End must request a nonselected target even when already focused there.
- Enter/Space activation from focus ahead of controlled selection; unchanged
  selection does not produce duplicate keyboard callbacks.
- Tab/Shift-Tab leave/reenter the selected tab; panel controls keep their own
  keys, state and focus across owner rerenders. Multiple groups stay isolated.
- Keyboard opt-out, all-disabled/invalid selection, light/dark and all four
  existing variants and control sizes as relevant to these contracts.
- Formatting, workspace/library tests, native interaction tests and CI; real
  WASM build, full Chromium smoke, final main audit and deployment when merged.

## Remaining issue scope

Native role/selection assertions are not a practical screen-reader audit.
Official Ant Design reference screenshots, all accessibility ID relations and
OS assistive-technology behavior remain open under #6 unless independently
verified. This change must document exact results without closing that issue
prematurely.
