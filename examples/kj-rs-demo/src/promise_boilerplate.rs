use kj_rs::OwnPromiseNode;
use std::future::Future;
use std::future::IntoFuture;

// =======================================================================================
// Boilerplate follows
//
// TODO(now): Generate boilerplate with a macro.

#[allow(dead_code)]
#[repr(transparent)]
pub struct PromiseVoid(*const ());

// TODO(now): `where T: Send`? Do I need to do this for Future too?
unsafe impl Send for PromiseVoid {}

impl Drop for PromiseVoid {
    fn drop(&mut self) {
        // TODO(now): Safety comment.
        unsafe extern "C" {
            fn promise_drop_in_place_void(promise: *mut PromiseVoid);
        }
        unsafe {
            promise_drop_in_place_void(self);
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
        unsafe extern "C" {
            fn promise_into_own_promise_node_void(
                promise: *mut PromiseVoid,
                result: *mut OwnPromiseNode,
            );
        }
        let mut promise = ::cxx::core::mem::MaybeUninit::new(self);
        let mut ret = ::cxx::core::mem::MaybeUninit::<OwnPromiseNode>::uninit();
        unsafe {
            promise_into_own_promise_node_void(promise.as_mut_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }
    // https://github.com/dtolnay/cxx/blob/86cd652c06c5cb4c2e24d3ab555cf707b4ae0883/macro/src/expand.rs#L635
    unsafe fn unwrap(node: OwnPromiseNode) -> std::result::Result<Self::Output, kj_rs::Exception> {
        unsafe extern "C" {
            fn own_promise_node_unwrap_void(
                node: *mut OwnPromiseNode,
                result: *mut (),
            ) -> ::kj_rs::private::Result;
        }
        let mut node = ::cxx::core::mem::MaybeUninit::new(node);
        let mut ret = ::cxx::core::mem::MaybeUninit::<Self::Output>::uninit();
        own_promise_node_unwrap_void(node.as_mut_ptr(), ret.as_mut_ptr()).exception()?;
        Ok(ret.assume_init())
    }
}

// ---------------------------------------------------------

#[allow(dead_code)]
#[repr(transparent)]
pub struct PromiseI32(*const ());

// TODO(now): `where T: Send`? Do I need to do this for Future too?
unsafe impl Send for PromiseI32 {}

impl Drop for PromiseI32 {
    fn drop(&mut self) {
        // TODO(now): Safety comment.
        unsafe extern "C" {
            fn promise_drop_in_place_i32(promise: *mut PromiseI32);
        }
        unsafe {
            promise_drop_in_place_i32(self);
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
        unsafe extern "C" {
            fn promise_into_own_promise_node_i32(
                promise: *mut PromiseI32,
                result: *mut OwnPromiseNode,
            );
        }
        let mut promise = ::cxx::core::mem::MaybeUninit::new(self);
        let mut ret = ::cxx::core::mem::MaybeUninit::<OwnPromiseNode>::uninit();
        unsafe {
            promise_into_own_promise_node_i32(promise.as_mut_ptr(), ret.as_mut_ptr());
            ret.assume_init()
        }
    }
    // https://github.com/dtolnay/cxx/blob/86cd652c06c5cb4c2e24d3ab555cf707b4ae0883/macro/src/expand.rs#L635
    unsafe fn unwrap(node: OwnPromiseNode) -> std::result::Result<Self::Output, kj_rs::Exception> {
        unsafe extern "C" {
            fn own_promise_node_unwrap_i32(
                node: *mut OwnPromiseNode,
                result: *mut i32,
            ) -> ::kj_rs::private::Result;
        }
        // `own_promise_node_unwrap_*()` consumes `node`, so we must avoid dropping it ourselves.
        let mut node = ::cxx::core::mem::MaybeUninit::new(node);
        let mut ret = ::cxx::core::mem::MaybeUninit::<Self::Output>::uninit();
        own_promise_node_unwrap_i32(node.as_mut_ptr(), ret.as_mut_ptr()).exception()?;
        Ok(ret.assume_init())
    }
}
