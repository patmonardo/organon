#!/usr/bin/env python3
"""Quality checks for Sankara translation passage records.

This script performs lightweight structural and semantic checks without external
schema dependencies, so it can run in any baseline Python environment.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

RECORD_ID_RE = re.compile(r"^SKR\.[A-Z0-9_]+\.[A-Za-z0-9_.-]+\.[0-9]{3,}$")
SHA_RE = re.compile(r"^[a-f0-9]{64}$")


def load_json(path: Path) -> dict[str, Any]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise ValueError(f"invalid JSON: {exc}") from exc
    if not isinstance(data, dict):
        raise ValueError("root value must be an object")
    return data


def is_passage_record(data: dict[str, Any]) -> bool:
    required = {"record_id", "source", "text", "translation", "analysis", "provenance", "qa"}
    return required.issubset(data.keys())


def require_non_empty_str(data: dict[str, Any], key: str, errors: list[str]) -> str:
    value = data.get(key)
    if not isinstance(value, str) or not value.strip():
        errors.append(f"missing or empty string: {key}")
        return ""
    return value.strip()


def validate_record(data: dict[str, Any], path: Path) -> list[str]:
    errors: list[str] = []

    record_id = require_non_empty_str(data, "record_id", errors)
    if record_id and not RECORD_ID_RE.match(record_id):
        errors.append("record_id does not match SKR.<WORK>.<SECTION>.<INDEX> pattern")

    source = data.get("source")
    if not isinstance(source, dict):
        errors.append("source must be an object")
    else:
        sha = require_non_empty_str(source, "source_sha256", errors)
        if sha and not SHA_RE.match(sha):
            errors.append("source.source_sha256 must be lowercase 64-char hex")
        require_non_empty_str(source, "source_url", errors)

    text = data.get("text")
    if not isinstance(text, dict):
        errors.append("text must be an object")
    else:
        require_non_empty_str(text, "source_text", errors)

    translation = data.get("translation")
    literal = ""
    technical = ""
    if not isinstance(translation, dict):
        errors.append("translation must be an object")
    else:
        literal = require_non_empty_str(translation, "literal_translation", errors)
        technical = require_non_empty_str(translation, "technical_translation", errors)
        if literal and technical and literal == technical:
            errors.append("literal_translation and technical_translation must differ")

    analysis = data.get("analysis")
    if not isinstance(analysis, dict):
        errors.append("analysis must be an object")
    else:
        lexical_notes = analysis.get("lexical_notes")
        if not isinstance(lexical_notes, list):
            errors.append("analysis.lexical_notes must be an array")
        elif not lexical_notes:
            # Enforce at least one lexical note per protocol.
            errors.append("analysis.lexical_notes must contain at least one note")

        argument_tags = analysis.get("argument_tags")
        if not isinstance(argument_tags, list):
            errors.append("analysis.argument_tags must be an array")
        elif not argument_tags:
            errors.append("analysis.argument_tags must contain at least one tag")

    qa = data.get("qa")
    if not isinstance(qa, dict):
        errors.append("qa must be an object")
    else:
        confidence = qa.get("confidence")
        if not isinstance(confidence, (int, float)):
            errors.append("qa.confidence must be numeric")
        elif confidence < 0 or confidence > 1:
            errors.append("qa.confidence must be between 0 and 1")

    if errors:
        prefix = f"{path}:"
        return [f"{prefix} {err}" for err in errors]
    return []


def find_record_files(path: Path) -> list[Path]:
    if path.is_file():
        return [path]
    if not path.exists():
        return []
    return sorted(p for p in path.rglob("*.json") if p.is_file())


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate Sankara translation passage records")
    parser.add_argument(
        "--path",
        type=Path,
        default=Path("ref/sankara/translation/passage_records"),
        help="JSON file or directory to validate",
    )
    parser.add_argument(
        "--strict-all-json",
        action="store_true",
        help="Fail if JSON files are present that are not passage records",
    )
    args = parser.parse_args()

    files = find_record_files(args.path)
    if not files:
        print(f"no JSON records found at: {args.path}")
        return 2

    all_errors: list[str] = []
    validated_count = 0
    skipped_count = 0
    for file_path in files:
        try:
            data = load_json(file_path)
        except ValueError as exc:
            all_errors.append(f"{file_path}: {exc}")
            continue
        if not is_passage_record(data):
            if args.strict_all_json:
                all_errors.append(f"{file_path}: not a passage record JSON object")
            else:
                skipped_count += 1
            continue
        validated_count += 1
        all_errors.extend(validate_record(data, file_path))

    if all_errors:
        print("QA FAILED")
        for err in all_errors:
            print(f"- {err}")
        return 1

    print(f"QA PASSED: validated={validated_count}, skipped={skipped_count}, scanned={len(files)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
