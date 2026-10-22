# Tabs interaction verification

`AntTabs` keeps the existing controlled index API and pinned GPUI Kit 0.7.3
TabBar/Tab rendering. The owner applies requested indices and calls
`cx.notify()`. Focus handles survive ordinary controlled rerenders.

## Keyboard and activation

- Left/Right wrap and skip disabled tabs. Home/End target the first/last
  enabled tab. A target already focused still requests selection when the
  owner's controlled index differs.
- Enter/Space use GPUI's native synthesized key-up click. Keyboard activation
  of the already-selected tab is a no-op. Pointer activation retains its
  callback, including clicking the selected tab.
- Only the selected enabled tab is a Tab stop; an invalid/disabled selection
  falls back to the first enabled tab for focus. All-disabled groups have no
  tab stop or selected panel.
- Tab leaves the bar for panel controls. Shift-Tab returns through the panel
  to the selected tab. Panel editors keep their own arrow/Home/End behavior.
  `.keyboard(false)` keeps pointer activation and excludes tabs from Tab order.

## Evidence

`cargo test -p gpui-antd --test tabs_interactions` exercises the production
wrapper with real GPUI window events. Eight tests check callbacks, selection,
focus handles, disabled rejection, invalid selection, keyboard opt-out,
confirmation on release/cancellation, modified keys, independent groups and an actual InputState in the panel.
The navigation/selected-panel test runs all four variants, three control
sizes and both Ant themes (24 combinations). These are interaction/geometry
tests, not reference-image parity tests.

The tests reproduced missing Home/End requests when focus was ahead of the
owner and duplicate Enter/Space requests from two activation paths. The
regressions failed before their fixes.

`website/scripts/tabs-smoke.mjs`, invoked by `npm run smoke`, drives the actual
Rust/WASM Gallery at 900×550 in light/dark. It compares painted panel regions
after Right/Left wrap and Home/End, Tab/Space switch activation, panel key
isolation, Shift-Tab return and state retained after the panel remounts.
Screenshots go to the existing smoke artifact directory. No DOM stand-ins or
test-only state bridge is used. The Gallery switch now stores requested values
in its owner rather than always rebuilding with `checked(true)`.

## Accessibility and remaining work — issue #6

Native snapshots verify Tab roles, labels and selected state. The fixed Kit
version omits the disabled accessibility flag despite rejecting disabled
input. Its test registration cannot observe focus handles attached before
internal registration, so tests inspect the actual window FocusHandle and
pair it with selection, Tab order and panel behavior instead of inventing
focus metadata.

TabList/TabPanel role emission follows the existing Rust composition; full
ID relationships (`aria-controls`/`aria-labelledby`), a native disabled flag,
practical OS screen-reader announcements and official Ant Design screenshot
comparisons remain open. Chromium comparisons check behavior across states,
not equivalence to an official reference. Issue #6 stays open.

The dark Gallery screenshot also shows an existing theme mismatch: Kit-owned
tab colors follow the Kit application theme, while `.theme(AntTheme::dark())`
sets wrapper/panel colors. Card/Segmented backgrounds and some labels/icons
therefore still have poor dark contrast. Both-theme interaction tests do not
certify dark visual parity. A Kit/Ant application-theme bridge needs its own
coverage across the Gallery's other Kit-backed components.
