#pragma once

#include <rust/cxx.h>

#include <kj/async.h>

namespace kj_rs {

using OwnPromiseNode = kj::_::OwnPromiseNode;

void own_promise_node_drop_in_place(OwnPromiseNode*);

// https://github.com/dtolnay/cxx/blob/86cd652c06c5cb4c2e24d3ab555cf707b4ae0883/src/cxx.cc#L518
namespace repr {
struct PtrLen final {
  void* ptr;
  std::size_t len;
};
}  // namespace repr

}  // namespace kj_rs

namespace rust {

// OwnPromiseNodes happen to follow Rust move semantics.
template <>
struct IsRelocatable<::kj_rs::OwnPromiseNode>: std::true_type {};

}  // namespace rust
