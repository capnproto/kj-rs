use cxx::ExternType;

use crate::PromiseAwaiter;

use std::marker::PhantomData;

use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

type CxxResult<T> = std::result::Result<T, cxx::Exception>;

// The inner pointer is never read on Rust's side, so Rust thinks it's dead code.
#[allow(dead_code)]
#[repr(transparent)]
pub struct OwnPromiseNode(*const ());

// Safety: KJ Promises are not associated with threads, but with event loops at construction time.
// Therefore, they can be polled from any thread, as long as that thread has the correct event loop
// active at the time of the call to `poll()`. If the correct event loop is not active, the
// OwnPromiseNode's API will typically panic, undefined behavior could be possible. However, Rust
// doesn't have direct access to OwnPromiseNode's API. Instead, it can only use the Promise by
// having GuardedRustPromiseAwaiter consume it, and GuardedRustPromiseAwaiter implements the
// correct-executor guarantee.
unsafe impl Send for OwnPromiseNode {}

impl Drop for OwnPromiseNode {
    fn drop(&mut self) {
        // Safety:
        // 1. Pointer to self is non-null, and obviously points to valid memory.
        // 2. We do not read or write to the OwnPromiseNode's memory, so there are no atomicity nor
        //    interleaved pointer/reference access concerns.
        //
        // https://doc.rust-lang.org/std/ptr/index.html#safety
        unsafe {
            crate::ffi::own_promise_node_drop_in_place(self);
        }
    }
}

// Safety: We have a static_assert in promise.c++ which breaks if you change the size or alignment
// of the C++ definition of OwnPromiseNode, with a comment directing the reader to adjust the
// OwnPromiseNode definition in this .rs file.
//
// https://docs.rs/cxx/latest/cxx/trait.ExternType.html#integrating-with-bindgen-generated-types
unsafe impl ExternType for OwnPromiseNode {
    type Id = cxx::type_id!("::kj_rs::OwnPromiseNode");
    type Kind = cxx::kind::Trivial;
}

pub trait KjPromise: Sized {
    type Output;
    fn into_own_promise_node(self) -> OwnPromiseNode;

    // Safety: You must guarantee that `node` was previously returned from this same type's
    // `into_own_promise_node()` implementation.
    unsafe fn unwrap(node: OwnPromiseNode) -> CxxResult<Self::Output>;
}

pub struct PromiseFuture<P: KjPromise> {
    awaiter: PromiseAwaiter,
    _marker: PhantomData<P>,
}

impl<P: KjPromise> PromiseFuture<P> {
    pub fn new(promise: P) -> Self {
        PromiseFuture {
            awaiter: PromiseAwaiter::new(promise.into_own_promise_node()),
            _marker: Default::default(),
        }
    }
}

impl<P: KjPromise> Future for PromiseFuture<P> {
    type Output = CxxResult<P::Output>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // TODO(now): Safety comment.
        let mut awaiter = unsafe { self.map_unchecked_mut(|s| &mut s.awaiter) };
        if awaiter.as_mut().poll(cx) {
            let node = awaiter.get_awaiter().take_own_promise_node();
            // TODO(now): Safety comment.
            let value = unsafe { P::unwrap(node) };
            Poll::Ready(value)
        } else {
            Poll::Pending
        }
    }
}
