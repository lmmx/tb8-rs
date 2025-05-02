# Just is a task runner, like Make but without the build system / dependency tracking part.
# docs: https://github.com/casey/just
#
# The `-ci` variants are ran in CI, they do command grouping on GitHub Actions, set consistent env vars etc.,
# but they require bash.
#
# The non`-ci` variants can be run locally without having bash installed.

set dotenv-load

default: precommit prepush

precommit: code-quality
prepush: clippy test
precommit-fix: code-quality-fix

commit-msg message:
  printf "{{ message }}" | conventional_commits_linter --from-stdin --allow-angular-type-only

ci: precommit prepush docs

clippy-all:
    cargo clippy --workspace --all-targets --all-features --target-dir target/clippy-all-features -- -D warnings

clippy:
    cargo clippy --workspace --all-targets --target-dir target/clippy -- -D warnings

test *args:
    cargo nextest run {{args}} < /dev/null

test-ci *args:
    #!/usr/bin/env -S bash -euo pipefail
    source .envrc
    echo -e "\033[1;33m🏃 Running all but doc-tests with nextest...\033[0m"
    cmd_group "cargo nextest run --features slow-tests {{args}} < /dev/null"

    echo -e "\033[1;36m📚 Running documentation tests...\033[0m"
    cmd_group "cargo test --features slow-tests --doc {{args}}"

doc-tests *args:
    cargo test --doc {{args}}

doc-tests-ci *args:
    #!/usr/bin/env -S bash -euo pipefail
    source .envrc
    echo -e "\033[1;36m📚 Running documentation tests...\033[0m"
    cmd_group "cargo test --doc {{args}}"

fix-eof-ws mode="":
    #!/usr/bin/env sh
    ARGS=''
    if [ "{{mode}}" = "check" ]; then
        ARGS="--check-only"
    fi
    whitespace-format --add-new-line-marker-at-end-of-file \
          --new-line-marker=linux \
          --normalize-new-line-markers \
          --exclude ".git/|target/|.json$|.lock$" \
          $ARGS \
          .

code-quality:
    taplo lint
    taplo format --check
    just fix-eof-ws check
    cargo machete
    cargo fmt --check --all

code-quality-fix:
    taplo lint
    taplo format
    just fix-eof-ws
    cargo machete
    cargo fmt --all

docsrs *args:
    #!/usr/bin/env -S bash -eux
    source .envrc
    export RUSTDOCFLAGS="--cfg docsrs"
    cargo +nightly doc {{args}}

docs:
    cargo doc --workspace --all-features --no-deps --document-private-items --keep-going

lockfile:
    cargo update --workspace --locked
