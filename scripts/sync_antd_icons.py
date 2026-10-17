#!/usr/bin/env python3
"""Vendor every official Ant Design SVG, generate the Rust registry and manifest.

The supplied ant-design/ant-design-icons checkout must be the pinned MIT revision.
Only Python's standard library is needed.
"""
from __future__ import annotations
import argparse
import json
import re
import subprocess
from pathlib import Path

PIN = "7f2516ac91226d2b41f93b35cb5197c8d94f7189"
STYLES = ("outlined", "filled", "twotone")
SUFFIX = {"outlined": "Outlined", "filled": "Filled", "twotone": "TwoTone"}
ROOT = Path(__file__).resolve().parent.parent
ASSET_ROOT = ROOT / "crates/antd/assets/antd-icons"
RUST = ROOT / "crates/antd/src/icons.rs"
MANIFEST = ROOT / "website/public/antd-icons.json"


def rust_name(style: str, slug: str) -> str:
    if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", slug):
        raise ValueError(f"Unsupported upstream icon filename: {slug!r}")
    result = "".join(part[0].upper() + part[1:] for part in slug.split("-"))
    if result[0].isdigit():
        result = "_" + result
    return result + SUFFIX[style]


def normalize(svg: str, style: str) -> str:
    if "<svg" not in svg or "</svg>" not in svg:
        raise ValueError("Invalid SVG payload")
    if not re.search(r"<svg[^>]*\sfill=", svg, flags=re.I):
        svg = svg.replace("<svg", '<svg fill="currentColor"', 1)
    if style == "twotone":
        svg = re.sub(r"#333(?![0-9a-f])", "#1677ff", svg, flags=re.I)
        svg = re.sub(r"#e6e6e6", "#e6f4ff", svg, flags=re.I)
    return svg


def generate(upstream: Path) -> None:
    actual = subprocess.check_output(
        ["git", "-C", str(upstream), "rev-parse", "HEAD"], text=True
    ).strip()
    if actual != PIN:
        raise SystemExit(f"Expected official revision {PIN}; got {actual}")
    entries = []
    seen = set()
    for style in STYLES:
        directory = upstream / "packages/icons-svg/svg" / style
        if not directory.is_dir():
            raise SystemExit(f"Missing icon directory {directory}")
        paths = sorted(directory.glob("*.svg"))
        for path in paths:
            name = rust_name(style, path.stem)
            if name in seen:
                raise ValueError(f"Conflicting Rust icon enum variant {name}")
            seen.add(name)
            svg = normalize(path.read_text(encoding="utf-8"), style)
            target = ASSET_ROOT / style / path.name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(svg, encoding="utf-8")
            entries.append((style, path.stem, name))
        print(f"{style}: {len(paths)} icons")
    allowed = {ASSET_ROOT / s / (slug + ".svg") for s, slug, _ in entries}
    for stale in ASSET_ROOT.glob("*/*.svg"):
        if stale not in allowed:
            stale.unlink()
    if len(entries) < 800:
        raise ValueError("Incomplete upstream catalog: fewer than 800 icons")
    source = RUST.read_text(encoding="utf-8")
    begin = source.index("/// Every official Ant Design SVG icon")
    end = source.index("/// Render an actual official Ant Design SVG")
    registry = [
        "/// Every official Ant Design SVG icon, generated from a pinned upstream revision.",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]",
        "pub enum AntIconName {",
        *(f"    {name}," for _, _, name in entries),
        "}", "",
        "impl AntIconName {",
        "    pub const ALL: &'static [Self] = &[",
        *(f"        Self::{name}," for _, _, name in entries),
        "    ];", "",
        "    pub const fn path(self) -> &'static str {",
        "        match self {",
        *(f'            Self::{name} => "antd-icons/{style}/{slug}.svg",'
          for style, slug, name in entries),
        "        }", "    }", "",
        "    pub const fn ant_name(self) -> &'static str {",
        "        match self {",
        *(f'            Self::{name} => "{name}",' for _, _, name in entries),
        "        }", "    }", "",
        "    pub const fn style(self) -> &'static str {",
        "        match self {",
    ]
    for style in STYLES:
        names = [name for s, _, name in entries if s == style]
        registry.append("            " + "\n            | ".join(
            f"Self::{name}" for name in names
        ) + f' => "{style}",')
    registry.extend([
        "        }", "    }", "",
        "    /// Resolve the Rust enum by official name, such as SearchOutlined.",
        "    pub fn from_ant_name(name: &str) -> Option<Self> {",
        "        match name {",
        *(f'            "{name}" => Some(Self::{name}),'
          for _, _, name in entries),
        "            _ => None,", "        }", "    }",
        "}", "", "",
    ])
    source = source[:begin] + "\n".join(registry) + source[end:]
    label = "        let svg: Option<&'static [u8]> = match path {"
    start = source.index(label) + len(label)
    finish = source.index("            _ => None,", start)
    cases = "\n" + "\n".join(
        f'            "antd-icons/{style}/{slug}.svg" => '
        f'Some(include_bytes!("../assets/antd-icons/{style}/{slug}.svg").as_slice()),'
        for style, slug, _ in entries
    ) + "\n"
    source = source[:start] + cases + source[finish:]
    source = re.sub(
        r"assert_eq!\(AntIconName::ALL.len\(\), \d+\);",
        f"assert_eq!(AntIconName::ALL.len(), {len(entries)});", source,
    )
    source = source.replace("curated Ant Design SVG", "official Ant Design SVG")
    RUST.write_text(source, encoding="utf-8")
    MANIFEST.parent.mkdir(parents=True, exist_ok=True)
    MANIFEST.write_text(json.dumps({
        "source": "ant-design/ant-design-icons", "revision": PIN,
        "count": len(entries),
        "icons": [
            {"name": name, "style": style, "path": f"antd-icons/{style}/{slug}.svg"}
            for style, slug, name in entries
        ],
    }, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"Generated {len(entries)} official icon assets, enum, and website manifest.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upstream", required=True, type=Path)
    generate(parser.parse_args().upstream.resolve())
