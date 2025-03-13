"""
The `cc_headers` rule exposes all of the headers from targets in its `srcs` attribute as regular
files. Notably, this allows Cargo build scripts access to the headers when the `cc_headers` targets
are supplied as data dependencies to rules_rust's `cargo_build_script`.

Source: https://github.com/bazelbuild/bazel/issues/10300#issuecomment-558959917
"""

def _cc_headers_impl(ctx):
    headers = []
    for src in ctx.attr.srcs:
        headers += [
            h
            for h in src[CcInfo].compilation_context.headers.to_list()
            if src.label.package in h.path and
               h.extension in ["h", "hpp"]
        ]
    return [DefaultInfo(files = depset(headers))]

cc_headers = rule(
    attrs = {
        "srcs": attr.label_list(providers = [CcInfo]),
    },
    implementation = _cc_headers_impl,
)
