# list available recipes
default:
    @just --list

# install tools necessary during development
install-development-tools:
    cargo install cargo-msrv

# determine the Minimum Supported Rust Version
msrv-find:
    cargo msrv find --min 1.85 --output-format json -- cargo check -p rust-release --all-features
    cargo msrv find --min 1.85 --output-format json -- cargo check -p rust-releases-bundled --all-features
    cargo msrv find --min 1.85 --output-format json -- cargo check -p rust-releases-bundled-generator --all-features
    cargo msrv find --min 1.85 --output-format json -- cargo check -p rust-releases-core --all-features
    cargo msrv find --min 1.85 --output-format json -- cargo check -p rust-releases-github --all-features
    cargo msrv find --min 1.85 --output-format json -- cargo check -p rust-releases-io --all-features
    cargo msrv find --min 1.85 --output-format json -- cargo check -p rust-releases-rust-changelog --all-features
    cargo msrv find --min 1.85 --output-format json -- cargo check -p rust-releases-rust-dist --all-features
    cargo msrv find --min 1.85 --output-format json -- cargo check -p rust-toolchain --all-features

# verify the Minimum Supported Rust Version
msrv-verify:
    cargo msrv verify --output-format json -- cargo check -p rust-release --all-features
    cargo msrv verify --output-format json -- cargo check -p rust-releases-bundled --all-features
    cargo msrv verify --output-format json -- cargo check -p rust-releases-bundled-generator --all-features
    cargo msrv verify --output-format json -- cargo check -p rust-releases-core --all-features
    cargo msrv verify --output-format json -- cargo check -p rust-releases-github --all-features
    cargo msrv verify --output-format json -- cargo check -p rust-releases-io --all-features
    cargo msrv verify --output-format json -- cargo check -p rust-releases-rust-changelog --all-features
    cargo msrv verify --output-format json -- cargo check -p rust-releases-rust-dist --all-features
    cargo msrv verify --output-format json -- cargo check -p rust-toolchain --all-features

# run linter on all workspace packages
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# run tests in workspace
test:
    cargo test --all-features --all

# run license and advisory checks
deny:
    cargo deny --all-features check

# regenerate the release data bundled by 'rust-releases-bundled'
bundle-releases:
    cargo run --release -p rust-releases-bundled-generator

# bump the workspace version, and the workspace dependencies
bump version:
    ./.github/scripts/bump-version.py {{ version }}

# bump a separately versioned crate, e.g. `just bump-crate rust-toolchain 3.1.0`
bump-crate package version:
    ./.github/scripts/bump-version.py {{ version }} {{ package }}

cargo_publish_args := "--locked"

# publish every publishable workspace package, in dependency order
publish-workspace:
    cargo publish --workspace {{ cargo_publish_args }}

# publish 'rust-releases-core'
publish-core:
    cargo publish -p rust-releases-core {{ cargo_publish_args }}

# publish 'rust-releases-io'
publish-io:
    cargo publish -p rust-releases-io {{ cargo_publish_args }}

# publish 'rust-releases-github'
publish-github:
    cargo publish -p rust-releases-github {{ cargo_publish_args }}

# publish 'rust-releases-rust-changelog'
publish-rust-changelog:
    cargo publish -p rust-releases-rust-changelog {{ cargo_publish_args }}

# publish 'rust-releases-rust-dist'
publish-rust-dist:
    cargo publish -p rust-releases-rust-dist {{ cargo_publish_args }}

# publish 'rust-releases'
publish-top:
    cargo publish -p rust-releases {{ cargo_publish_args }}

# publish 'rust-releases-bundled'
publish-bundled:
    cargo publish -p rust-releases-bundled {{ cargo_publish_args }}

# publish 'rust-release'
publish-rust-release:
    cargo publish -p rust-release {{ cargo_publish_args }}

# publish 'rust-toolchain'
publish-rust-toolchain:
    cargo publish -p rust-toolchain {{ cargo_publish_args }}
