#pragma once

#include <kj-rs/awaiter.h>
#include <kj-rs/promise.h>

namespace kj_rs_demo {

using kj_rs::OwnPromiseNode;

// TODO(now): Generate boilerplate with a macro.
class PromiseVoid: public kj::Promise<void> {
public:
  using Promise::Promise;
  PromiseVoid(kj::Promise<void>&& promise): Promise(kj::mv(promise)) {}
  using IsRelocatable = std::true_type;
};
extern "C" ::kj_rs::repr::PtrLen own_promise_node_unwrap_void(OwnPromiseNode*, kj::_::FixVoid<void>*) noexcept;
extern "C" void promise_drop_in_place_void(PromiseVoid*) noexcept;
extern "C" void promise_into_own_promise_node_void(PromiseVoid*, OwnPromiseNode*) noexcept;

class PromiseI32: public kj::Promise<int32_t> {
public:
  using Promise::Promise;
  PromiseI32(kj::Promise<int32_t>&& promise): kj::Promise<int32_t>(kj::mv(promise)) {}
  using IsRelocatable = std::true_type;
};
extern "C" ::kj_rs::repr::PtrLen own_promise_node_unwrap_i32(OwnPromiseNode*, kj::_::FixVoid<int32_t>*) noexcept;
extern "C" void promise_drop_in_place_i32(PromiseI32*) noexcept;
extern "C" void promise_into_own_promise_node_i32(PromiseI32*, OwnPromiseNode*) noexcept;

}  // namespace kj_rs_demo
