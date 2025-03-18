#include <kj-rs-demo/test-promises.h>

#include <kj/debug.h>

namespace kj_rs_demo {

PromiseVoid new_ready_promise_void() {
  return kj::Promise<void>(kj::READY_NOW);
}

PromiseI32 new_ready_promise_i32(int32_t value) {
  return kj::Promise<int32_t>(value);
}

PromiseVoid new_pending_promise_void() {
  return kj::Promise<void>(kj::NEVER_DONE);
}

PromiseVoid new_coroutine_promise_void() {
  return []() -> kj::Promise<void> {
    co_await kj::Promise<void>(kj::READY_NOW);
    co_await kj::Promise<void>(kj::READY_NOW);
    co_await kj::Promise<void>(kj::READY_NOW);
  }();
}

PromiseVoid new_errored_promise_void() {
  return KJ_EXCEPTION(FAILED, "test error");
}

}  // namespace kj_rs_demo
