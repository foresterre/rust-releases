# list available recipes
default:
    @just --list

# install tools necessary during development
install-development-tools:
    cargo install cargo-msrv

# install tools needed to run publish recipes
install-publish-tools:
    cargo install cargo-release

# determine the Minimum Supported Rust Version
msrv-find:
    cargo msrv find --output-format json -- cargo check -p rust-release --all-features
    cargo msrv find --output-format json -- cargo check -p rust-releases-core --all-features
    cargo msrv find --output-format json -- cargo check -p rust-releases-io --all-features
    cargo msrv find --output-format json -- cargo check -p rust-releases-rust-changelog --all-features
    cargo msrv find --output-format json -- cargo check -p rust-releases-rust-dist --all-features
    cargo msrv find --output-format json -- cargo check -p rust-toolchain --all-features

# verify the Minimum Supported Rust Version
msrv-verify:
    cargo msrv verify --output-format json -- cargo check -p rust-release --all-features
    cargo msrv verify --output-format json -- cargo check -p rust-releases-core --all-features
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

cargo_release_args := "--dependent-version upgrade  --execute --no-tag --no-push"

# publish the rust-releases workspace, excludes rust-release and rust-toolchain which are to be released separately
publish-workspace version:
    just publish-core {{ version }}
    just publish-io {{ version }}
    just publish-rust-changelog {{ version }}
    just publish-rust-dist {{ version }}

# publish 'rust-releases-core'
publish-core version:
    cargo release -p rust-releases-core {{ cargo_release_args }} {{ version }}

# publish 'rust-releases-io'
publish-io version:
    cargo release -p rust-releases-io {{ cargo_release_args }} {{ version }}

# publish 'rust-releases-rust-changelog'
publish-rust-changelog version:
    cargo release -p rust-releases-rust-changelog {{ cargo_release_args }} {{ version }}

# publish 'rust-releases-rust-changelog'
publish-rust-dist version:
    cargo release -p rust-releases-rust-dist {{ cargo_release_args }} {{ version }}
