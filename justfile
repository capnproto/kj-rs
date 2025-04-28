cargo-update:
    bazel run //deps/rust:crates.io -- --repin
    
# called by rust-analyzer discoverConfig (quiet recipe with no output)
@_rust-analyzer:
  rm -rf ./rust-project.json
  # rust-analyzer doesn't like stderr output, redirect it to /dev/null
  bazel run @rules_rust//tools/rust_analyzer:discover_bazel_rust_project 2>/dev/null
