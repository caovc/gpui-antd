# Ant Design 6 component coverage — gpui-antd

**Reference:** [Ant Design Components Overview](https://ant.design/components/overview/) (6.6.6, rechecked 2026-10-19 with `antd list --version 6.6.6`). `Util` is not in that catalog.

**Status definitions:** Alpha = exported native GPUI component with a real
desktop/WASM Gallery path and documented public API, but NOT full feature,
a11y or pixel parity; Missing = no equivalent public component. In particular,
an existing helper used by Gallery does not count as implementing Ant Design
Data Entry or Feedback's similarly named full component.

| Official category | Alpha in gpui-antd | Missing / not yet public |
| --- | --- | --- |
| General (4) | Button, FloatButton, Icon (Icons), Typography | — |
| Layout (7) | Divider, Flex, Grid, Layout, Masonry, Space, Splitter | — |
| Navigation (7) | Anchor, Breadcrumb, Dropdown, Menu, Pagination, Steps, Tabs | — |
| Data Entry (18) | AutoComplete, Cascader, Checkbox, ColorPicker, DatePicker, Form, Input, InputNumber, Mentions, Radio, Rate, Select, Slider, Switch, TimePicker, Transfer, TreeSelect, Upload | — |
| Data Display (21) | Avatar, Badge, Calendar, Card, Carousel, Collapse, Descriptions, Empty, Image, List, Listy, Popover, QRCode, Segmented, Statistic, Table, Tag, Timeline, Tooltip, Tour, Tree | — |
| Feedback (11) | Alert, Drawer, Message, Modal, Notification, Popconfirm, Progress, Result, Skeleton, Spin, Watermark | — |
| Other (4) | Affix, App, BorderBeam, ConfigProvider | — |
| **Total (72)** | **72 alpha** | **0 missing** |

The extra Pro Components (ProTable, ProForm, ProLayout, etc.) are separate
higher-level packages and do not count toward this 72-item core overview.
Presence is alpha, not API or pixel parity. Issue #25 records the remaining
gaps: no viewport sticky, no OS file dialog, Tour has no spotlight mask,
and BorderBeam is a moving edge segment rather than a shader stroke.
This matrix tracks **component presence**, not API completeness. A complete
Ant Design-equivalent implementation still requires multiple release cycles.

## Wave 1 added in this branch

| Component | Implemented and available | Parity gaps |
| --- | --- | --- |
| Pagination | Controlled page / pageSize callback, ranges, jump slots, simple mode, sizes, total display, disable and hide-on-single-page | Page size changer, direct jump field, arbitrary page renderer, responsive shrink |
| Steps | Horizontal/vertical, controlled selected index, optional step description/status, disabled step and GPUI-native button activation | Progress percentage, icon customization, compact dot mode, mobile behavior |
| Rate | Controlled whole stars, clear, configurable count, disable and GPUI buttons | Half stars, hover preview, custom characters, tooltips |
| Collapse | Controlled open key set, accordion or multi-open, disabled header, borderless, real native panel content | Animated motion, collapsible icon placement, nested focus-regression checks |
| Descriptions | Title, bordered/unbordered, equal-width 1–6 columns, label/value strings | Responsive columns, row/column spanning and arbitrary value elements |
| Result | Success/info/warning/error/403/404/500 status, title, subtitle and native actions | Official illustrations, full token variants |
| Skeleton | Avatar, title and 0–20 paragraph placeholders | Animated shimmer, Input/Button/Image subcomponents |
| Spin | Animated GPUI Kit Spinner, visibility, tip and native content | Container masking, nested loading overlay, custom indicators |

## Wave 2: data entry and view selection

| Component | Implemented foundation | Remaining Ant Design parity work |
| --- | --- | --- |
| InputNumber | Persistent GPUI Kit numeric state, stacked stepper buttons, placeholder synchronization, 0..100 bounds and decimal steps, status/size/disabled tokens | Precision/formats beyond Kit, exact Ant Design stepper placement, dedicated formatter/parser facade |
| Slider | Native Kit pointer/touch drag, value/range state, vertical/reverse, themed track, focusable single-value keyboard controls, caller-supplied track marks | Range keyboard thumbs, drag tooltip, custom handle formatting |
| Segmented | Controlled selection, disabled options, official icon slots, Arrow/Home/End roving focus, block and size | Animated selection indicator and more exact Ant CSS geometry |
| TimePicker | Persistent native segmented editor plus scrollable hour/minute/second and AM/PM popup columns, scroll-to-selected rows, keyboard edit, status/disabled | Presets, clear/null semantics, range selection, localization |
| AutoComplete | InputState free-form text, filtered suggestion list, click/Enter/Escape, parent-controlled open/active | Anchored collision-aware popover (current panel is inline), outside click close, full ARIA combobox, virtualization |

DatePicker retains Kit's calendar/preset panel and datetime editor so the
parent-owned state's disabled-date rules remain authoritative. Its outer field
uses Ant tokens; full Ant popup styling remains a gap in Kit 0.7.3. Formatting
and browse/open state stay with Kit rather than a second custom date model.

Dropdown uses Base Popover for focus and dismissal, shares Menu's navigation
helper for Up/Down/Home/End and skips disabled items/separators. Enter/Space
consume the host's Confirm action and select once. Regression coverage lives in
`crates/antd/tests/control_interactions.rs`; these headless native tests also
check date presets/disabled dates/datetime, time scrolling, number hints and
Tooltip side geometry. Browser smoke and pixel parity are separate checks.

Catalog popup regressions in the same native test target cover Cascader
placement, disabled options, selection and dismissal in both themes; Mentions
continuous editor typing, Escape/outside dismissal and query changes; targetless
Tour visibility; and collapsed Menu flyout geometry. When supplying a Mentions
field with `.trigger(AntInput::new(&input))`, also pass
`.track_focus(&input.read(cx).focus_handle(cx))` (import `Focusable`) so the
suggestion popover retains editor focus. Dismissal is transient element state;
the field text and selected value remain owned by the parent.

Browser smoke additionally checks Cascader, ColorPicker, TreeSelect, Image and
Tour panel pixels below the trigger, Escape and outside dismissal in light and
dark themes. These checks establish working native overlays, not full Ant
Design screenshot parity, keyboard list navigation or a screen-reader audit.

Dropdown, Popover and Popconfirm expose `.placement(OverlayPlacement)` for
`BottomLeft` (default), `BottomRight`, `TopLeft` and `TopRight`. Tooltip uses
`TooltipPlacement` with those four corner alignments plus `Top` (default),
`Bottom`, `Left` and `Right`. Kit resolves collisions; other Ant Design
placement/arrow variants and keyboard-focus tooltip activation remain open.

## Wave 3: feedback and enterprise display foundations

| Component | Initial native implementation | Explicit parity gaps |
| --- | --- | --- |
| Drawer | Root-hosted Kit Sheet, four edges, backdrop, size, resizable, close callback, GPUI-content closure | Controlled open, nested push, title actions, transition callbacks and detailed motion |
| Message | Imperative top-center Kit toast, keyed update, severity, persistent and auto-hide lifecycle | Dedicated Ant message visuals, loading, duration seconds and promise handles |
| Notification | Root-hosted stacked notification, heading/details, severity, keyed replacement and placements | Rich custom action/close controls, Ant animated stacks and per-item duration |
| Tree | Controlled hierarchy, parent-child check cascade, half-check states, strict mode, disabled barriers, focus navigation | Virtualized rows, lazy load and drag/drop |
| Table | Semantic Kit Table, native exact-match filter dropdowns, filtered pagination, numeric/text sorting, page-select and persistent row selection | Complex filters, virtualization, fixed columns, resize, edit cells, remote data |

## Wave 4: Enterprise interactions and Timeline

| Component | Added or hardened | Remaining gaps |
| --- | --- | --- |
| Tree | Cascaded check/uncheck of collapsed descendants, bottom-up parent checks, indeterminate icon, strict independent mode, disabled subtree barrier | Lazy loading, virtualization, drag/drop, fully audited treeitem ARIA check states |
| Table | Controlled per-column exact-match filters via native Dropdown menus; filter before sort/page, total reflects filtered rows; select/deselect all enabled visible page rows | Multi-choice filters, search/fuzzy predicates, fixed columns, large data virtualization, editable cells |
| Timeline | Left/Right/Alternate alignment, status dots/icons, reverse ordering, pending spinner, timestamps and native GPUI child content | Animated connector/entry motion, responsive alternate layout, exact Ant pixel parity |
| Token Studio | 50 independent component names, live Table/Tree/Timeline overrides and Rust GPU scene | Seed derivation parity, broader native consumption, global ConfigProvider |

## Wave 5: Calendar and floating actions

| Component | Native foundation | Remaining Ant Design parity |
| --- | --- | --- |
| Calendar | GPUI Kit date and date-range state, month/year navigation, keyboard focus, disabled day rules, 1–3 adjacent months, Ant surface/border/radius tokens | Date cell custom renderer, event scheduling cell, locale/week-start facades, full screenshot fidelity |
| FloatButton | GPUI Base accessible buttons, SVG icons, default/primary/danger, shape, badge, offsets, disabled and callbacks | Global fixed portal attachment, scroll-to-top integration, tooltip placement and full group motion |
| FloatButton.Group | Controlled open, native submenu actions with stable keys, close-on-select and disabled actions | Hover activation, collision/viewport positioning, animation |

## Next batches (dependency order)

1. **Data-entry parity:** AutoComplete and TimePicker popup semantics, Slider
   range keyboard, InputNumber formatter/parser. Cascader, ColorPicker,
   Mentions and TreeSelect open through base Popover; search, Transfer search
   and the Upload file dialog are still open.
2. **Enterprise data display:** Table/Tree virtualization and ARIA. Image,
   Carousel, Listy, QRCode and Tour exist as alpha; portals, autoplay and
   target measurement do not.
3. **Rich feedback:** Drawer open-state parity, Message/Notification action APIs, richer Spin/Skeleton;
   use real Kit Root overlays and focus management (never paint a fake DOM
   dialog or toast over a WASM screenshot).
4. **Alpha follow-ups:** viewport Affix, Upload file dialog, Watermark images.
   Cascader, TreeSelect, ColorPicker, Mentions, Image and Tour already open
   through base Popover.
5. **Parity hardening:** Screenshot/reference diffs against actual Ant Design,
   dark-mode contrast, accessible value/role semantics, focus restoration,
   keyboard e2e, viewport responsiveness and mobile interactions.

## Standard for claiming alpha

- Exported through crates/antd/src/lib.rs and constructed in a real Rust Gallery.
- Listed in website/src/main.js with a working WASM iframe, copied Rust sample,
  APIs and accurate partial-feature description.
- Deterministic Rust tests for the controlled transition helpers and edge cases.
- cargo check --workspace, cargo test and WASM Chromium smoke pass on that SHA.
- Document remaining accessibility and visual parity gaps rather than
  advertising 1:1 certification.
