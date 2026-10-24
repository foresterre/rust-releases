import json
import subprocess
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

def unreleased_packages(selected: list[str]) -> list[dict]:
    metadata = json.loads(
        subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"],
            check=True,
            capture_output=True,
            text=True,
        ).stdout
    )

    packages = []
    for package in metadata["packages"]:
        if selected and package["name"] not in selected:
            print(f"skipping not-selected {package['name']} {package['version']}", flush=True)
            continue

        if package["publish"] == []:
            print(f"skipping not-publishable {package['name']} {package['version']}", flush=True)
            continue

        if published(package["name"], package["version"]):
            print(f"skipping published {package['name']} {package['version']}", flush=True)
            continue

        packages.append(package)

    return packages
