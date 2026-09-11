#!/usr/bin/env python3
import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

UNRELEASED = re.compile(r"^## Unreleased$", re.MULTILINE)

def main() -> None:
    selected = sys.argv[1:]
    release_date = datetime.now(timezone.utc).strftime("%Y-%m-%d")
    metadata = json.loads(
        subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"],
            check=True,
            capture_output=True,
            text=True,
        ).stdout
    )

    for package in metadata["packages"]:
        if package["publish"] == [] or (selected and package["name"] not in selected):
            continue

        changelog = Path(package["manifest_path"]).with_name("CHANGELOG.md")
        header = f"## {package['version']} - {release_date}"
        content = UNRELEASED.sub(
            lambda match: f"{match[0]}\n\n{header}", changelog.read_text(), count=1
        )
        changelog.write_text(content)

if __name__ == "__main__":
    main()
