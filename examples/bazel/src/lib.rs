mod future;
pub use future::BoxFuture;
use future::PtrBoxFuture;

mod promise;
pub use promise::Promise;
use promise::PtrPromise;

mod future_boilerplate;
use future_boilerplate::*;

mod promise_boilerplate;

mod test_futures;
use test_futures::*;

type CxxResult<T> = std::result::Result<T, cxx::Exception>;

type Result<T> = std::io::Result<T>;
type Error = std::io::Error;

#[cxx::bridge(namespace = "kj_rs::tests")]
mod ffi {
    // Cross-namespace example from https://github.com/dtolnay/cxx/pull/465/files
    #[namespace = "kj_rs"]
    unsafe extern "C++" {
        include!("kj-rs/waker.h");
        type KjWaker = kj_rs::KjWaker;
        type OwnPromiseNode = kj_rs::OwnPromiseNode;
    }

    // -----------------------------------------------------
    // Boilerplate

    extern "Rust" {
        // TODO(now): Generate boilerplate with a macro.
        fn box_future_poll_void(
            future: Pin<&mut BoxFutureVoid>,
            waker: &KjWaker,
            fulfiller: Pin<&mut BoxFutureFulfillerVoid>,
        ) -> bool;
        unsafe fn box_future_drop_in_place_void(ptr: PtrBoxFutureVoid);

        // TODO(now): Generate boilerplate with a macro.
        fn box_future_poll_fallible_void(
            future: Pin<&mut BoxFutureFallibleVoid>,
            waker: &KjWaker,
            fulfiller: Pin<&mut BoxFutureFulfillerFallibleVoid>,
        ) -> Result<bool>;
        unsafe fn box_future_drop_in_place_fallible_void(ptr: PtrBoxFutureFallibleVoid);

        fn box_future_poll_fallible_i32(
            future: Pin<&mut BoxFutureFallibleI32>,
            waker: &KjWaker,
            fulfiller: Pin<&mut BoxFutureFulfillerFallibleI32>,
        ) -> Result<bool>;
        unsafe fn box_future_drop_in_place_fallible_i32(ptr: PtrBoxFutureFallibleI32);
    }

    unsafe extern "C++" {
        include!("kj-rs/tests/future-boilerplate.h");

        // TODO(now): Generate boilerplate with a macro.
        type BoxFutureVoid = crate::BoxFuture<()>;
        type PtrBoxFutureVoid = crate::PtrBoxFuture<()>;
        type BoxFutureFulfillerVoid;
        fn fulfill(self: Pin<&mut BoxFutureFulfillerVoid>);

        // TODO(now): Generate boilerplate with a macro.
        type BoxFutureFallibleVoid = crate::BoxFuture<crate::Result<()>>;
        type PtrBoxFutureFallibleVoid = crate::PtrBoxFuture<crate::Result<()>>;
        type BoxFutureFulfillerFallibleVoid;
        fn fulfill(self: Pin<&mut BoxFutureFulfillerFallibleVoid>);

        type BoxFutureFallibleI32 = crate::BoxFuture<crate::Result<i32>>;
        type PtrBoxFutureFallibleI32 = crate::PtrBoxFuture<crate::Result<i32>>;
        type BoxFutureFulfillerFallibleI32;
        fn fulfill(self: Pin<&mut BoxFutureFulfillerFallibleI32>, value: i32);
    }

    unsafe extern "C++" {
        include!("kj-rs/tests/promise-boilerplate.h");

        // TODO(now): Generate boilerplate with a macro.
        type PromiseVoid = crate::Promise<()>;
        type PtrPromiseVoid = crate::PtrPromise<()>;
        fn own_promise_node_unwrap_void(node: OwnPromiseNode) -> Result<()>;
        unsafe fn promise_drop_in_place_void(promise: PtrPromiseVoid);
        fn promise_into_own_promise_node_void(promise: PromiseVoid) -> OwnPromiseNode;

        type PromiseI32 = crate::Promise<i32>;
        type PtrPromiseI32 = crate::PtrPromise<i32>;
        fn own_promise_node_unwrap_i32(node: OwnPromiseNode) -> Result<i32>;
        unsafe fn promise_drop_in_place_i32(promise: PtrPromiseI32);
        fn promise_into_own_promise_node_i32(promise: PromiseI32) -> OwnPromiseNode;
    }

    // -----------------------------------------------------
    // Test functions

    // Helper functions to create Promises for testing purposes.
    unsafe extern "C++" {
        include!("kj-rs/tests/test-promises.h");

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
