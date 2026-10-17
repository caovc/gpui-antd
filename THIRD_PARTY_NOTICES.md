# Third-party acknowledgements

This library is inspired by the Ant Design design language and references the
published component specification and visual tokens at https://ant.design/.

GPUI Kit and GPUI Base by Longbridge (Apache-2.0):
https://github.com/longbridge/gpui-kit

GPUI by Zed Industries (Apache-2.0):
https://github.com/zed-industries/zed

The WASM build script downloads the IBM Plex Sans subset font distributed in
GPUI Kit's gallery under the SIL Open Font License (OFL):
https://github.com/longbridge/gpui-kit/blob/main/crates/story-web/fonts/OFL.txt

No Ant Design React source code is shipped or translated here. The component
behavior and layout in this repository are native Rust implementations.

## Ant Design official SVG icons (MIT)

The icons in `crates/antd/assets/antd-icons` are original SVG assets from
[ant-design/ant-design-icons](https://github.com/ant-design/ant-design-icons),
commit `7f2516ac91226d2b41f93b35cb5197c8d94f7189`, licensed under MIT.
Icon names preserve Ant Design naming conventions. For alpha, some SVG root
fills were set to `currentColor` for GPUI theme tinting; two-tone palettes
were set to Ant Design blue. See `crates/antd/assets/LICENSE-ANT-DESIGN-ICONS`
for the complete license and copyright notice.
