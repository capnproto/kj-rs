#[allow(unused)]
use std::{ffi::c_void, pin::Pin};
use std::{
    marker::PhantomData,
    task::{Context, Poll, Waker},
};

use static_assertions::assert_eq_size;

use crate::{FuturePollStatus, KjWaker};

type PollCallback = unsafe extern "C" fn(
    node: *mut c_void,
    waker: *const c_void,
    ret: *mut c_void,
) -> FuturePollStatus;

type FuturePtr<T> = *mut (dyn Future<Output = Result<T, String>> + Send);

/// Represents a dyn Future<Output = T> + Send.
#[repr(C)]
pub struct RustFuture<T> {
    pub fut: FuturePtr<T>,
    pub poll: PollCallback,
    _marker: PhantomData<T>,
}

type InfallibleFuturePtr<T> = *mut (dyn Future<Output = T> + Send);

#[repr(C)]
pub struct RustInfallibleFuture<T> {
    pub fut: InfallibleFuturePtr<T>,
    pub poll: PollCallback,
    _marker: PhantomData<T>,
}

assert_eq_size!(RustFuture<()>, [*mut c_void; 3]);

impl<T: Unpin> RustFuture<T> {
    unsafe extern "C" fn poll(
        fut: *mut c_void,
        waker: *const c_void,
        ret: *mut c_void,
    ) -> FuturePollStatus {
        let fut = unsafe { *(fut.cast::<FuturePtr<T>>()) };
        let fut = unsafe { Pin::new_unchecked(&mut *fut) };
        let waker = unsafe { &*(waker as *const KjWaker) };
        let waker = Waker::from(waker);
        let mut context = Context::from_waker(&waker);
        match fut.poll(&mut context) {
            Poll::Ready(Ok(value)) => {
                unsafe { std::ptr::write(ret as *mut T, value) };
                FuturePollStatus::Complete
            }
            Poll::Ready(Err(error)) => {
                unsafe { std::ptr::write(ret as *mut String, error.to_string()) };
                FuturePollStatus::Error
            }
            Poll::Pending => FuturePollStatus::Pending,
        }
    }
}

impl<T: Unpin> RustInfallibleFuture<T> {
    unsafe extern "C" fn poll(
        fut: *mut c_void,
        waker: *const c_void,
        ret: *mut c_void,
    ) -> FuturePollStatus {
        let fut = unsafe { *(fut.cast::<InfallibleFuturePtr<T>>()) };
        let fut = unsafe { Pin::new_unchecked(&mut *fut) };
        let waker = unsafe { &*(waker as *const KjWaker) };
        let waker = Waker::from(waker);
        let mut context = Context::from_waker(&waker);
        match fut.poll(&mut context) {
            Poll::Ready(value) => {
                unsafe { std::ptr::write(ret as *mut T, value) };
                FuturePollStatus::Complete
            }
            Poll::Pending => FuturePollStatus::Pending,
        }
    }
}

pub fn rust_future<T: Unpin>(
    fut: Pin<Box<dyn Future<Output = Result<T, String>> + Send>>,
) -> RustFuture<T> {
    let fut = Box::into_raw(unsafe { Pin::into_inner_unchecked(fut) });
    let poll = RustFuture::<T>::poll;
    RustFuture {
        fut,
        poll,
        _marker: Default::default(),
    }
}

pub fn rust_infallible_future<T: Unpin>(
    fut: Pin<Box<dyn Future<Output = T> + Send>>,
) -> RustInfallibleFuture<T> {
    let fut = Box::into_raw(unsafe { Pin::into_inner_unchecked(fut) });
    let poll = RustInfallibleFuture::<T>::poll;
    RustInfallibleFuture {
        fut,
        poll,
        _marker: Default::default(),
    }
}
