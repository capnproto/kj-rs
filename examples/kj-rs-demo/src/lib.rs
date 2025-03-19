mod promise;
pub use promise::Promise;

mod future_boilerplate;
use future_boilerplate::*;

mod promise_boilerplate;

mod test_futures;
use test_futures::*;

type Result<T> = std::io::Result<T>;
type Error = std::io::Error;

#[cxx::bridge(namespace = "kj_rs_demo")]
mod ffi {
    // Cross-namespace example from https://github.com/dtolnay/cxx/pull/465/files
    #[namespace = "kj_rs"]
    unsafe extern "C++" {
        include!("kj-rs/waker.h");
        type OwnPromiseNode = kj_rs::OwnPromiseNode;
    }

    // -----------------------------------------------------
    // Boilerplate

    unsafe extern "C++" {
        include!("kj-rs-demo/future-boilerplate.h");

        type BoxFutureVoid = crate::BoxFutureVoid;
        type BoxFutureFallibleVoid = crate::BoxFutureFallibleVoid;
        type BoxFutureFallibleI32 = crate::BoxFutureFallibleI32;
    }

    unsafe extern "C++" {
        include!("kj-rs-demo/promise-boilerplate.h");

        // TODO(now): Generate boilerplate with a macro.
        type PromiseVoid = crate::Promise<()>;
        fn own_promise_node_unwrap_void(node: OwnPromiseNode) -> Result<()>;
        unsafe fn promise_drop_in_place_void(promise: *mut PromiseVoid);
        fn promise_into_own_promise_node_void(promise: PromiseVoid) -> OwnPromiseNode;

        type PromiseI32 = crate::Promise<i32>;
        fn own_promise_node_unwrap_i32(node: OwnPromiseNode) -> Result<i32>;
        unsafe fn promise_drop_in_place_i32(promise: *mut PromiseI32);
        fn promise_into_own_promise_node_i32(promise: PromiseI32) -> OwnPromiseNode;
    }

    // -----------------------------------------------------
    // Test functions

    // Helper functions to create Promises for testing purposes.
    unsafe extern "C++" {
        include!("kj-rs-demo/test-promises.h");

        fn new_ready_promise_void() -> PromiseVoid;
        fn new_pending_promise_void() -> PromiseVoid;
        fn new_coroutine_promise_void() -> PromiseVoid;

        fn new_errored_promise_void() -> PromiseVoid;
        fn new_ready_promise_i32(value: i32) -> PromiseI32;
    }

    enum CloningAction {
        None,
        CloneSameThread,
        CloneBackgroundThread,
        WakeByRefThenCloneSameThread,
    }

    enum WakingAction {
        None,
        WakeByRefSameThread,
        WakeByRefBackgroundThread,
        WakeSameThread,
        WakeBackgroundThread,
    }

    // Helper functions to create BoxFutureVoids for testing purposes.
    extern "Rust" {
        fn new_pending_future_void() -> BoxFutureVoid;
        fn new_ready_future_void() -> BoxFutureVoid;
        fn new_waking_future_void(
            cloning_action: CloningAction,
            waking_action: WakingAction,
        ) -> BoxFutureVoid;
        fn new_threaded_delay_future_void() -> BoxFutureVoid;
        fn new_layered_ready_future_void() -> BoxFutureFallibleVoid;

        fn new_naive_select_future_void() -> BoxFutureFallibleVoid;
        fn new_wrapped_waker_future_void() -> BoxFutureFallibleVoid;

        fn new_errored_future_fallible_void() -> BoxFutureFallibleVoid;
        fn new_error_handling_future_void() -> BoxFutureVoid;

        fn new_awaiting_future_i32() -> BoxFutureVoid;
        fn new_ready_future_fallible_i32(value: i32) -> BoxFutureFallibleI32;
    }
}
