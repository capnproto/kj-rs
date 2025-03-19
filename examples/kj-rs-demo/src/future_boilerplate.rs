use std::future::Future;

use std::pin::Pin;

use std::task::Context;
use std::task::Poll;

use kj_rs::FuturePollStatus;
use kj_rs::KjWaker;

use crate::Result;

// =======================================================================================

pub struct BoxFutureVoid(::kj_rs::BoxFuture<std::result::Result<(), std::convert::Infallible>>);

impl<F: Future<Output = std::result::Result<(), std::convert::Infallible>> + Send + 'static>
    From<Pin<Box<F>>> for BoxFutureVoid
{
    fn from(value: Pin<Box<F>>) -> Self {
        Self(value.into())
    }
}

impl Future for BoxFutureVoid {
    type Output = std::result::Result<(), std::convert::Infallible>;
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
unsafe impl ::cxx::ExternType for BoxFutureVoid {
    type Id = ::cxx::type_id!("kj_rs_demo::BoxFutureVoid");
    type Kind = ::cxx::kind::Trivial;
}

// TODO(now): Safety comment.
#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "box_future_drop_in_place_void"]
pub unsafe extern "C" fn box_future_drop_in_place_void(ptr: *mut BoxFutureVoid) {
    ::std::ptr::drop_in_place(ptr);
}

// TODO(now): Safety comment.
#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "box_future_poll_void"]
pub unsafe extern "C" fn box_future_poll_void(
    future: Pin<&mut BoxFutureVoid>,
    waker: &KjWaker,
    result: *mut (),
) -> ::kj_rs::FuturePollStatus {
    ::kj_rs::box_future_poll::<BoxFutureVoid, (), ::std::convert::Infallible>(future, waker, result)
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
unsafe impl ::cxx::ExternType for BoxFutureFallibleVoid {
    type Id = ::cxx::type_id!("::kj_rs_demo::BoxFutureFallibleVoid");
    type Kind = ::cxx::kind::Trivial;
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "box_future_drop_in_place_fallible_void"]
pub unsafe extern "C" fn box_future_drop_in_place_fallible_void(ptr: *mut BoxFutureFallibleVoid) {
    ::std::ptr::drop_in_place(ptr);
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "box_future_poll_fallible_void"]
pub unsafe extern "C" fn box_future_poll_fallible_void(
    future: Pin<&mut BoxFutureFallibleVoid>,
    waker: &KjWaker,
    result: *mut (),
) -> ::kj_rs::FuturePollStatus {
    ::kj_rs::box_future_poll::<BoxFutureFallibleVoid, (), crate::Error>(future, waker, result)
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

unsafe impl ::cxx::ExternType for BoxFutureFallibleI32 {
    type Id = ::cxx::type_id!("kj_rs_demo::BoxFutureFallibleI32");
    type Kind = ::cxx::kind::Trivial;
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "box_future_drop_in_place_fallible_i32"]
pub unsafe extern "C" fn box_future_drop_in_place_fallible_i32(ptr: *mut BoxFutureFallibleI32) {
    ::std::ptr::drop_in_place(ptr);
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "box_future_poll_fallible_i32"]
pub unsafe extern "C" fn box_future_poll_fallible_i32(
    future: Pin<&mut BoxFutureFallibleI32>,
    waker: &KjWaker,
    result: *mut (),
) -> FuturePollStatus {
    ::kj_rs::box_future_poll::<BoxFutureFallibleI32, i32, crate::Error>(future, waker, result)
}
