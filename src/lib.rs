mod awaiter;
pub use awaiter::GuardedRustPromiseAwaiter;
use awaiter::OptionWaker;
pub use awaiter::PromiseAwaiter;
use awaiter::PtrGuardedRustPromiseAwaiter;
use awaiter::WakerRef;

mod lazy_pin_init;

mod promise;
pub use promise::OwnPromiseNode;
pub use promise::PtrOwnPromiseNode;

mod waker;

pub type CxxResult<T> = std::result::Result<T, cxx::Exception>;

pub type Result<T> = std::io::Result<T>;
pub type Error = std::io::Error;

pub use crate::ffi::KjWaker;

#[cxx::bridge(namespace = "kj_rs")]
mod ffi {
    extern "Rust" {
        type WakerRef<'a>;
    }

    extern "Rust" {
        // We expose the Rust Waker type to C++ through this OptionWaker reference wrapper. cxx-rs
        // does not allow us to export types defined outside this crate, such as Waker, directly.
        //
        // `LazyRustPromiseAwaiter` (the implementation of `.await` syntax/the IntoFuture trait),
        // stores a OptionWaker immediately after `GuardedRustPromiseAwaiter` in declaration order.
        // pass the Waker to the `RustPromiseAwaiter` class, which is implemented in C++
        type OptionWaker;
        fn set(&mut self, waker: &WakerRef);
        fn set_none(&mut self);
        fn wake(&mut self);
    }

    unsafe extern "C++" {
        include!("kj-rs/waker.h");

        // Match the definition of the abstract virtual class in the C++ header.
        type KjWaker;
        fn clone(&self) -> *const KjWaker;
        fn wake(&self);
        fn wake_by_ref(&self);
        fn drop(&self);
    }

    unsafe extern "C++" {
        include!("kj-rs/promise.h");

        type OwnPromiseNode = crate::OwnPromiseNode;
        type PtrOwnPromiseNode = crate::PtrOwnPromiseNode;

        unsafe fn own_promise_node_drop_in_place(node: PtrOwnPromiseNode);
    }

    unsafe extern "C++" {
        include!("kj-rs/awaiter.h");

        type GuardedRustPromiseAwaiter = crate::GuardedRustPromiseAwaiter;
        type PtrGuardedRustPromiseAwaiter = crate::PtrGuardedRustPromiseAwaiter;

        unsafe fn guarded_rust_promise_awaiter_new_in_place(
            ptr: PtrGuardedRustPromiseAwaiter,
            rust_waker_ptr: *mut OptionWaker,
            node: OwnPromiseNode,
        );
        unsafe fn guarded_rust_promise_awaiter_drop_in_place(ptr: PtrGuardedRustPromiseAwaiter);

        // TODO(now): Safety comment.
        unsafe fn poll(
            self: Pin<&mut GuardedRustPromiseAwaiter>,
            waker: &WakerRef,
            maybe_kj_waker: *const KjWaker,
        ) -> bool;

        fn take_own_promise_node(self: Pin<&mut GuardedRustPromiseAwaiter>) -> OwnPromiseNode;
    }
}
