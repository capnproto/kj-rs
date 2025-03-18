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
void own_promise_node_unwrap_void(OwnPromiseNode);
void promise_drop_in_place_void(PromiseVoid*);
OwnPromiseNode promise_into_own_promise_node_void(PromiseVoid);

class PromiseI32: public kj::Promise<int32_t> {
public:
  using Promise::Promise;
  PromiseI32(kj::Promise<int32_t>&& promise): kj::Promise<int32_t>(kj::mv(promise)) {}
  using IsRelocatable = std::true_type;
};
int32_t own_promise_node_unwrap_i32(OwnPromiseNode);
void promise_drop_in_place_i32(PromiseI32*);
OwnPromiseNode promise_into_own_promise_node_i32(PromiseI32);

}  // namespace kj_rs_demo
