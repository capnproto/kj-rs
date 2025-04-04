# Based on https://github.com/bazelbuild/rules_rust/blob/main/examples/crate_universe/using_cxx/rust_cxx_bridge.bzl

# buildifier: disable=module-docstring
load("@bazel_skylib//rules:run_binary.bzl", "run_binary")
load("@rules_cc//cc:defs.bzl", "cc_library")

def rust_cxx_bridge(name, src, hdrs, deps = [], include_prefix = None, strip_include_prefix = None):
    """A macro defining a cxx bridge library

    Args:
        name (string): The name of the new target
        src (string): The rust source file to generate a bridge for
        deps (list, optional): A list of dependencies for the underlying cc_library. Defaults to [].
        include_prefix (string, optional): Path where lib.rs.h is available via the
            :{name}/include` target. Defaults to None.
    """
    run_binary(
        name = "%s/generated" % name,
        srcs = [src],
        outs = [
            src + ".h",
            src + ".cc",
        ],
        args = [
            "$(location %s)" % src,
            "-o",
            "$(location %s.h)" % src,
            "-o",
            "$(location %s.cc)" % src,
        ],
        tool = "@crates_vendor//:cxxbridge-cmd__cxxbridge",
    )

    cc_library(
        name = name,
        srcs = [src + ".cc"],
        hdrs = [src + ".h"] + hdrs,
        # linkstatic = True,
        include_prefix = include_prefix,
        strip_include_prefix = strip_include_prefix,
        deps = deps,
    )
