use std::future::Future;

use std::pin::Pin;

use std::task::Context;
use std::task::Poll;
use std::task::Poll::Pending;
use std::task::Poll::Ready;
use std::task::Waker;

use cxx::ExternType;

use crate::ffi::KjWaker;

use crate::Result;

// =======================================================================================

pub struct BoxFutureVoid(::kj_rs::BoxFuture<()>);

impl<F: Future<Output = ()> + Send + 'static> From<Pin<Box<F>>> for BoxFutureVoid {
    fn from(value: Pin<Box<F>>) -> Self {
        Self(value.into())
    }
}

impl Future for BoxFutureVoid {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // TODO(now): Safety comment.
        let pinned = unsafe { self.map_unchecked_mut(|s| &mut s.0) };
        pinned.poll(cx)
    }
}

// TODO(now): Define these trait implementations with a macro?

// Safety: The size of a Pin<P> is the size of P; the size of a Box<T> is the size of a reference to
// T, and references to `dyn Trait` types contain two pointers: one for the object, one for the
// vtable. So, the size of `Pin<Box<dyn Future<Output = T>>>` (a.k.a. `BoxFuture<T>`) is two
// pointers, and that is unlikely to change.
//
// https://doc.rust-lang.org/std/keyword.dyn.html
// - "As such, a dyn Trait reference contains two pointers."
unsafe impl ExternType for BoxFutureVoid {
    type Id = cxx::type_id!("kj_rs_demo::BoxFutureVoid");
    type Kind = cxx::kind::Trivial;
}

pub fn box_future_poll_void(
    future: Pin<&mut BoxFutureVoid>,
    waker: &KjWaker,
    fulfiller: Pin<&mut crate::ffi::BoxFutureFulfillerVoid>,
) -> bool {
    let waker = Waker::from(waker);
    let mut cx = Context::from_waker(&waker);
    match future.poll(&mut cx) {
        Ready(_v) => {
            fulfiller.fulfill();
            true
        }
        Pending => false,
    }
}

// TODO(now): Safety comment.
#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "box_future_drop_in_place_void"]
pub unsafe extern "C" fn box_future_drop_in_place_void(ptr: *mut BoxFutureVoid) {
    std::ptr::drop_in_place(ptr);
}

// ---------------------------------------------------------

pub struct BoxFutureFallibleVoid(::kj_rs::BoxFuture<Result<()>>);

impl<F: Future<Output = Result<()>> + Send + 'static> From<Pin<Box<F>>> for BoxFutureFallibleVoid {
    fn from(value: Pin<Box<F>>) -> Self {
        Self(value.into())
    }
}

impl Future for BoxFutureFallibleVoid {
    type Output = Result<()>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // TODO(now): Safety comment.
        let pinned = unsafe { self.map_unchecked_mut(|s| &mut s.0) };
        pinned.poll(cx)
    }
}

// Safety: The size of a Pin<P> is the size of P; the size of a Box<T> is the size of a reference to
// T, and references to `dyn Trait` types contain two pointers: one for the object, one for the
// vtable. So, the size of `Pin<Box<dyn Future<Output = T>>>` (a.k.a. `BoxFuture<T>`) is two
// pointers, and that is unlikely to change.
//
// https://doc.rust-lang.org/std/keyword.dyn.html
// - "As such, a dyn Trait reference contains two pointers."
unsafe impl ExternType for BoxFutureFallibleVoid {
    type Id = cxx::type_id!("kj_rs_demo::BoxFutureFallibleVoid");
    type Kind = cxx::kind::Trivial;
}

pub fn box_future_poll_fallible_void(
    future: Pin<&mut BoxFutureFallibleVoid>,
    waker: &KjWaker,
    fulfiller: Pin<&mut crate::ffi::BoxFutureFulfillerFallibleVoid>,
) -> Result<bool> {
    let waker = Waker::from(waker);
    let mut cx = Context::from_waker(&waker);
    match future.poll(&mut cx) {
        Ready(Ok(_v)) => {
            fulfiller.fulfill();
            Ok(true)
        }
        Ready(Err(e)) => Err(e),
        Pending => Ok(false),
    }
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "box_future_drop_in_place_fallible_void"]
pub unsafe extern "C" fn box_future_drop_in_place_fallible_void(ptr: *mut BoxFutureFallibleVoid) {
    std::ptr::drop_in_place(ptr);
}

// ---------------------------------------------------------

pub struct BoxFutureFallibleI32(::kj_rs::BoxFuture<Result<i32>>);

impl<F: Future<Output = Result<i32>> + Send + 'static> From<Pin<Box<F>>> for BoxFutureFallibleI32 {
    fn from(value: Pin<Box<F>>) -> Self {
        Self(value.into())
    }
}

impl Future for BoxFutureFallibleI32 {
    type Output = Result<i32>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // TODO(now): Safety comment.
        let pinned = unsafe { self.map_unchecked_mut(|s| &mut s.0) };
        pinned.poll(cx)
    }
}

unsafe impl ExternType for BoxFutureFallibleI32 {
    type Id = cxx::type_id!("kj_rs_demo::BoxFutureFallibleI32");
    type Kind = cxx::kind::Trivial;
}

pub fn box_future_poll_fallible_i32(
    future: Pin<&mut BoxFutureFallibleI32>,
    waker: &KjWaker,
    fulfiller: Pin<&mut crate::ffi::BoxFutureFulfillerFallibleI32>,
) -> Result<bool> {
    let waker = Waker::from(waker);
    let mut cx = Context::from_waker(&waker);
    match future.poll(&mut cx) {
        Ready(Ok(v)) => {
            fulfiller.fulfill(v);
            Ok(true)
        }
        Ready(Err(e)) => Err(e),
        Pending => Ok(false),
    }
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "box_future_drop_in_place_fallible_i32"]
pub unsafe extern "C" fn box_future_drop_in_place_fallible_i32(ptr: *mut BoxFutureFallibleI32) {
    std::ptr::drop_in_place(ptr);
}
