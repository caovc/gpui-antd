# Ant Design 6 component coverage — gpui-antd

**Reference:** [Ant Design Components Overview](https://ant.design/components/overview/) (6.6.6, consulted 2026-10-17).

**Status definitions:** Alpha = exported native GPUI component with a real
desktop/WASM Gallery path and documented public API, but NOT full feature,
a11y or pixel parity; Missing = no equivalent public component. In particular,
an existing helper used by Gallery does not count as implementing Ant Design
Data Entry or Feedback's similarly named full component.

| Official category | Alpha in gpui-antd | Missing / not yet public |
| --- | --- | --- |
| General (4) | Button, Icon (Icons), Typography | FloatButton |
| Layout (7) | Divider, Flex, Grid, Layout, Space | Masonry, Splitter |
| Navigation (7) | Breadcrumb, Dropdown, Menu, Pagination, Steps, Tabs | Anchor |
| Data Entry (18) | AutoComplete, Checkbox, DatePicker, Form, Input, InputNumber, Radio, Rate, Select, Slider, Switch, TimePicker | Cascader, ColorPicker, Mentions, Transfer, TreeSelect, Upload |
| Data Display (21) | Avatar, Badge, Card, Collapse, Descriptions, Empty, Popover, Segmented, Statistic, Table, Tag, Tooltip, Tree | Calendar, Carousel, Image, List (deprecated), Listy, QRCode, Timeline, Tour |
| Feedback (11) | Alert, Drawer, Message, Modal, Notification, Popconfirm, Progress, Result, Skeleton, Spin | Watermark |
| Other (5) | — | Affix, App, BorderBeam, ConfigProvider, Util |
| **Total (73)** | **49 alpha** | **24 missing** |

The extra Pro Components (ProTable, ProForm, ProLayout, etc.) are separate
higher-level packages and do not count toward this 73-item core overview.
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
| InputNumber | Persistent GPUI Kit numeric state, stepper buttons, 0..100 bounds and decimal steps, status/size/disabled tokens | Precision/formats beyond Kit, exact Ant Design stepper placement, dedicated formatter/parser facade |
| Slider | Native Kit pointer/touch drag, value/range state, vertical/reverse, themed track, focusable single-value keyboard controls | Range keyboard thumbs, marks, tooltips, custom handle formatting |
| Segmented | Controlled selection, disabled options, official icon slots, Arrow/Home/End roving focus, block and size | Animated selection indicator and more exact Ant CSS geometry |
| TimePicker | Persistent native segmented time editor, minute/second, AM/PM, keyboard edit, status/disabled | Clock popup, presets, clear/null semantics, range selection, localization |
| AutoComplete | InputState free-form text, filtered suggestion list, click/Enter/Escape, parent-controlled open/active | Anchored collision-aware popover (current panel is inline), outside click close, full ARIA combobox, virtualization |

## Wave 3: feedback and enterprise display foundations

| Component | Initial native implementation | Explicit parity gaps |
| --- | --- | --- |
| Drawer | Root-hosted Kit Sheet, four edges, backdrop, size, resizable, close callback, GPUI-content closure | Controlled open, nested push, title actions, transition callbacks and detailed motion |
| Message | Imperative top-center Kit toast, keyed update, severity, persistent and auto-hide lifecycle | Dedicated Ant message visuals, loading, duration seconds and promise handles |
| Notification | Root-hosted stacked notification, heading/details, severity, keyed replacement and placements | Rich custom action/close controls, Ant animated stacks and per-item duration |
| Tree | Controlled hierarchy, expand/selected/checked keys, disabled nodes, focus navigation | Cascading/indeterminate check, virtualized rows, lazy load and drag/drop |
| Table | Semantic Kit Table, numeric/text sorting, controlled row selection, pagination, striped mode | Filtering, virtualization, fixed columns, resize, edit cells, remote data |

## Next batches (dependency order)

1. **Remaining core data entry:** Cascader, ColorPicker, Mentions, Transfer,
   TreeSelect and Upload. Finish AutoComplete and TimePicker popup semantics,
   Slider range keyboard support, and InputNumber formatter/parsing APIs.
2. **Enterprise data display:** enhance Table/Tree virtualization and ARIA,
   implement TreeSelect, Calendar, Timeline, Image and Carousel. Reuse gpui-grid
   for high-volume tables, rather than reimplementing its engine.
3. **Rich feedback:** finish Drawer open-state parity, Message/Notification action APIs, richer Spin/Skeleton;
   use real Kit Root overlays and focus management (never paint a fake DOM
   dialog or toast over a WASM screenshot).
4. **Advanced/complex:** Upload, Transfer, Cascader, ColorPicker, Mentions,
   Tour, QRCode, Masonry, Splitter, Watermark, Affix, ConfigProvider and App.
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
