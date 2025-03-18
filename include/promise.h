#pragma once

#include <kj/async.h>
#include <rust/cxx.h>

namespace kj_rs {

using OwnPromiseNode = kj::_::OwnPromiseNode;

void own_promise_node_drop_in_place(OwnPromiseNode*);

}  // namespace kj_rs

namespace rust {

// OwnPromiseNodes happen to follow Rust move semantics.
template <>
struct IsRelocatable<::kj_rs::OwnPromiseNode>: std::true_type {};

}  // namespace rust
