use crate::promise::PromiseTarget;
use crate::Promise;
use kj_rs::OwnPromiseNode;

// =======================================================================================
// Boilerplate follows
//
// TODO(now): Generate boilerplate with a macro.

// TODO(now): Safety comment.
unsafe impl cxx::ExternType for Promise<()> {
    type Id = cxx::type_id!("::kj_rs_demo::PromiseVoid");
    type Kind = cxx::kind::Trivial;
}

impl PromiseTarget for () {
    fn into_own_promise_node(promise: Promise<Self>) -> OwnPromiseNode {
        crate::ffi::promise_into_own_promise_node_void(promise)
    }
    unsafe fn drop_in_place(ptr: *mut Promise<Self>) {
        crate::ffi::promise_drop_in_place_void(ptr);
    }
    fn unwrap(node: OwnPromiseNode) -> std::result::Result<Self, cxx::Exception> {
        crate::ffi::own_promise_node_unwrap_void(node)
    }
}

// ---------------------------------------------------------

// TODO(now): Safety comment.
unsafe impl cxx::ExternType for Promise<i32> {
    type Id = cxx::type_id!("::kj_rs_demo::PromiseI32");
    type Kind = cxx::kind::Trivial;
}

impl PromiseTarget for i32 {
    fn into_own_promise_node(promise: Promise<Self>) -> OwnPromiseNode {
        crate::ffi::promise_into_own_promise_node_i32(promise)
    }
    unsafe fn drop_in_place(ptr: *mut Promise<Self>) {
        crate::ffi::promise_drop_in_place_i32(ptr);
    }
    fn unwrap(node: OwnPromiseNode) -> std::result::Result<Self, cxx::Exception> {
        crate::ffi::own_promise_node_unwrap_i32(node)
    }
}
