#!/usr/bin/env python3
import argparse
import json
import sys


def cmd_append(args):
    raw_bytes = sys.stdin.buffer.read()
    try:
        # Handle UTF-8 with/without BOM (common on Windows PowerShell pipes)
        raw = raw_bytes.decode("utf-8-sig").strip()
        if not raw:
            raise UnicodeDecodeError("utf-8-sig", raw_bytes, 0, 1, "empty")
    except UnicodeDecodeError:
        try:
            raw = raw_bytes.decode("utf-16").strip()
        except UnicodeDecodeError:
            raw = raw_bytes.decode("cp1252").strip()

    try:
        payload = json.loads(raw)
    except json.JSONDecodeError as e:
        print(f"Error: invalid JSON — {e}", file=sys.stderr)
        sys.exit(1)

    if not isinstance(payload, dict):
        print("Error: payload must be a JSON object", file=sys.stderr)
        sys.exit(1)

    for field in ("slug", "content"):
        if field not in payload:
            print(f"Error: missing required field '{field}'", file=sys.stderr)
            sys.exit(1)
        if not isinstance(payload[field], str):
            print(f"Error: field '{field}' must be a string", file=sys.stderr)
            sys.exit(1)

    if "nextSteps" in payload:
        if not isinstance(payload["nextSteps"], list) or not all(
            isinstance(s, str) for s in payload["nextSteps"]
        ):
            print("Error: 'nextSteps' must be an array of strings", file=sys.stderr)
            sys.exit(1)

    print(f"slug:    {payload['slug']}")
    print(f"content: {payload['content']}")
    if payload.get("nextSteps"):
        print("nextSteps:")
        for step in payload["nextSteps"]:
            print(f"  - {step}")


def main():
    parser = argparse.ArgumentParser(prog="progress")
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser(
        "append",
        help="Append a progress entry (reads JSON from stdin)",
    )

    args = parser.parse_args()

    if args.command == "append":
        cmd_append(args)


if __name__ == "__main__":
    main()
