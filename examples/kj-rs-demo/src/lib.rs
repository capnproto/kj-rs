use std::future::Future;

mod promise_boilerplate;
use promise_boilerplate::*;

mod test_futures;
use test_futures::*;

type InfallibleResult<T> = std::result::Result<T, std::convert::Infallible>;

type Result<T> = std::io::Result<T>;
type Error = std::io::Error;

#[kj_rs::bridge_future(namespace = kj_rs_demo)]
unsafe impl Future for BoxFutureVoidInfallible {
    type Output = InfallibleResult<()>;
}

#[kj_rs::bridge_future(namespace = kj_rs_demo)]
unsafe impl Future for BoxFutureVoid {
    type Output = Result<()>;
}

#[kj_rs::bridge_future(namespace = kj_rs_demo)]
unsafe impl Future for BoxFutureI32 {
    type Output = Result<i32>;
}

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

    unsafe extern "C++" {
        include!("kj-rs-demo/promise-boilerplate.h");

        type PromiseVoid = crate::PromiseVoid;
        type PromiseI32 = crate::PromiseI32;
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
