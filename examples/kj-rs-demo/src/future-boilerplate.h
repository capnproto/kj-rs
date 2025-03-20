#include <kj-rs/awaiter.h>

#include <array>

namespace kj_rs_demo {

class BoxFutureVoidInfallible;

extern "C" void BoxFutureVoidInfallible_drop_in_place(BoxFutureVoidInfallible* ptr);
extern "C" ::kj_rs::FuturePollStatus BoxFutureVoidInfallible_poll(BoxFutureVoidInfallible& future, const ::kj_rs::KjWaker& waker, void* result);

class BoxFutureVoidInfallible {
public:
  BoxFutureVoidInfallible(BoxFutureVoidInfallible&& other) noexcept: repr(other.repr) {
    other.repr = {0, 0};
  }
  ~BoxFutureVoidInfallible() noexcept {
    if (repr != std::array<std::uintptr_t, 2>{0, 0}) {
      // Safety: We can assume that `this` is a valid pointer while we're in the destructor.
      BoxFutureVoidInfallible_drop_in_place(this);
    }
  }

  using ExceptionOrValue = ::kj::_::ExceptionOr<::kj::_::FixVoid<void>>;

  // Poll our Future with the given KjWaker. Returns true if the future returned `Poll::Ready`,
  // false if the future returned `Poll::Pending`.
  //
  // `output` will contain the result of the Future iff `poll()` returns true.
  bool poll(const ::kj_rs::KjWaker& waker, ExceptionOrValue& output) noexcept {
    ::kj_rs::BoxFuturePoller<::kj::_::FixVoid<void>> poller;
    return poller.poll([this, &waker](void* result) {
      // Safety: `*this` is accepted as `Pin<&mut ...>` in the Rust implementation of
      // `box_future_poll()`. This is safe because it effectively implements Unpin, being
      // non-self-referential, so it's fine if we decide to move it later.
      return BoxFutureVoidInfallible_poll(*this, waker, result);
    }, output);
  }

  // Tell cxx-rs that this type follows Rust's move semantics, and can thus be passed across the FFI
  // boundary.
  using IsRelocatable = ::std::true_type;

private:
  // Match Rust's representation of a `Box<dyn Trait>`.
  ::std::array<std::uintptr_t, 2> repr;
};

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureVoidInfallible> operator co_await(BoxFutureVoidInfallible& future) {
  return ::kj::mv(future);
}

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureVoidInfallible> operator co_await(BoxFutureVoidInfallible&& future) {
  return ::kj::mv(future);
}

// ---------------------------------------------------------

class BoxFutureVoid;

extern "C" void BoxFutureVoid_drop_in_place(BoxFutureVoid* ptr);
extern "C" ::kj_rs::FuturePollStatus BoxFutureVoid_poll(BoxFutureVoid& future, const ::kj_rs::KjWaker& waker, void* result);

class BoxFutureVoid {
public:
  BoxFutureVoid(BoxFutureVoid&& other) noexcept: repr(other.repr) {
    other.repr = {0, 0};
  }
  ~BoxFutureVoid() noexcept {
    if (repr != ::std::array<::std::uintptr_t, 2>{0, 0}) {
      // Safety: We can assume that `this` is a valid pointer while we're in the destructor.
      BoxFutureVoid_drop_in_place(this);
    }
  }

  // We use the same output type for both fallible and infallible results.
  using ExceptionOrValue = ::kj::_::ExceptionOr<::kj::_::FixVoid<void>>;

  // Poll our Future with the given KjWaker. Returns true if the future returned `Poll::Ready`,
  // false if the future returned `Poll::Pending`.
  //
  // `output` will contain the result of the Future iff `poll()` returns true.
  bool poll(const ::kj_rs::KjWaker& waker, ExceptionOrValue& output) noexcept {
    ::kj_rs::BoxFuturePoller<::kj::_::FixVoid<void>> poller;
    return poller.poll([this, &waker](void* result) {
      // Safety: `*this` is accepted as `Pin<&mut ...>` in the Rust implementation of
      // `box_future_poll()`. This is safe because it effectively implements Unpin, being
      // non-self-referential, so it's fine if we decide to move it later.
      return BoxFutureVoid_poll(*this, waker, result);
    }, output);
  }

  // Tell cxx-rs that this type follows Rust's move semantics, and can thus be passed across the FFI
  // boundary.
  using IsRelocatable = ::std::true_type;

private:
  // Match Rust's representation of a `Box<dyn Trait>`.
  ::std::array<::std::uintptr_t, 2> repr;
};

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureVoid> operator co_await(BoxFutureVoid& future) {
  return ::kj::mv(future);
}

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureVoid> operator co_await(BoxFutureVoid&& future) {
  return ::kj::mv(future);
}

// ---------------------------------------------------------

class BoxFutureI32;

extern "C" void BoxFutureI32_drop_in_place(BoxFutureI32* ptr);
extern "C" ::kj_rs::FuturePollStatus BoxFutureI32_poll(BoxFutureI32& future, const ::kj_rs::KjWaker& waker, void* result);

class BoxFutureI32 {
public:
  BoxFutureI32(BoxFutureI32&& other) noexcept: repr(other.repr) {
    other.repr = {0, 0};
  }
  ~BoxFutureI32() noexcept {
    if (repr != ::std::array<::std::uintptr_t, 2>{0, 0}) {
      // Safety: We can assume that `this` is a valid pointer while we're in the destructor.
      BoxFutureI32_drop_in_place(this);
    }
  }

  // We use the same output type for both fallible and infallible results.
  using ExceptionOrValue = ::kj::_::ExceptionOr<::kj::_::FixVoid<int32_t>>;

  // Poll our Future with the given KjWaker. Returns true if the future returned `Poll::Ready`,
  // false if the future returned `Poll::Pending`.
  //
  // `output` will contain the result of the Future iff `poll()` returns true.
  bool poll(const ::kj_rs::KjWaker& waker, ExceptionOrValue& output) noexcept {
    ::kj_rs::BoxFuturePoller<::kj::_::FixVoid<int32_t>> poller;
    return poller.poll([this, &waker](void* result) {
      // Safety: `*this` is accepted as `Pin<&mut ...>` in the Rust implementation of
      // `box_future_poll()`. This is safe because it effectively implements Unpin, being
      // non-self-referential, so it's fine if we decide to move it later.
      return BoxFutureI32_poll(*this, waker, result);
    }, output);
  }

  // Tell cxx-rs that this type follows Rust's move semantics, and can thus be passed across the FFI
  // boundary.
  using IsRelocatable = ::std::true_type;

private:
  // Match Rust's representation of a `Box<dyn Trait>`.
  ::std::array<::std::uintptr_t, 2> repr;
};

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureI32> operator co_await(BoxFutureI32& future) {
  return ::kj::mv(future);
}

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureI32> operator co_await(BoxFutureI32&& future) {
  return ::kj::mv(future);
}

}  // namespace kj_rs_demo
