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
| Data Entry (18) | Checkbox, DatePicker, Form, Input, Radio, Rate, Select, Switch | AutoComplete, Cascader, ColorPicker, InputNumber, Mentions, Slider, TimePicker, Transfer, TreeSelect, Upload |
| Data Display (21) | Avatar, Badge, Card, Collapse, Descriptions, Empty, Popover, Statistic, Tag, Tooltip | Calendar, Carousel, Image, List (deprecated), Listy, QRCode, Segmented, Table, Timeline, Tour, Tree |
| Feedback (11) | Alert, Modal, Popconfirm, Progress, Result, Skeleton, Spin | Drawer, Message, Notification, Watermark |
| Other (5) | — | Affix, App, BorderBeam, ConfigProvider, Util |
| **Total (73)** | **39 alpha** | **34 missing** |

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

## Next batches (dependency order)

1. **Core form/data entry:** InputNumber, Slider, Segmented, TimePicker,
   AutoComplete. Prioritize controlled state, keyboard navigation and minimum
   text-input/drag semantics; reuse Kit models.
2. **Enterprise data display:** Table, Tree, TreeSelect, Calendar, Timeline,
   Image, Carousel. Keep Table virtualization separate from baseline table
   rendering; reuse the dedicated gpui-grid design where appropriate.
3. **Rich feedback:** Drawer, Message, Notification, richer Spin/Skeleton;
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
