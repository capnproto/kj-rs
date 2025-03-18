use kj_rs::OwnPromiseNode;
use std::future::Future;
use std::future::IntoFuture;

// =======================================================================================
// Boilerplate follows
//
// TODO(now): Generate boilerplate with a macro.

#[allow(dead_code)]
pub struct PromiseVoid(*const ());

// TODO(now): `where T: Send`? Do I need to do this for Future too?
unsafe impl Send for PromiseVoid {}

impl Drop for PromiseVoid {
    fn drop(&mut self) {
        // TODO(now): Safety comment.
        unsafe {
            crate::ffi::promise_drop_in_place_void(self);
        }
    }
}

impl IntoFuture for PromiseVoid {
    type IntoFuture = ::kj_rs::PromiseFuture<PromiseVoid>;
    type Output = <::kj_rs::PromiseFuture<PromiseVoid> as Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        ::kj_rs::PromiseFuture::new(self)
    }
}

// TODO(now): Safety comment.
unsafe impl ::cxx::ExternType for PromiseVoid {
    type Id = ::cxx::type_id!("::kj_rs_demo::PromiseVoid");
    type Kind = ::cxx::kind::Trivial;
}

impl ::kj_rs::KjPromise for PromiseVoid {
    type Output = ();
    fn into_own_promise_node(self) -> OwnPromiseNode {
        crate::ffi::promise_into_own_promise_node_void(self)
    }
    unsafe fn unwrap(node: OwnPromiseNode) -> std::result::Result<Self::Output, cxx::Exception> {
        crate::ffi::own_promise_node_unwrap_void(node)
    }
}

// ---------------------------------------------------------

#[allow(dead_code)]
pub struct PromiseI32(*const ());

// TODO(now): `where T: Send`? Do I need to do this for Future too?
unsafe impl Send for PromiseI32 {}

impl Drop for PromiseI32 {
    fn drop(&mut self) {
        // TODO(now): Safety comment.
        unsafe {
            crate::ffi::promise_drop_in_place_i32(self);
        }
    }
}

impl IntoFuture for PromiseI32 {
    type IntoFuture = ::kj_rs::PromiseFuture<PromiseI32>;
    type Output = <::kj_rs::PromiseFuture<PromiseI32> as Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        ::kj_rs::PromiseFuture::new(self)
    }
}

// TODO(now): Safety comment.
unsafe impl ::cxx::ExternType for PromiseI32 {
    type Id = cxx::type_id!("::kj_rs_demo::PromiseI32");
    type Kind = cxx::kind::Trivial;
}

impl ::kj_rs::KjPromise for PromiseI32 {
    type Output = i32;
    fn into_own_promise_node(self) -> OwnPromiseNode {
        crate::ffi::promise_into_own_promise_node_i32(self)
    }
    unsafe fn unwrap(node: OwnPromiseNode) -> std::result::Result<Self::Output, cxx::Exception> {
        crate::ffi::own_promise_node_unwrap_i32(node)
    }
}
