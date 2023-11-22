build:
  cargo build

build-refactor:
  # requires cargo-limit to be installed
  reset
  (cargo lbuild --color=always 2>&1) | less -R

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
