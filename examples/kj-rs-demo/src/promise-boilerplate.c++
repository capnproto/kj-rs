#include <kj-rs-demo/promise-boilerplate.h>
#include <rust/cxx.h>

namespace kj_rs_demo {

namespace {

template <typename T>
T unwrapNode(OwnPromiseNode node) {
  kj::_::ExceptionOr<kj::_::FixVoid<T>> result;

  node->get(result);
  KJ_IF_SOME(exception, kj::runCatchingExceptions([&node]() {
    node = nullptr;
  })) {
    result.addException(kj::mv(exception));
  }

  return kj::_::convertToReturn(kj::mv(result));
}

extern "C" {
  ::kj_rs::repr::PtrLen cxxbridge1$exception(const char *, std::size_t len) noexcept;
}

template <typename T>
::kj_rs::repr::PtrLen unwrapImpl(OwnPromiseNode node, kj::_::FixVoid<T>* result) noexcept {
  // Similar to cxxbridge-cmd generated code:
  // https://github.com/dtolnay/cxx/blob/86cd652c06c5cb4c2e24d3ab555cf707b4ae0883/gen/src/write.rs#L787
  ::kj_rs::repr::PtrLen error;
  KJ_IF_SOME(exception, kj::runCatchingExceptions([&] {
    if constexpr(kj::isSameType<void, T>()) {
      unwrapNode<T>(kj::mv(node));
      new (result) kj::_::FixVoid<void>();
    } else {
      new (result) kj::_::FixVoid<T>(unwrapNode<T>(kj::mv(node)));
    }
    error.ptr = nullptr;
  })) {
    auto description = exception.getDescription();
    error = cxxbridge1$exception(description.cStr(), description.size());
  }
  return error;
}

}  // namespace

// TODO(now): Generate boilerplate with a macro.
extern "C" void promise_into_own_promise_node_void(PromiseVoid* promise, OwnPromiseNode* result) noexcept {
  new (result) OwnPromiseNode(kj::_::PromiseNode::from(kj::mv(*promise)));
};
extern "C" void promise_drop_in_place_void(PromiseVoid* promise) noexcept {
  // TODO(now): How to propagate exceptions?
  kj::dtor(*promise);
}
extern "C" ::kj_rs::repr::PtrLen own_promise_node_unwrap_void(OwnPromiseNode* node, kj::_::FixVoid<void>* result) noexcept {
  return unwrapImpl<void>(kj::mv(*node), result);
}

extern "C" void promise_into_own_promise_node_i32(PromiseI32* promise, OwnPromiseNode* result) noexcept {
  new (result) OwnPromiseNode(kj::_::PromiseNode::from(kj::mv(*promise)));
};
extern "C" void promise_drop_in_place_i32(PromiseI32* promise) noexcept {
  // TODO(now): How to propagate exceptions?
  kj::dtor(*promise);
}
extern "C" ::kj_rs::repr::PtrLen own_promise_node_unwrap_i32(OwnPromiseNode* node, kj::_::FixVoid<int32_t>* result) noexcept {
  return unwrapImpl<int32_t>(kj::mv(*node), result);
}

}  // namespace kj_rs_demo
