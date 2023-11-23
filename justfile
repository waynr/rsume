build:
  cargo build

build-refactor:
  # requires cargo-limit to be installed
  reset
  (cargo lbuild --color=always 2>&1) | less -R

test:
  cargo test

test-refactor:
  # requires cargo-limit to be installed
  reset
  (cargo ltest --color=always 2>&1) | less -R

test-template:
  ./target/debug/resume-generator \
    -t templates/general-purpose.typ \
    testdata/sample.resume.yaml

watchexec target:
  watchexec \
    -c \
    -e toml,rs,typ \
    -w justfile \
    -w src \
    -w templates \
    -w Cargo.toml \
    --restart \
    just {{target}}

we-build-refactor:
  just watchexec build-refactor

we-build:
  just watchexec build

we-test-refactor:
  just watchexec test-refactor

we-test:
  just watchexec test

we-test-template:
  just watchexec test-template
