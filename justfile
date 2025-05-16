alias w := watch
alias b := build
alias t := test

watch +WATCH_TARGET='test':
    watchexec -rc -w . -- just {{WATCH_TARGET}}

build:
    bazel build //...

test:
    bazel test //...

format: rustfmt clang-format

clang-format:
    clang-format -i src/*.h src/*.c++ examples/kj-rs-demo/src/*.h examples/kj-rs-demo/src/*.c++

rustfmt:
  bazel run @rules_rust//:rustfmt

cargo-update:
    bazel run //deps/rust:crates.io -- --repin

compile-commands:
    bazel run @hedron_compile_commands//:refresh_all
    
# called by rust-analyzer discoverConfig (quiet recipe with no output)
@_rust-analyzer:
  rm -rf ./rust-project.json
  # rust-analyzer doesn't like stderr output, redirect it to /dev/null
  bazel run @rules_rust//tools/rust_analyzer:discover_bazel_rust_project 2>/dev/null
