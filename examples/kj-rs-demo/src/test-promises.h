#pragma once

#include <kj-rs-demo/promise-boilerplate.h>

namespace kj_rs_demo {

PromiseVoid new_ready_promise_void();
PromiseVoid new_pending_promise_void();
PromiseVoid new_coroutine_promise_void();

PromiseVoid new_errored_promise_void();
PromiseI32 new_ready_promise_i32(int32_t);

}  // namespace kj_rs_demo
