#pragma once

#include "src/future.h"
#include <kj-rs/awaiter.h>
#include <rust/cxx.h>

#include <kj/async.h>
#include <kj/debug.h>

// These types are shared with rust
namespace kj_rs {

using OwnPromiseNode = kj::_::OwnPromiseNode;

void own_promise_node_drop_in_place(OwnPromiseNode*);

namespace repr {

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wreturn-type-c-linkage"

// ::cxx::private::PtrLen
struct PtrLen final {
  void* ptr = nullptr;
  std::size_t len = 0;
};

extern "C" {
repr::PtrLen cxxbridge1$exception(const char*, std::size_t len) noexcept;
}

// ::cxx::private::Result
struct Result final {
  PtrLen err = {};
  inline static Result ok() {
    return {};
  }
  inline static Result error(kj::Exception& e);
};

// ::kj_rs::promise::UnwrapCallback
using UnwrapCallback = Result (*)(void /* kj::_::PromiseNode */* node, void /* T */* ret);
// ::kj_rs::promise::KjPromiseNodeImpl
struct KjPromiseNodeImpl {
  template <typename T>
  inline KjPromiseNodeImpl(kj::Promise<T>&& p);

  kj::_::PromiseNode* node;
  repr::UnwrapCallback unwrap;
};

using PollCallback = kj_rs::FuturePollStatus (*)(void /* RustFuture::fut */* fut, const void* waker, void /* T */* ret);

// ::kj_rs::promise::RustFuture
struct RustFuture {

  template <typename T>
  operator kj::Promise<T>() {
    struct Impl {
      using ExceptionOrValue = ::kj::_::ExceptionOr<::kj::_::FixVoid<T>>;
      using Output = ::kj::_::FixVoid<T>;

      bool poll(const ::kj_rs::KjWaker& waker, ExceptionOrValue& output) noexcept {
        ::kj_rs::BoxFuturePoller<Output> poller;
        return poller.poll([this, &waker](void* result) {
          // Safety: `*this` is accepted as `Pin<&mut ...>` in the Rust implementation of
          // `box_future_poll()`. This is safe because it effectively implements Unpin, being
          // non-self-referential, so it's fine if we decide to move it later.
          return fut.poll(&fut.repr, &waker, result);
        }, output);
      }

      RustFuture fut;
    };

    return kj::_::PromiseNode::to<kj::Promise<T>>(
        kj::_::allocPromise<FutureAwaiter<Impl>>(Impl{.fut = kj::mv(*this)}));
  }

  ::std::array<std::uintptr_t, 2> repr;
  PollCallback poll;
};

static_assert(sizeof(RustFuture) == 3 * sizeof(std::uintptr_t), "incorrest RustFutureType");

#pragma GCC diagnostic pop
}  // namespace repr

namespace _ {
template <typename T>
repr::Result unwrapCallback(void* nodePtr, void* ret) noexcept {
  auto node = OwnPromiseNode(reinterpret_cast<kj::_::PromiseNode*>(nodePtr));

  kj::_::ExceptionOr<kj::_::FixVoid<T>> result;
  node->get(result);

  KJ_IF_SOME(e, kj::runCatchingExceptions([&node]() { node = nullptr; })) {
    result.addException(kj::mv(e));
  }

  KJ_IF_SOME(e, result.exception) {
    return repr::Result::error(e);
  } else {
    if constexpr (!kj::isSameType<T, void>()) {
      new (reinterpret_cast<T*>(ret)) T(::kj::mv(KJ_ASSERT_NONNULL(result.value)));
    }
    return repr::Result::ok();
  }
}
}  // namespace _

namespace repr {

inline Result Result::error(kj::Exception& e) {
  auto description = e.getDescription();
  // will malloc a copy
  auto err = cxxbridge1$exception(description.cStr(), description.size());
  return {err};
}

template <typename T>
inline KjPromiseNodeImpl::KjPromiseNodeImpl(kj::Promise<T>&& p)
    : node(kj::_::PromiseNode::from(kj::mv(p)).template disown<kj::_::PromiseDisposer>()),
      unwrap(::kj_rs::_::unwrapCallback<T>) {}

}  // namespace repr

}  // namespace kj_rs

namespace rust {

// OwnPromiseNodes happen to follow Rust move semantics.
template <>
struct IsRelocatable<::kj_rs::OwnPromiseNode>: std::true_type {};

}  // namespace rust
