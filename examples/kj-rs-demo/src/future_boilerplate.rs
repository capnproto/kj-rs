use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use kj_rs::FuturePollStatus;
use kj_rs::KjWaker;

use crate::Result;

// =======================================================================================

pub struct BoxFutureVoidInfallible(::kj_rs::BoxFuture<std::result::Result<(), Infallible>>);

impl<F: Future<Output = std::result::Result<(), Infallible>> + Send + 'static> From<Pin<Box<F>>>
    for BoxFutureVoidInfallible
{
    fn from(value: Pin<Box<F>>) -> Self {
        Self(value.into())
    }
}

impl Future for BoxFutureVoidInfallible {
    type Output = std::result::Result<(), Infallible>;
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
unsafe impl ::cxx::ExternType for BoxFutureVoidInfallible {
    type Id = ::cxx::type_id!("kj_rs_demo::BoxFutureVoidInfallible");
    type Kind = ::cxx::kind::Trivial;
}

// TODO(now): Safety comment.
#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "BoxFutureVoidInfallible_drop_in_place"]
pub unsafe extern "C" fn BoxFutureVoidInfallible_drop_in_place(ptr: *mut BoxFutureVoidInfallible) {
    ::std::ptr::drop_in_place(ptr);
}

// TODO(now): Safety comment.
#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "BoxFutureVoidInfallible_poll"]
pub unsafe extern "C" fn BoxFutureVoidInfallible_poll(
    future: Pin<&mut BoxFutureVoidInfallible>,
    waker: &KjWaker,
    result: *mut (),
) -> ::kj_rs::FuturePollStatus {
    ::kj_rs::box_future_poll::<BoxFutureVoidInfallible, (), Infallible>(future, waker, result)
}

// ---------------------------------------------------------

pub struct BoxFutureVoid(::kj_rs::BoxFuture<Result<()>>);

impl<F: Future<Output = Result<()>> + Send + 'static> From<Pin<Box<F>>> for BoxFutureVoid {
    fn from(value: Pin<Box<F>>) -> Self {
        Self(value.into())
    }
}

impl Future for BoxFutureVoid {
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
unsafe impl ::cxx::ExternType for BoxFutureVoid {
    type Id = ::cxx::type_id!("::kj_rs_demo::BoxFutureVoid");
    type Kind = ::cxx::kind::Trivial;
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "BoxFutureVoid_drop_in_place"]
pub unsafe extern "C" fn BoxFutureVoid_drop_in_place(ptr: *mut BoxFutureVoid) {
    ::std::ptr::drop_in_place(ptr);
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "BoxFutureVoid_poll"]
pub unsafe extern "C" fn BoxFutureVoid_poll(
    future: Pin<&mut BoxFutureVoid>,
    waker: &KjWaker,
    result: *mut (),
) -> ::kj_rs::FuturePollStatus {
    ::kj_rs::box_future_poll::<BoxFutureVoid, (), crate::Error>(future, waker, result)
}

// ---------------------------------------------------------

pub struct BoxFutureI32(::kj_rs::BoxFuture<Result<i32>>);

impl<F: Future<Output = Result<i32>> + Send + 'static> From<Pin<Box<F>>> for BoxFutureI32 {
    fn from(value: Pin<Box<F>>) -> Self {
        Self(value.into())
    }
}

impl Future for BoxFutureI32 {
    type Output = Result<i32>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // TODO(now): Safety comment.
        let pinned = unsafe { self.map_unchecked_mut(|s| &mut s.0) };
        pinned.poll(cx)
    }
}

unsafe impl ::cxx::ExternType for BoxFutureI32 {
    type Id = ::cxx::type_id!("kj_rs_demo::BoxFutureI32");
    type Kind = ::cxx::kind::Trivial;
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "BoxFutureI32_drop_in_place"]
pub unsafe extern "C" fn BoxFutureI32_drop_in_place(ptr: *mut BoxFutureI32) {
    ::std::ptr::drop_in_place(ptr);
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[export_name = "BoxFutureI32_poll"]
pub unsafe extern "C" fn BoxFutureI32_poll(
    future: Pin<&mut BoxFutureI32>,
    waker: &KjWaker,
    result: *mut (),
) -> FuturePollStatus {
    ::kj_rs::box_future_poll::<BoxFutureI32, i32, crate::Error>(future, waker, result)
}
