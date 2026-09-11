#!/usr/bin/env python3
import json
import subprocess
import sys
import urllib.error
import urllib.request

INDEX = "https://index.crates.io"

def index_url(name: str) -> str:
    if len(name) <= 2:
        return f"{INDEX}/{len(name)}/{name}"
    if len(name) == 3:
        return f"{INDEX}/3/{name[0]}/{name}"
    return f"{INDEX}/{name[:2]}/{name[2:4]}/{name}"

def published(name: str, version: str) -> bool:
    try:
        with urllib.request.urlopen(index_url(name)) as response:
            entries = response.read().decode().splitlines()
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return False
        raise

    return any(json.loads(entry)["vers"] == version for entry in entries)

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
