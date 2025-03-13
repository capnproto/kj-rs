# Based on https://github.com/bazelbuild/rules_rust/blob/main/examples/crate_universe/using_cxx/rust_cxx_bridge.bzl

# buildifier: disable=module-docstring
load("@bazel_skylib//rules:run_binary.bzl", "run_binary")
load("@rules_cc//cc:defs.bzl", "cc_library")

def rust_cxx_bridge(name, src, deps = [], include_prefix = None):
    """A macro defining a cxx bridge library

    Args:
        name (string): The name of the new target
        src (string): The rust source file to generate a bridge for
        deps (list, optional): A list of dependencies for the underlying cc_library. Defaults to [].
        include_prefix (string, optional): Path where lib.rs.h is available via the
            :{name}/include` target. Defaults to None.
    """
    native.alias(
        name = "%s/header" % name,
        actual = src + ".h",
    )

    native.alias(
        name = "%s/source" % name,
        actual = src + ".cc",
    )

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
        tool = "@cxxbridge-cmd//:cxxbridge-cmd",
    )

    cc_library(
        name = name,
        srcs = [src + ".cc"],
        linkstatic = True,
        deps = deps + [":%s/include" % name],
    )

    cc_library(
        name = "%s/include" % name,
        hdrs = [src + ".h"],
        include_prefix = include_prefix,
    )
