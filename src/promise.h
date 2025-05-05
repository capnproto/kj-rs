#pragma once

#include <kj/async.h>
#include <kj/debug.h>
#include <rust/cxx.h>

namespace kj_rs {

using OwnPromiseNode = kj::_::OwnPromiseNode;

void own_promise_node_drop_in_place(OwnPromiseNode *);

namespace repr {

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wreturn-type-c-linkage"

// ::cxx::private::PtrLen
struct PtrLen final {
  void *ptr = nullptr;
  std::size_t len = 0;
};

extern "C" {
repr::PtrLen cxxbridge1$exception(const char *, std::size_t len) noexcept;
}

// ::cxx::private::Result
struct Result final {
  PtrLen err = {};

  static Result ok() { return {}; }

  static Result error(kj::Exception &e) {
    auto description = e.getDescription();
    auto err = cxxbridge1$exception(description.cStr(), description.size());
    return {err};
  }
};

using UnwrapCallback = Result (*)(const void /* kj::_::PromiseNode */ *node,
                                  void /* T */ *ret);

#pragma GCC diagnostic pop
} // namespace repr

template <typename T>
static repr::Result unwrapCallback(const void *node, void *ret) noexcept {
  kj::_::ExceptionOr<kj::_::FixVoid<T>> result;
  auto promiseNode = OwnPromiseNode(
      reinterpret_cast<kj::_::PromiseNode *>(const_cast<void *>(node)));
  promiseNode->get(result);

  KJ_IF_SOME(e, kj::runCatchingExceptions(
                    [&promiseNode]() { promiseNode = nullptr; })) {
    return repr::Result::error(e);
  }

  KJ_IF_SOME(e, result.exception) { return repr::Result::error(e); }

  if constexpr (!kj::isSameType<T, void>()) {
    *reinterpret_cast<T *>(ret) = kj::_::convertToReturn(kj::mv(result));
  }
  return repr::Result::ok();
}
struct KjPromiseNodeImpl {
  template <typename T>
  KjPromiseNodeImpl(kj::Promise<T> &&p)
      : node(kj::_::PromiseNode::from(kj::mv(p))
                 .template disown<kj::_::PromiseDisposer>()),
        unwrap(unwrapCallback<T>) {}

  kj::_::PromiseNode *node;
  repr::UnwrapCallback unwrap = nullptr;
};

} // namespace kj_rs

namespace rust {

// OwnPromiseNodes happen to follow Rust move semantics.
template <> struct IsRelocatable<::kj_rs::OwnPromiseNode> : std::true_type {};

} // namespace rust
