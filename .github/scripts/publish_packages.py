#!/usr/bin/env python3
import subprocess
import sys

from unreleased_packages import unreleased_packages

def main() -> None:
    packages = [package["name"] for package in unreleased_packages([])]

    if not packages:
        print("every workspace package is already published")
        return

    selection = [argument for package in packages for argument in ("-p", package)]
    sys.exit(subprocess.run(["cargo", "publish", *selection, *sys.argv[1:]]).returncode)

if __name__ == "__main__":
    main()
