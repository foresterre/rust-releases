#!/usr/bin/env python3
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
VERSION = re.compile(r'^version = "([^"]+)"$', re.MULTILINE)

def next_version(current: str, requested: str) -> str:
    if requested not in ("major", "minor", "patch"):
        return requested

    major, minor, patch = (int(part) for part in current.split("."))
    if requested == "major":
        return f"{major + 1}.0.0"
    if requested == "minor":
        return f"{major}.{minor + 1}.0"
    return f"{major}.{minor}.{patch + 1}"


def main() -> None:
    requested = sys.argv[1]
    package = sys.argv[2] if len(sys.argv) > 2 else None

    root_manifest = ROOT / "Cargo.toml"
    manifest = ROOT / "crates" / package / "Cargo.toml" if package else root_manifest

    content = manifest.read_text()
    match = VERSION.search(content)
    if match is None:
        sys.exit(f"{manifest} does not set its own version")

    current = match[1]
    version = next_version(current, requested)
    manifest.write_text(VERSION.sub(f'version = "{version}"', content, count=1))

    dependencies = root_manifest.read_text()
    if package:
        dependencies = re.sub(
            rf'^({re.escape(package)} = {{ version = "\^?){re.escape(current)}"',
            rf'\g<1>{version}"',
            dependencies,
            flags=re.MULTILINE,
        )
    else:
        dependencies = dependencies.replace(
            f'version = "^{current}"', f'version = "^{version}"'
        )
    root_manifest.write_text(dependencies)

    print(f"bumped {package or 'the workspace'} from {current} to {version}", file=sys.stderr)
    print(version)

if __name__ == "__main__":
    main()
