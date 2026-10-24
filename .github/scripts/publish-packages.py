#!/usr/bin/env python3
import json
import subprocess
import sys

from crates_index import published

def main() -> None:
    metadata = json.loads(
        subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"],
            check=True,
            capture_output=True,
            text=True,
        ).stdout
    )

    packages = [
        package["name"]
        for package in metadata["packages"]
        if package["publish"] != []
        and not published(package["name"], package["version"])
    ]

    if not packages:
        print("every workspace package is already published")
        return

    selection = [argument for package in packages for argument in ("-p", package)]
    sys.exit(subprocess.run(["cargo", "publish", *selection, *sys.argv[1:]]).returncode)

if __name__ == "__main__":
    main()
