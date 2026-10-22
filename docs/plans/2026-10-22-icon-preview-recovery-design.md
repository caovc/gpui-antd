# Icon preview navigation recovery — Issue #31

## Confirmed failure

Cancelling the last CiTwoTone navigation leaves the requested iframe `src`
on CiTwoTone while its actual document/canvas remains SearchOutlined.
Reselecting CiTwoTone sends no request because `refreshPreview` compares only
the requested property. A real GPUI browser regression fails on that missing
retry. Explicitly reassigning the same URL recovers the selected canvas.

Offscreen/initial-document/WASM-delay probes passed with both lazy and eager
loading, so lazy loading alone is not established as the original cause.
The transport cause of the initial uninstrumented production cancellation is
unknown; the reproducible persistent failure is the shell's missing recovery.

## Ownership and behavior

Keep selection, source links and Rust code in `icons-explorer.js`. A small
`icon-preview.js` controller owns requested route, loading/ready/error state,
one 60-second readiness deadline and cleanup. It reuses the real GPUI
`gpui-antd:ready/error` messages. Accept readiness only from the current frame,
same origin, expected component, unique attempt token and exact sender/loaded
document URLs, with a nonzero
canvas and no startup overlay. A src assignment or load event is insufficient.
Each navigation gets a `previewRequest` query value echoed by `demo.js` in
both ready and error messages; even a queued error from an identical-route
retry cannot fail the new attempt.

GPUI Web initializes and focuses an IME textarea even for the icon canvas.
Remember the host's latest focused control during loading, restoring it if
startup enters the iframe. Also reconcile focus when readiness is accepted;
once ready, leave intentional preview focus alone. A real-browser gate that
finishes the prior GPUI load between focusing the next result and Enter
failed before this focus handoff and passes with it.

Hide and make the previous frame inert while loading or failed. Preserve its
layout, show progress and expose Retry after renderer failure or timeout.
Repeated normal settings deduplicate a live request; an explicit reselection
or Retry can reassign the same URL. An already painted healthy selection
does not restart. Start requested inspector navigation eagerly, including
when the old frame is hidden. Clear timers/listeners on route removal.

Do not replace the Rust demo, automatically loop failed requests, insert
arbitrary retry sleeps or duplicate the Rust renderer's state. Serializing
every selection behind an old network request would delay the latest intent;
the controller instead verifies the current document before exposing it.

## Verification

Protocol tests cover stale and forged messages, real canvas readiness,
reselection, timeout/retry, late readiness and disposal. Browser tests use
actual Rust/WASM canvases, force cancellation and delayed initial navigation,
and verify latest-selection recovery in both themes and desktop/mobile.
Keep full Native/WASM/browser/icon CI and main Pages deployment gates.
