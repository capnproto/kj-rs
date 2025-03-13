// Copyright (c) 2024 Cloudflare, Inc.
// Licensed under the Apache 2.0 license found in the LICENSE file or at:
//     https://opensource.org/licenses/Apache-2.0

use cxx_build::CFG;

use std::env;
use std::fs;
use std::path::Path;

const HEADERS: &[&str] = &[
    "awaiter.h",
    "executor-guarded.h",
    "future.h",
    "linked-group.h",
    "promise.h",
    "waker.h",
];

const SOURCES: &[&str] = &[
    "awaiter.c++",
    "executor-guarded.c++",
    "promise.c++",
    "waker.c++",
];

fn main() {
    // We require the builder to explicitly specify the KJ library include and static library paths.
    let libkj_include_path =
        env::var("LIBKJ_INCLUDE_PATH").expect("LIBKJ_INCLUDE_PATH must be set");
    let libkj_link_search_path =
        env::var("LIBKJ_LINK_SEARCH_PATH").expect("LIBKJ_LINK_SEARCH_PATH must be set");

    let libkj_include_path: Vec<&Path> = libkj_include_path.split(':').map(Path::new).collect();
    let libkj_link_search_path: Vec<&Path> =
        libkj_link_search_path.split(':').map(Path::new).collect();

    println!("cargo:rerun-if-env-changed=LIBKJ_INCLUDE_PATH");
    println!("cargo:rerun-if-env-changed=LIBKJ_LINK_SEARCH_PATH");

    for p in libkj_link_search_path {
        println!("cargo:rustc-link-search=native={}", p.display());
    }

    // Awkward: we can only list one library in our "link" value in Cargo.toml, but we need to link
    // with both kj and kj-async.
    println!("cargo:rustc-link-lib=static=kj");
    println!("cargo:rustc-link-lib=static=kj-async");

    for file in HEADERS {
        println!("cargo:rerun-if-changed=include/{}", file);
    }
    for file in SOURCES {
        println!("cargo:rerun-if-changed=src/{}", file);
    }

    // We publicly depend on KJ.
    CFG.exported_header_dirs.extend(libkj_include_path);

    // cxxbridge leaves a symlink at $OUT_DIR/cxxbridge/crate/kj-rs pointing to our
    // CARGO_MANIFEST_DIR. The intent is to provide access to our crate's headers under the prefix
    // `kj-rs/`.`
    // - https://github.com/dtolnay/cxx/issues/754
    // - https://github.com/dtolnay/cxx/issues/1004
    let out_dir = env::var("OUT_DIR").expect("cargo guarantees OUT_DIR is set");
    let out_dir = Path::new(&out_dir);

    // We export our own headers so that they are available with `#include <kj-rs/foo.h>`.
    let out_include_dir = out_dir.join("include");
    CFG.exported_header_dirs.push(&out_include_dir);

    let out_include_kj_rs_dir = out_include_dir.join("kj-rs");
    fs::create_dir_all(&out_include_kj_rs_dir).expect("directory should not yet exist");

    let local_include_dir = Path::new("include");
    for header in HEADERS {
        fs::copy(
            local_include_dir.join(header),
            out_include_kj_rs_dir.join(header),
        )
        .expect("header should not yet exist");
    }

    let local_src_dir = Path::new("src");
    cxx_build::bridge(local_src_dir.join("lib.rs"))
        .cpp(true)
        .files(SOURCES.into_iter().map(|s| local_src_dir.join(s)))
        .cpp_set_stdlib("c++")
        .std("c++23")
        .compile("kj-rs");

    // The symlink-to-directory at $OUT_DIR/cxxbridge/crate/kj-rs confuses rules_rust's
    // recently-added symlink fixup behavior, which ends up trying to copy a directory as a file.
    // - https://github.com/bazelbuild/rules_rust/pull/3067
    //
    // We'll deal with this by just removing the symlink. The only thing dependents need are our
    // headers, which we've already made available at $OUT_DIR/include/kj-rs/.

    let out_cxxbridge_crate_dir = out_dir.join("cxxbridge").join("crate");
    let out_kj_rs_dir = out_cxxbridge_crate_dir.join("kj-rs");
    assert!(out_kj_rs_dir.is_symlink());
    fs::remove_file(&out_kj_rs_dir).expect("cxxbridge/crate/kj-rs should be removable");
}
