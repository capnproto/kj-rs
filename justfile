alias w := watch
alias b := build
alias t := test

watch +WATCH_TARGET='test':
    watchexec -rc -w tests -w src -w gen -w macro -- just {{WATCH_TARGET}}

build:
    bazel build //...

test:
    bazel test //...

cargo-update:
    bazel run //third-party:vendor
    
# called by rust-analyzer discoverConfig (quiet recipe with no output)
@_rust-analyzer:
  rm -rf ./rust-project.json
  # rust-analyzer doesn't like stderr output, redirect it to /dev/null
  bazel run @rules_rust//tools/rust_analyzer:discover_bazel_rust_project 2>/dev/null
