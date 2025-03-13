#pragma once

#include <kj-rs/awaiter.h>
#include <kj-rs/promise.h>

namespace kj_rs::tests {

using kj_rs::OwnPromiseNode;

// TODO(now): Generate boilerplate with a macro.
using PromiseVoid = kj::Promise<void>;
using PtrPromiseVoid = PromiseVoid*;
void own_promise_node_unwrap_void(OwnPromiseNode);
void promise_drop_in_place_void(PtrPromiseVoid);
OwnPromiseNode promise_into_own_promise_node_void(PromiseVoid);

using PromiseI32 = kj::Promise<int32_t>;
using PtrPromiseI32 = PromiseI32*;
int32_t own_promise_node_unwrap_i32(OwnPromiseNode);
void promise_drop_in_place_i32(PtrPromiseI32);
OwnPromiseNode promise_into_own_promise_node_i32(PromiseI32);

}  // namespace kj_rs::tests
