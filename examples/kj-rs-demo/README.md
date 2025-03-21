# Basic Example

This directory contains an example which exercises kj-rs. Both Bazel and Cargo build systems are demonstrated.

## Dependencies

### `kj-rs` and other crates

We vendor all library crates, in particular kj-rs, using rules_rust's crate_universe macros.

There are several ways to use rules_rust to maintain Rust dependencies, vendoring being only one of them. Vendoring is a bit clunky, because it requires a separate repinning step now and then. However, vendoring seems to be the easiest way which still allows overriding the kj-rs crate source code to our local copy in the parent directory. Using `crate.spec()` and `crate.from_specs()` doesn't allow such overriding, and does not seem to generate a BUILD.bazel file for kj-rs somewhere that we can reference in a `new_local_repository()` rule. Meanwhile, `crate.from_cargo()` invokes Cargo, which cannot see up into the parent directory of this Bazel module -- only Bazel can do that.

Periodically, in particular whenever the kj-rs build system, directory structure, or version is modified, we must repin our Rust dependencies in order to regenerate kj-rs's BUILD.bazel file.

```sh
bazel run //deps/rust:crates_vendor -- --repin
```

### `cxxbridge-cmd` dependency

The `cxxbridge-cmd` CLI command is managed in MODULE.bazel, using rules_rust's `crate.from_cargo()` applied to the CLI's source code as an HTTP archive.

This requires a different way to repin:

```sh
CARGO_BAZEL_REPIN=true bazel build //...
```
