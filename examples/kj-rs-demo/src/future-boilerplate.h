#include <kj-rs/awaiter.h>

#include <array>

namespace kj_rs_demo {

class BoxFutureVoid;

extern "C" void box_future_drop_in_place_void(BoxFutureVoid* ptr);
extern "C" ::kj_rs::FuturePollStatus box_future_poll_void(BoxFutureVoid& future, const ::kj_rs::KjWaker& waker, void* result);

class BoxFutureVoid {
public:
  BoxFutureVoid(BoxFutureVoid&& other) noexcept: repr(other.repr) {
    other.repr = {0, 0};
  }
  ~BoxFutureVoid() noexcept {
    if (repr != std::array<std::uintptr_t, 2>{0, 0}) {
      // Safety: We can assume that `this` is a valid pointer while we're in the destructor.
      box_future_drop_in_place_void(this);
    }
  }

  using ExceptionOrValue = ::kj::_::ExceptionOr<::kj::_::FixVoid<void>>;

  // Poll our Future with the given KjWaker. Returns true if the future returned `Poll::Ready`,
  // false if the future returned `Poll::Pending`.
  //
  // `output` will contain the result of the Future iff `poll()` returns true.
  bool poll(const ::kj_rs::KjWaker& waker, ExceptionOrValue& output) noexcept {
    BoxFuturePoller<::kj::_::FixVoid<void>> poller;
    return poller.poll([this, &waker](void* result) {
      // Safety: `*this` is accepted as `Pin<&mut ...>` in the Rust implementation of
      // `box_future_poll()`. This is safe because it effectively implements Unpin, being
      // non-self-referential, so it's fine if we decide to move it later.
      return box_future_poll_void(*this, waker, result);
    }, output);
  }

  // Tell cxx-rs that this type follows Rust's move semantics, and can thus be passed across the FFI
  // boundary.
  using IsRelocatable = ::std::true_type;

private:
  // Match Rust's representation of a `Box<dyn Trait>`.
  ::std::array<std::uintptr_t, 2> repr;
};

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureVoid> operator co_await(BoxFutureVoid& future) {
  return ::kj::mv(future);
}

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureVoid> operator co_await(BoxFutureVoid&& future) {
  return ::kj::mv(future);
}

// ---------------------------------------------------------

class BoxFutureFallibleVoid;

extern "C" void box_future_drop_in_place_fallible_void(BoxFutureFallibleVoid* ptr);
extern "C" ::kj_rs::FuturePollStatus box_future_poll_fallible_void(BoxFutureFallibleVoid& future, const KjWaker& waker, void* result);

class BoxFutureFallibleVoid {
public:
  BoxFutureFallibleVoid(BoxFutureFallibleVoid&& other) noexcept: repr(other.repr) {
    other.repr = {0, 0};
  }
  ~BoxFutureFallibleVoid() noexcept {
    if (repr != ::std::array<::std::uintptr_t, 2>{0, 0}) {
      // Safety: We can assume that `this` is a valid pointer while we're in the destructor.
      box_future_drop_in_place_fallible_void(this);
    }
  }

  // We use the same output type for both fallible and infallible results.
  using ExceptionOrValue = ::kj::_::ExceptionOr<::kj::_::FixVoid<void>>;

  // Poll our Future with the given KjWaker. Returns true if the future returned `Poll::Ready`,
  // false if the future returned `Poll::Pending`.
  //
  // `output` will contain the result of the Future iff `poll()` returns true.
  bool poll(const ::kj_rs::KjWaker& waker, ExceptionOrValue& output) noexcept {
    BoxFuturePoller<::kj::_::FixVoid<void>> poller;
    return poller.poll([this, &waker](void* result) {
      // Safety: `*this` is accepted as `Pin<&mut ...>` in the Rust implementation of
      // `box_future_poll()`. This is safe because it effectively implements Unpin, being
      // non-self-referential, so it's fine if we decide to move it later.
      return box_future_poll_fallible_void(*this, waker, result);
    }, output);
  }

  // Tell cxx-rs that this type follows Rust's move semantics, and can thus be passed across the FFI
  // boundary.
  using IsRelocatable = ::std::true_type;

private:
  // Match Rust's representation of a `Box<dyn Trait>`.
  ::std::array<::std::uintptr_t, 2> repr;
};

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureFallibleVoid> operator co_await(BoxFutureFallibleVoid& future) {
  return ::kj::mv(future);
}

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureFallibleVoid> operator co_await(BoxFutureFallibleVoid&& future) {
  return ::kj::mv(future);
}

// ---------------------------------------------------------

class BoxFutureFallibleI32;

extern "C" void box_future_drop_in_place_fallible_i32(BoxFutureFallibleI32* ptr);
extern "C" ::kj_rs::FuturePollStatus box_future_poll_fallible_i32(BoxFutureFallibleI32& future, const KjWaker& waker, void* result);

class BoxFutureFallibleI32 {
public:
  BoxFutureFallibleI32(BoxFutureFallibleI32&& other) noexcept: repr(other.repr) {
    other.repr = {0, 0};
  }
  ~BoxFutureFallibleI32() noexcept {
    if (repr != ::std::array<::std::uintptr_t, 2>{0, 0}) {
      // Safety: We can assume that `this` is a valid pointer while we're in the destructor.
      box_future_drop_in_place_fallible_i32(this);
    }
  }

  // We use the same output type for both fallible and infallible results.
  using ExceptionOrValue = ::kj::_::ExceptionOr<::kj::_::FixVoid<int32_t>>;

  // Poll our Future with the given KjWaker. Returns true if the future returned `Poll::Ready`,
  // false if the future returned `Poll::Pending`.
  //
  // `output` will contain the result of the Future iff `poll()` returns true.
  bool poll(const ::kj_rs::KjWaker& waker, ExceptionOrValue& output) noexcept {
    BoxFuturePoller<::kj::_::FixVoid<int32_t>> poller;
    return poller.poll([this, &waker](void* result) {
      // Safety: `*this` is accepted as `Pin<&mut ...>` in the Rust implementation of
      // `box_future_poll()`. This is safe because it effectively implements Unpin, being
      // non-self-referential, so it's fine if we decide to move it later.
      return box_future_poll_fallible_i32(*this, waker, result);
    }, output);
  }

  // Tell cxx-rs that this type follows Rust's move semantics, and can thus be passed across the FFI
  // boundary.
  using IsRelocatable = ::std::true_type;

private:
  // Match Rust's representation of a `Box<dyn Trait>`.
  ::std::array<::std::uintptr_t, 2> repr;
};

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureFallibleI32> operator co_await(BoxFutureFallibleI32& future) {
  return ::kj::mv(future);
}

static inline ::kj_rs::LazyFutureAwaiter<BoxFutureFallibleI32> operator co_await(BoxFutureFallibleI32&& future) {
  return ::kj::mv(future);
}

}  // namespace kj_rs_demo
