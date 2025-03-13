// This file contains boilerplate which must occur once per crate, rather than once per type.

use kj_rs::CxxResult;
use kj_rs::OwnPromiseNode;
use std::marker::PhantomData;

use std::future::Future;
use std::future::IntoFuture;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

pub trait PromiseTarget: Sized {
    fn into_own_promise_node(this: Promise<Self>) -> OwnPromiseNode;
    unsafe fn drop_in_place(this: PtrPromise<Self>);
    fn unwrap(node: OwnPromiseNode) -> CxxResult<Self>;
}

#[allow(dead_code)]
pub struct Promise<T: PromiseTarget>(*const (), PhantomData<T>);

// TODO(now): `where T: Send`? Do I need to do this for Future too?
unsafe impl<T: PromiseTarget> Send for Promise<T> {}

impl<T: PromiseTarget> Drop for Promise<T> {
    fn drop(&mut self) {
        // TODO(now): Safety comment.
        unsafe {
            T::drop_in_place(PtrPromise(self));
        }
    }
}

#[repr(transparent)]
pub struct PtrPromise<T: PromiseTarget>(*mut Promise<T>);

impl<T: PromiseTarget> IntoFuture for Promise<T> {
    type IntoFuture = PromiseFuture<T>;
    type Output = <PromiseFuture<T> as Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        PromiseFuture::new(self)
    }
}

pub struct PromiseFuture<T: PromiseTarget> {
    awaiter: kj_rs::PromiseAwaiter,
    _marker: PhantomData<T>,
}

impl<T: PromiseTarget> PromiseFuture<T> {
    fn new(promise: Promise<T>) -> Self {
        PromiseFuture {
            awaiter: kj_rs::PromiseAwaiter::new(T::into_own_promise_node(promise)),
            _marker: Default::default(),
        }
    }
}

impl<T: PromiseTarget> Future for PromiseFuture<T> {
    type Output = CxxResult<T>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // TODO(now): Safety comment.
        let mut awaiter = unsafe { self.map_unchecked_mut(|s| &mut s.awaiter) };
        if awaiter.as_mut().poll(cx) {
            Poll::Ready(T::unwrap(awaiter.get_awaiter().take_own_promise_node()))
        } else {
            Poll::Pending
        }
    }
}
