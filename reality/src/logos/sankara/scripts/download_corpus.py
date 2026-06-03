#!/usr/bin/env python3
"""Download Sankara corpus source pages with optional recursive crawling."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import time
from collections import deque
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterable
from urllib import request
from urllib.error import HTTPError, URLError
from urllib.parse import parse_qsl, urlencode, urldefrag, urljoin, urlparse


NAVIGATION_BLOCK_PATTERNS = (
    re.compile(r"(?is)<nav\b.*?</nav>"),
    re.compile(r'(?is)<header\b.*?</header>'),
    re.compile(r'(?is)<footer\b.*?</footer>'),
    re.compile(r'(?is)<aside\b.*?</aside>'),
)

GITA_FAMILY_PREFIXES = (
    "/display/bhashya/Gita/devanagari",
)

GITA_LISTING_PREFIXES = (
    "/listing/moola/Gita/devanagari",
)

BS_FAMILY_PREFIXES = (
    "/display/bhashya/BS/devanagari",
)

CORPUS_INDEX_PATHS = (
    "/%E0%A4%97%E0%A5%8D%E0%A4%B0%E0%A4%A8%E0%A5%8D%E0%A4%A5%E0%A4%BE%E0%A4%83/",
    "/ग्रन्थाः/",
)

CORPUS_CONTENT_PREFIXES = (
    "/display/",
    "/listing/",
)

BHASHYA_DISPLAY_PATH_RE = re.compile(r"^/display/bhashya/[^/]+/devanagari$")


SCRIPT_DIR = Path(__file__).resolve().parent
SANKARA_DIR = SCRIPT_DIR.parent


def resolve_default_path(value: str | Path, *base_dirs: Path) -> Path:
    path = Path(value)
    if path.is_absolute():
        return path
    for base_dir in base_dirs:
        candidate = (base_dir / path).resolve()
        if candidate.exists() or candidate.parent.exists():
            return candidate
    return (Path.cwd() / path).resolve()


def read_url_file(path: Path) -> list[str]:
    urls: list[str] = []
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        urls.append(line)
    return urls


def safe_slug(url: str) -> str:
    parsed = urlparse(url)
    parts = [parsed.netloc, *[piece for piece in parsed.path.split("/") if piece]]

    query_pairs = parse_qsl(parsed.query, keep_blank_values=True)
    query_map: dict[str, str] = {}
    for key, value in query_pairs:
        if key not in query_map:
            query_map[key] = value

    # Canonical naming policy:
    # - For bhashya display pages, prefer semantic id-based names.
    # - Include page only when id is absent.
    if parsed.path.startswith("/display/bhashya/"):
        id_value = query_map.get("id", "").strip()
        if id_value:
            parts.append(f"id-{id_value}")
        else:
            page_value = query_map.get("page", "").strip()
            if page_value:
                parts.append(f"page-{page_value}")
        for key, value in query_pairs:
            if key in {"id", "page"}:
                continue
            parts.append(f"{key}-{value}")
    else:
        if query_pairs:
            parts.extend(f"{key}-{value}" for key, value in query_pairs)

    if parsed.fragment:
        parts.append(parsed.fragment)
    base = "_".join(parts)
    base = re.sub(r"[^a-zA-Z0-9._-]+", "_", base).strip("_")
    return base[:160] if base else "url"


def fetch(url: str, timeout_sec: int, user_agent: str) -> tuple[bytes, str, str]:
    req = request.Request(url, headers={"User-Agent": user_agent})
    with request.urlopen(req, timeout=timeout_sec) as resp:
        body = resp.read()
        content_type = resp.headers.get_content_type()
        final_url = resp.geturl()
        return body, content_type, final_url


def normalize_url(url: str) -> str:
    clean, _frag = urldefrag(url.strip())
    return clean


def normalize_seed_url(url: str) -> str:
    parsed = urlparse(url.strip())
    fragment = parsed.fragment.strip()
    if not fragment:
        return normalize_url(url)

    # Some texts encode chapter selection as in-page anchors (e.g. #Ka_C02),
    # but the server payload is actually page-parametrized. Promote those
    # fragments to query parameters so each seeded chapter becomes fetchable.
    if BHASHYA_DISPLAY_PATH_RE.match(parsed.path):
        match = re.match(r"[A-Za-z]+_C(\d{2})(?:_.+)?$", fragment)
        if match:
            page_number = str(int(match.group(1)))
            pairs = [
                (k, v)
                for k, v in parse_qsl(parsed.query, keep_blank_values=True)
                if k not in {"id", "page"}
            ]
            pairs.extend([("page", page_number), ("id", fragment)])
            promoted_query = urlencode(pairs, doseq=True)
            return parsed._replace(query=promoted_query, fragment="").geturl()

    return parsed._replace(fragment="").geturl()


def normalize_query(url: str, allowed_query_keys: set[str]) -> str:
    parsed = urlparse(url)
    if not parsed.query:
        return url

    pairs = parse_qsl(parsed.query, keep_blank_values=True)
    if allowed_query_keys:
        pairs = [(k, v) for k, v in pairs if k in allowed_query_keys]

    # Deterministic ordering prevents duplicate crawl keys caused by param order.
    pairs.sort(key=lambda kv: (kv[0], kv[1]))
    normalized_query = urlencode(pairs, doseq=True)
    return parsed._replace(query=normalized_query).geturl()


def canonicalize_gita_chapter_url(url: str) -> str:
    parsed = urlparse(url)
    pairs = parse_qsl(parsed.query, keep_blank_values=True)
    if parsed.path == "/display/bhashya/Gita/devanagari" and not pairs:
        return parsed._replace(query=urlencode([("page", "1")], doseq=True)).geturl()
    if not pairs:
        return url

    values_by_key: dict[str, list[str]] = {}
    for key, value in pairs:
        values_by_key.setdefault(key, []).append(value)

    page_values = values_by_key.get("page")
    if parsed.path == "/display/bhashya/Gita/devanagari" and page_values:
        # Gita display pages are chapter-scale; distinct verse ids on the same page
        # produce duplicate chapter downloads, so canonicalize to page only.
        normalized_query = urlencode([("page", page_values[0])], doseq=True)
        return parsed._replace(query=normalized_query).geturl()

    if parsed.path.startswith("/display/bhashya/Gita/") and parsed.path != "/display/bhashya/Gita/devanagari":
        id_values = values_by_key.get("id")
        if not id_values:
            return url
        match = re.match(r"BG_C(\d{2})_", id_values[0])
        if not match:
            return url
        chapter_number = str(int(match.group(1)))
        normalized_query = urlencode([("page", chapter_number)], doseq=True)
        return parsed._replace(path="/display/bhashya/Gita/devanagari", query=normalized_query).geturl()

    return url


def canonicalize_bs_chapter_url(url: str) -> str:
    parsed = urlparse(url)
    pairs = parse_qsl(parsed.query, keep_blank_values=True)
    if parsed.path == "/display/bhashya/BS/devanagari" and not pairs:
        return parsed._replace(query=urlencode([("page", "1")], doseq=True)).geturl()
    if not pairs:
        return url

    values_by_key: dict[str, list[str]] = {}
    for key, value in pairs:
        values_by_key.setdefault(key, []).append(value)

    page_values = values_by_key.get("page")
    if parsed.path == "/display/bhashya/BS/devanagari" and page_values:
        normalized_query = urlencode([("page", page_values[0])], doseq=True)
        return parsed._replace(query=normalized_query).geturl()

    if parsed.path == "/display/bhashya/BS/BS":
        id_values = values_by_key.get("id")
        if not id_values:
            return url
        match = re.match(r"BS_C(\d{2})_", id_values[0])
        if not match:
            return url
        chapter_number = str(int(match.group(1)))
        normalized_query = urlencode([("page", chapter_number)], doseq=True)
        return parsed._replace(path="/display/bhashya/BS/devanagari", query=normalized_query).geturl()

    return url


def canonicalize_recursive_url(url: str) -> str:
    return canonicalize_bs_chapter_url(canonicalize_gita_chapter_url(url))


def strip_navigation_blocks(html_text: str) -> str:
    stripped = html_text
    for pattern in NAVIGATION_BLOCK_PATTERNS:
        stripped = pattern.sub("", stripped)
    return stripped


def extract_bs_chapter_page_links(base_url: str, html_text: str) -> list[str]:
    parsed = urlparse(base_url)
    if parsed.path != "/display/bhashya/BS/devanagari":
        return []

    chapter_urls: list[str] = []
    seen_urls: set[str] = set()
    for match in re.finditer(r'href\s*=\s*"#BS_C(\d{2})(?:[^\"]*)"|href\s*=\s*\'#BS_C(\d{2})(?:[^\']*)\'', html_text):
        chapter_text = match.group(1) or match.group(2)
        if not chapter_text:
            continue
        page_number = str(int(chapter_text))
        chapter_url = parsed._replace(query=urlencode([("page", page_number)], doseq=True), fragment="").geturl()
        if chapter_url in seen_urls:
            continue
        seen_urls.add(chapter_url)
        chapter_urls.append(chapter_url)
    return chapter_urls


def extract_gita_chapter_page_links(base_url: str, html_text: str) -> list[str]:
    parsed = urlparse(base_url)
    if parsed.path != "/display/bhashya/Gita/devanagari":
        return []

    chapter_urls: list[str] = []
    seen_urls: set[str] = set()
    for match in re.finditer(r'href\s*=\s*"#BG_C(\d{2})(?:[^\"]*)"|href\s*=\s*\'#BG_C(\d{2})(?:[^\']*)\'', html_text):
        chapter_text = match.group(1) or match.group(2)
        if not chapter_text:
            continue
        page_number = str(int(chapter_text))
        chapter_url = parsed._replace(query=urlencode([("page", page_number)], doseq=True), fragment="").geturl()
        if chapter_url in seen_urls:
            continue
        seen_urls.add(chapter_url)
        chapter_urls.append(chapter_url)
    return chapter_urls


def extract_links(base_url: str, html_text: str) -> list[str]:
    href_pattern = re.compile(r'href\s*=\s*"([^"]+)"|href\s*=\s*\'([^\']+)\'')
    links: list[str] = []
    for m in href_pattern.finditer(html_text):
        href = m.group(1) or m.group(2) or ""
        href = href.strip()
        if not href:
            continue
        if href.startswith(("#", "mailto:", "javascript:", "tel:")):
            continue
        absolute = normalize_url(urljoin(base_url, href))
        if absolute:
            links.append(absolute)
    links.extend(extract_bs_chapter_page_links(base_url, html_text))
    links.extend(extract_gita_chapter_page_links(base_url, html_text))
    return links


def seed_path_prefixes(seed_urls: Iterable[str]) -> list[str]:
    prefixes: list[str] = []
    seen_prefixes: set[str] = set()
    has_gita_display_seed = False
    has_gita_listing_seed = False
    has_bs_seed = False
    has_corpus_index_seed = False
    for seed in seed_urls:
        parsed = urlparse(seed)
        path = parsed.path or "/"
        if path == "/display/bhashya/Gita/devanagari":
            has_gita_display_seed = True
        if path == "/listing/moola/Gita/devanagari":
            has_gita_listing_seed = True
        if path == "/display/bhashya/BS/devanagari":
            has_bs_seed = True
        if path in CORPUS_INDEX_PATHS:
            has_corpus_index_seed = True
        if path == "/listing/moola/Gita/devanagari" or BHASHYA_DISPLAY_PATH_RE.match(path):
            pass
        elif not path.endswith("/"):
            path = path.rsplit("/", 1)[0] + "/"
        if path and path not in seen_prefixes:
            prefixes.append(path)
            seen_prefixes.add(path)
    if has_gita_display_seed:
        for prefix in GITA_FAMILY_PREFIXES:
            if prefix not in seen_prefixes:
                prefixes.append(prefix)
                seen_prefixes.add(prefix)
    if has_gita_listing_seed:
        for prefix in GITA_LISTING_PREFIXES:
            if prefix not in seen_prefixes:
                prefixes.append(prefix)
                seen_prefixes.add(prefix)
    if has_bs_seed:
        for prefix in BS_FAMILY_PREFIXES:
            if prefix not in seen_prefixes:
                prefixes.append(prefix)
                seen_prefixes.add(prefix)
    if has_corpus_index_seed:
        for prefix in CORPUS_CONTENT_PREFIXES:
            if prefix not in seen_prefixes:
                prefixes.append(prefix)
                seen_prefixes.add(prefix)
    return prefixes


def should_visit(
    url: str,
    allowed_hosts: set[str],
    allowed_path_prefixes: list[str],
    deny_pattern: re.Pattern[str],
) -> bool:
    parsed = urlparse(url)
    if parsed.scheme not in {"http", "https"}:
        return False
    if parsed.netloc not in allowed_hosts:
        return False
    path = parsed.path or "/"
    lowered = path.lower()
    if deny_pattern.search(lowered):
        return False
    if allowed_path_prefixes:
        return any(path.startswith(prefix) for prefix in allowed_path_prefixes)
    return True


def load_previously_seen_urls(manifest_path: Path) -> set[str]:
    if not manifest_path.exists():
        return set()
    seen: set[str] = set()
    for line in manifest_path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            obj = json.loads(line)
        except json.JSONDecodeError:
            continue
        url = obj.get("url")
        if isinstance(url, str) and url:
            seen.add(url)
    return seen


def append_jsonl(path: Path, obj: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("a", encoding="utf-8") as f:
        f.write(json.dumps(obj, ensure_ascii=False) + "\n")


def build_seed_fragment_targets(
    seed_urls: Iterable[str],
    allowed_query_keys: set[str],
) -> dict[str, set[str]]:
    targets: dict[str, set[str]] = {}
    for seed in seed_urls:
        parsed = urlparse(seed)
        fragment = parsed.fragment.strip()
        if not fragment:
            continue
        normalized_seed = normalize_seed_url(seed)
        normalized = canonicalize_recursive_url(
            normalize_query(normalized_seed, allowed_query_keys)
        )
        if not normalized:
            continue
        targets.setdefault(normalized, set()).add(fragment)
    return targets


def download_urls(
    urls: Iterable[str],
    out_dir: Path,
    timeout_sec: int,
    sleep_ms: int,
    user_agent: str,
) -> None:
    pages_dir = out_dir / "pages"
    manifest = out_dir / "manifest.jsonl"
    pages_dir.mkdir(parents=True, exist_ok=True)

    prepared_urls: list[tuple[str, str]] = []
    seen_prepared: set[str] = set()
    for raw_url in urls:
        normalized_url = normalize_seed_url(raw_url)
        if normalized_url in seen_prepared:
            continue
        seen_prepared.add(normalized_url)
        prepared_urls.append((raw_url, normalized_url))

    for i, (seed_url, url) in enumerate(prepared_urls, start=1):
        ts = datetime.now(timezone.utc).isoformat()
        slug = safe_slug(url)
        page_dir = pages_dir / slug
        page_dir.mkdir(parents=True, exist_ok=True)
        html_path = page_dir / "index.html"

        record = {
            "timestamp_utc": ts,
            "url": url,
            "seed_url": seed_url,
            "slug": slug,
            "html_path": str(html_path.resolve()),
            "status": "error",
            "mode": "seed_only",
        }

        try:
            body, content_type, final_url = fetch(url, timeout_sec=timeout_sec, user_agent=user_agent)
            html_path.write_bytes(body)
            sha = hashlib.sha256(body).hexdigest()
            record.update(
                {
                    "status": "ok",
                    "sha256": sha,
                    "bytes": len(body),
                    "content_type": content_type,
                    "final_url": final_url,
                }
            )
            print(f"[{i}] ok  {url} -> {html_path}")
        except HTTPError as e:
            record.update({"status": "http_error", "code": e.code, "reason": str(e.reason)})
            print(f"[{i}] HTTP {e.code} {url}")
        except URLError as e:
            record.update({"status": "url_error", "reason": str(e.reason)})
            print(f"[{i}] URL error {url}: {e.reason}")
        except Exception as e:  # pragma: no cover (defensive path)
            record.update({"status": "exception", "reason": repr(e)})
            print(f"[{i}] exception {url}: {e}")

        append_jsonl(manifest, record)

        if sleep_ms > 0:
            time.sleep(sleep_ms / 1000.0)


def crawl_urls(
    seed_urls: Iterable[str],
    out_dir: Path,
    timeout_sec: int,
    sleep_ms: int,
    user_agent: str,
    max_pages: int,
    max_depth: int,
    allowed_hosts: set[str],
    allowed_path_prefixes: list[str],
    deny_pattern: re.Pattern[str],
    ignore_manifest: bool,
    max_links_per_page: int,
    max_queue_size: int,
    allowed_query_keys: set[str],
) -> None:
    pages_dir = out_dir / "pages"
    manifest = out_dir / "manifest.jsonl"
    pages_dir.mkdir(parents=True, exist_ok=True)

    seen: set[str] = set() if ignore_manifest else load_previously_seen_urls(manifest)
    queue: deque[tuple[str, int, str | None]] = deque()
    seed_fragment_targets = build_seed_fragment_targets(seed_urls, allowed_query_keys)

    for seed in seed_urls:
        seed = canonicalize_recursive_url(
            normalize_query(normalize_seed_url(seed), allowed_query_keys)
        )
        if not seed:
            continue
        queue.append((seed, 0, None))

    fetched_count = 0
    discovered_count = 0
    seed_prefixes = seed_path_prefixes(seed_urls)
    effective_path_prefixes = allowed_path_prefixes or seed_prefixes

    while queue and fetched_count < max_pages:
        url, depth, parent_url = queue.popleft()
        if url in seen:
            continue
        if depth > max_depth:
            continue
        if not should_visit(url, allowed_hosts, effective_path_prefixes, deny_pattern):
            continue

        seen.add(url)
        fetched_count += 1

        ts = datetime.now(timezone.utc).isoformat()
        slug = safe_slug(url)
        page_dir = pages_dir / slug
        page_dir.mkdir(parents=True, exist_ok=True)
        html_path = page_dir / "index.html"

        record = {
            "timestamp_utc": ts,
            "mode": "recursive",
            "url": url,
            "slug": slug,
            "html_path": str(html_path.resolve()),
            "status": "error",
            "depth": depth,
            "parent_url": parent_url,
            "discovered_links": 0,
            "queued_links": 0,
        }

        if url in seed_fragment_targets:
            record["seed_fragment_targets"] = sorted(seed_fragment_targets[url])

        try:
            body, content_type, final_url = fetch(url, timeout_sec=timeout_sec, user_agent=user_agent)
            html_path.write_bytes(body)

            sha = hashlib.sha256(body).hexdigest()
            record.update(
                {
                    "status": "ok",
                    "sha256": sha,
                    "bytes": len(body),
                    "content_type": content_type,
                    "final_url": final_url,
                }
            )

            queued_links = 0
            if content_type == "text/html":
                html_text = body.decode("utf-8", errors="replace")

                if url in seed_fragment_targets:
                    targets = sorted(seed_fragment_targets[url])
                    found_ids = sorted(
                        fragment
                        for fragment in targets
                        if re.search(rf'id=["\']{re.escape(fragment)}["\']', html_text)
                        or re.search(rf'name=["\']{re.escape(fragment)}["\']', html_text)
                    )
                    found_links = sorted(
                        fragment
                        for fragment in targets
                        if re.search(rf'href\s*=\s*["\']#{re.escape(fragment)}["\']', html_text)
                    )
                    record["seed_fragment_ids_found"] = found_ids
                    record["seed_fragment_links_found"] = found_links
                    record["seed_fragment_missing_ids"] = [
                        fragment for fragment in targets if fragment not in found_ids
                    ]

                links = extract_links(final_url or url, strip_navigation_blocks(html_text))
                record["discovered_links"] = len(links)
                discovered_count += len(links)

                for link in links:
                    link = canonicalize_recursive_url(normalize_query(link, allowed_query_keys))
                    if link in seen:
                        continue
                    if not should_visit(link, allowed_hosts, effective_path_prefixes, deny_pattern):
                        continue
                    if max_queue_size > 0 and len(queue) >= max_queue_size:
                        break
                    queue.append((link, depth + 1, url))
                    queued_links += 1
                    if max_links_per_page > 0 and queued_links >= max_links_per_page:
                        break

            record["queued_links"] = queued_links
            print(
                f"[{fetched_count}] ok depth={depth} queued={queued_links} "
                f"{url} -> {html_path}"
            )

        except HTTPError as e:
            record.update({"status": "http_error", "code": e.code, "reason": str(e.reason)})
            print(f"[{fetched_count}] HTTP {e.code} depth={depth} {url}")
        except URLError as e:
            record.update({"status": "url_error", "reason": str(e.reason)})
            print(f"[{fetched_count}] URL error depth={depth} {url}: {e.reason}")
        except Exception as e:  # pragma: no cover (defensive path)
            record.update({"status": "exception", "reason": repr(e)})
            print(f"[{fetched_count}] exception depth={depth} {url}: {e}")

        append_jsonl(manifest, record)

        if sleep_ms > 0:
            time.sleep(sleep_ms / 1000.0)

    print(
        f"crawl complete: fetched={fetched_count}, "
        f"discovered_links={discovered_count}, queue_remaining={len(queue)}"
    )


def main() -> int:
    parser = argparse.ArgumentParser(description="Download Sankara source pages with provenance manifest")
    parser.add_argument(
        "--url-file",
        type=Path,
        default=resolve_default_path("sources/seed_urls.txt", SANKARA_DIR, SCRIPT_DIR),
        help="Path to newline-delimited URL file",
    )
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=resolve_default_path("raw", SANKARA_DIR, SCRIPT_DIR),
        help="Output directory for raw pages and manifest",
    )
    parser.add_argument("--timeout-sec", type=int, default=30)
    parser.add_argument("--sleep-ms", type=int, default=400)
    parser.add_argument(
        "--recursive",
        action="store_true",
        help="Recursively crawl links from seeds (domain/path filtered)",
    )
    parser.add_argument(
        "--max-pages",
        type=int,
        default=500,
        help="Maximum pages to fetch in recursive mode",
    )
    parser.add_argument(
        "--max-depth",
        type=int,
        default=6,
        help="Maximum crawl depth from seeds in recursive mode",
    )
    parser.add_argument(
        "--allow-host",
        action="append",
        default=[],
        help="Allowed host for recursive crawl (repeatable). Defaults to seed hosts.",
    )
    parser.add_argument(
        "--allow-path-prefix",
        action="append",
        default=[],
        help="Allowed path prefix for recursive crawl (repeatable).",
    )
    parser.add_argument(
        "--deny-path-regex",
        default=r"\.(css|js|png|jpe?g|gif|svg|ico|woff2?|ttf|eot|pdf|zip|mp3|mp4|webm|ogg|xml)$",
        help="Regex to reject URL paths during crawl",
    )
    parser.add_argument(
        "--ignore-manifest",
        action="store_true",
        help="Ignore previously recorded URLs and fetch from scratch",
    )
    parser.add_argument(
        "--max-links-per-page",
        type=int,
        default=250,
        help="Maximum discovered links to enqueue from each HTML page (0 disables cap)",
    )
    parser.add_argument(
        "--max-queue-size",
        type=int,
        default=50000,
        help="Maximum pending URL queue length during recursive crawl (0 disables cap)",
    )
    parser.add_argument(
        "--allow-query-key",
        action="append",
        default=[],
        help="Allowed query parameter keys in recursive URLs (repeatable). If empty, keep all.",
    )
    parser.add_argument(
        "--user-agent",
        default="organon-sankara-dev/0.1 (+https://github.com/)",
    )
    args = parser.parse_args()

    if not args.url_file.exists():
        raise SystemExit(f"URL file not found: {args.url_file}")

    urls = read_url_file(args.url_file)
    if not urls:
        raise SystemExit("No URLs found in url-file")

    if args.recursive:
        seed_hosts = {urlparse(u).netloc for u in urls if urlparse(u).netloc}
        allowed_hosts = set(args.allow_host) if args.allow_host else seed_hosts
        if not allowed_hosts:
            raise SystemExit("No allowed hosts resolved for recursive crawl")

        deny_pattern = re.compile(args.deny_path_regex, flags=re.IGNORECASE)
        crawl_urls(
            seed_urls=urls,
            out_dir=args.out_dir,
            timeout_sec=args.timeout_sec,
            sleep_ms=args.sleep_ms,
            user_agent=args.user_agent,
            max_pages=args.max_pages,
            max_depth=args.max_depth,
            allowed_hosts=allowed_hosts,
            allowed_path_prefixes=args.allow_path_prefix,
            deny_pattern=deny_pattern,
            ignore_manifest=args.ignore_manifest,
            max_links_per_page=args.max_links_per_page,
            max_queue_size=args.max_queue_size,
            allowed_query_keys=set(args.allow_query_key),
        )
    else:
        download_urls(
            urls=urls,
            out_dir=args.out_dir,
            timeout_sec=args.timeout_sec,
            sleep_ms=args.sleep_ms,
            user_agent=args.user_agent,
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
