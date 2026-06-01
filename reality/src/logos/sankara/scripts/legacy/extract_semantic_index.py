#!/usr/bin/env python3
"""Extract graph-shaped semantic index from downloaded Sankara HTML pages."""

from __future__ import annotations

import json
import re
from pathlib import Path

ID_PATTERN = re.compile(r'id="([^"]+)"')
CLASS_PATTERN = re.compile(r'class="([^"]+)"')
HREF_PATTERN = re.compile(r'href="([^"]+)"')


def append_jsonl(path: Path, obj: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as f:
        f.write(json.dumps(obj, ensure_ascii=False) + "\n")


def clear_file(path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("", encoding="utf-8")


def infer_kind(node_id: str, classes: list[str]) -> str:
    if "chapter" in classes:
        return "chapter"
    if "aux_bhashya" in classes:
        return "bhashya_segment"
    if "verse" in classes:
        return "verse"
    if node_id.endswith("_W001") or "_W" in node_id:
        return "word"
    if "quote_" in node_id:
        return "quote"
    return "html_node"


def extract_from_html(source_key: str, html: str) -> tuple[list[dict], list[dict]]:
    nodes: list[dict] = []
    edges: list[dict] = []

    lines = html.splitlines()
    for line_no, line in enumerate(lines, start=1):
        ids = ID_PATTERN.findall(line)
        classes = CLASS_PATTERN.findall(line)
        hrefs = HREF_PATTERN.findall(line)

        class_values: list[str] = []
        for cls in classes:
            class_values.extend([c for c in cls.split() if c])

        for node_id in ids:
            node_uid = f"{source_key}:{node_id}"
            nodes.append(
                {
                    "id": node_uid,
                    "source": source_key,
                    "local_id": node_id,
                    "line": line_no,
                    "kind": infer_kind(node_id, class_values),
                    "classes": class_values,
                }
            )

        for href in hrefs:
            edges.append(
                {
                    "source": source_key,
                    "line": line_no,
                    "kind": "links_to",
                    "href": href,
                }
            )

    return nodes, edges


def main() -> int:
    raw_pages = Path("ref/sankara/raw/pages")
    out_nodes = Path("ref/sankara/derived/semantic_nodes.jsonl")
    out_edges = Path("ref/sankara/derived/semantic_edges.jsonl")

    if not raw_pages.exists():
        raise SystemExit(f"No raw pages directory found: {raw_pages}")

    clear_file(out_nodes)
    clear_file(out_edges)

    total_nodes = 0
    total_edges = 0
    files = sorted(raw_pages.glob("*/index.html"))

    for html_path in files:
        source_key = html_path.parent.name
        html = html_path.read_text(encoding="utf-8", errors="replace")
        nodes, edges = extract_from_html(source_key=source_key, html=html)

        for n in nodes:
            append_jsonl(out_nodes, n)
        for e in edges:
            append_jsonl(out_edges, e)

        total_nodes += len(nodes)
        total_edges += len(edges)
        print(f"extracted {source_key}: {len(nodes)} nodes, {len(edges)} edges")

    print(f"done: {len(files)} files, {total_nodes} nodes, {total_edges} edges")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
