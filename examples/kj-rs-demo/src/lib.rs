mod future_boilerplate;
use future_boilerplate::*;

mod test_futures;
use test_futures::*;

type Result<T> = std::io::Result<T>;
type Error = std::io::Error;

#[cxx::bridge(namespace = "kj_rs_demo")]
mod ffi {

    // -----------------------------------------------------
    // Boilerplate

    unsafe extern "C++" {
        include!("kj-rs-demo/future-boilerplate.h");

        type BoxFutureVoidInfallible = crate::BoxFutureVoidInfallible;
        type BoxFutureVoid = crate::BoxFutureVoid;
        type BoxFutureI32 = crate::BoxFutureI32;
    }

    // -----------------------------------------------------
    // Test functions

    // Helper functions to create Promises for testing purposes.
    unsafe extern "C++" {
        include!("kj-rs-demo/test-promises.h");

        async fn new_ready_promise_void();
        async fn new_pending_promise_void();
        async fn new_coroutine_promise_void();

        async fn new_errored_promise_void();
        async fn new_ready_promise_i32(value: i32) -> i32;
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
        fn new_pending_future_void() -> BoxFutureVoidInfallible;
        fn new_ready_future_void() -> BoxFutureVoidInfallible;
        fn new_waking_future_void(
            cloning_action: CloningAction,
            waking_action: WakingAction,
        ) -> BoxFutureVoidInfallible;
        fn new_threaded_delay_future_void() -> BoxFutureVoidInfallible;
        fn new_layered_ready_future_void() -> BoxFutureVoid;

        fn new_naive_select_future_void() -> BoxFutureVoid;
        fn new_wrapped_waker_future_void() -> BoxFutureVoid;

        fn new_errored_future_void() -> BoxFutureVoid;
        fn new_error_handling_future_void_infallible() -> BoxFutureVoidInfallible;

        fn new_awaiting_future_i32() -> BoxFutureVoidInfallible;
        fn new_ready_future_i32(value: i32) -> BoxFutureI32;
    }
}
