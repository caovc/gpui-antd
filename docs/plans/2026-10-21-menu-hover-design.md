# Menu hover delays — Issue #8

Keep one native AntMenu implementation for desktop and WASM. Add
`AntMenuTrigger::Hover` as an explicit popup-menu option; preserve the existing
`Click` default. Inline groups continue to expand by activation.

## Approach

Retain GPUI Base Popover for positioning, keyboard dismissal, focus and overlays.
Store only transient pointer intent and a cancellable foreground task in keyed
element state. The parent Entity still owns open keys and selection.

Alternatives considered:

- Base HoverCard already has hover timers, but has no controlled open API in
  Kit 0.7.3 and would replace existing keyboard/click behavior.
- Gallery-owned timers would leave downstream consumers without the behavior.

## Contract

- Open/close delay builders take Rust Duration. Defaults are zero and 100 ms,
  matching the reference Menu delay values, while activation remains opt-in.
- Leaving before open cancels the request. Entering the panel cancels close;
  leaving both trigger and panel starts the close grace period.
- Click and keyboard remain immediate. Escape/outside dismissal cancels pending
  work and cannot reopen the same hovered trigger until the pointer exits.
- Disabling a submenu, switching activation mode, or unmounting cancels tasks.
- Hover requests do not select routes or mutate parent-owned open keys.
- Reconcile a late parent acceptance with current pointer intent, while a new
  keyboard/click activation supersedes the earlier hover request.
- Descendants currently expand inline inside a flyout; preserve that behavior
  and test that traversing nested rows keeps the ancestor open.

## Verification

Native UI integration tests advance the executor clock and dispatch real hover,
click and key events. Cover delay boundaries, rapid traversal, entering the
panel, re-entry, disabled/removed controls, controlled refusal, and click mode.
Add a real Rust Gallery hover demo and Chromium canvas pixel checks in light
and dark themes. Keep docs and exported APIs synchronized.

Horizontal overflow, typeahead, advanced nested collision, RTL and screen-reader
parity remain outside this batch. GPUI Base retains its existing popup focus
policy; this change is not official screenshot or accessibility certification.

Reference: https://ant.design/components/menu/
