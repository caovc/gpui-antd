# Native component support and withdrawn APIs

The public catalog contains **68 alpha components**. A component must perform
its core advertised operation through Rust GPUI before it is listed. A canvas,
styled surface or helper algorithm alone is insufficient. Partial keyboard,
accessibility and visual parity remains documented in
[the component matrix](COMPONENT_MATRIX.md) and the open roadmap issues.

## Withdrawn placeholders

The following four entries were removed from the crate exports, Gallery,
website navigation, search and smoke catalog. Old documentation and direct
WASM links explain that the component is unavailable; they do not render a
different component as a substitute.

| Withdrawn API | Evidence in the previous implementation | Migration |
| --- | --- | --- |
| `AntConfigProvider`, `AntConfig`, `AntDirection` | The provider painted a `div`; size was never propagated, disabled only changed opacity, and RTL only reversed the wrapper. Descendant controls did not receive configuration. | Pass `.theme(theme)`, `.size(size)` and `.disabled(disabled)` to controls that expose those builders. Keep values in the owning entity. Locale and inherited RTL have no replacement in this crate. |
| `AntApp` | Only folded children into `AntConfigProvider`; it provided no scoped message, notification or modal services. | Use an ordinary GPUI container. Initialize GPUI Kit and use the existing `AntMessage`, `AntNotification` and `AntModal` APIs with their documented Kit root requirements. |
| `AntAffix` | A bar was absolutely positioned in a fixed 180 px parent, with no scroll observation, viewport attachment or sticky behavior. | Use normal GPUI layout or an application-owned fixed header outside the scrollable content. There is no library Affix replacement. |
| `AntImage` (and its private `AntImageState` helper) | No image source API existed. Both thumbnail and preview painted `PictureOutlined`, with a popup explaining that real sources were missing. | Render actual images with GPUI's `img` element and native image sources. A full Ant-style preview/mask is not currently provided. |

This is a breaking change to early-alpha APIs. Remove imports and constructors
for these names; there are no deprecated aliases that preserve the placeholders.
React context, DOM sticky positioning and React service hooks are not assumed
to exist in GPUI. That does **not** mean native equivalents are impossible:
any reintroduction requires a native design, working state ownership, real
content and interaction tests before becoming public.

## Retained components and limits

The audit also checked the remaining catalog implementations in
`crates/antd/src/catalog.rs`. They use native primitives and remain available:

| Components | Working native foundation | Remaining limits |
| --- | --- | --- |
| Splitter, Listy | Kit resizable panels and virtual list measurement | Splitter advanced constraints; broader list keyboard/a11y behavior |
| Cascader, TreeSelect, ColorPicker, Mentions, Tour | Base Popover placement/dismissal and controlled selections; Mentions uses the parent's InputState | Search, saturation plane, full keyboard lists, external Tour targets/spotlight |
| QRCode | Actual `qrcode` matrix rendered as GPUI modules | Logo/export and visual reference tests |
| Carousel, Transfer, Anchor | Controlled index changes, key transfer and link activation | Autoplay/swipe, search and scroll-spy |
| Upload | File-list rendering/removal and a caller-owned pick callback | Application supplies real file selection and upload; no built-in OS dialog/network transport |
| Masonry, Watermark, BorderBeam | Variable-height cards in columns, repeated text marks and an animated top-edge segment | Masonry uses round-robin columns rather than height balancing; image/rotation watermarks and full-perimeter beam are absent |

These limits do not depend on pretending React APIs work. Existing native
Calendar, Input, DatePicker, Menu and Tabs also remain available. This audit
does not certify complete Ant Design parity or operating-system screen-reader
support. Follow [issue #25](https://github.com/caovc/gpui-antd/issues/25) for the
catalog roadmap and [issue #3](https://github.com/caovc/gpui-antd/issues/3) for
data components and explicit application configuration.

## Catalog verification

`npm run test:catalog` compares Rust `COMPONENTS`, documentation entries,
Studio preview choices and the displayed count. Browser smoke starts every
public native route and checks that all four withdrawn URLs report
unavailability without importing WASM. Token Studio's public preview list is
separate from its supported token namespaces; adding a preview does not claim
that every component supports per-component token overrides.
