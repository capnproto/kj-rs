#include <kj-rs/awaiter.h>

#include <array>

namespace kj_rs::tests {

using kj_rs::BoxFutureFulfiller;
using kj_rs::Fallible;
using kj_rs::RemoveFallible;

template <typename T>
class BoxFuture;

// ---------------------------------------------------------

// Function templates which are explicitly specialized for each instance of BoxFuture<T>.
template <typename T>
void box_future_drop_in_place(BoxFuture<T>* self);
template <typename T>
bool box_future_poll(BoxFuture<T>& self, const kj_rs::KjWaker& waker, kj_rs::BoxFutureFulfiller<T>&);

// A `Pin<Box<dyn Future<Output = ()>>>` owned by C++.
//
// The only way to construct a BoxFuture<T> is by returning one from a Rust function.
template <typename T>
class BoxFuture {
public:
  BoxFuture(BoxFuture&& other) noexcept: repr(other.repr) {
    other.repr = {0, 0};
  }
  ~BoxFuture() noexcept {
    if (repr != std::array<std::uintptr_t, 2>{0, 0}) {
      // Safety: We can assume that `this` is a valid pointer while we're in the destructor.
      box_future_drop_in_place(this);
    }
  }

  // We use the same output type for both fallible and infallible results.
  using ExceptionOrValue = kj::_::ExceptionOr<kj::_::FixVoid<RemoveFallible<T>>>;

  // Poll our Future with the given KjWaker. Returns true if the future returned `Poll::Ready`,
  // false if the future returned `Poll::Pending`.
  //
  // `output` will contain the result of the Future iff `poll()` returns true.
  bool poll(const kj_rs::KjWaker& waker, ExceptionOrValue& output) noexcept {
    bool ready = false;

    KJ_IF_SOME(exception, kj::runCatchingExceptions([&]() {
      kj_rs::BoxFutureFulfiller<T> fulfiller(output);
      // Safety: Both `*this` and `fulfiller` are accepted as `Pin<&mut ...>` in the Rust
      // implementation of `box_future_pull()`. This is safe because both effectively implements
      // Unpin, since they are non-self-referential, so it's fine if we decide to move them later.
      ready = box_future_poll(*this, waker, fulfiller);
    })) {
      output.addException(kj::mv(exception));
      ready = true;
    }

    return ready;
  }

  // Tell cxx-rs that this type follows Rust's move semantics, and can thus be passed across the FFI
  // boundary.
  using IsRelocatable = std::true_type;

private:
  // Match Rust's representation of a `Box<dyn Trait>`.
  std::array<std::uintptr_t, 2> repr;
};

template <typename T>
kj_rs::LazyFutureAwaiter<BoxFuture<T>> operator co_await(BoxFuture<T>& future) {
  return kj::mv(future);
}

template <typename T>
kj_rs::LazyFutureAwaiter<BoxFuture<T>> operator co_await(BoxFuture<T>&& future) {
  return kj::mv(future);
}

// =======================================================================================
// Boilerplate follows

using BoxFutureVoid = BoxFuture<void>;

// We define this pointer typedef so that cxx-rs can associate it with the same pointer type our
// drop function uses.
using PtrBoxFutureVoid = BoxFutureVoid*;

using BoxFutureFulfillerVoid = BoxFutureFulfiller<void>;

// ---------------------------------------------------------

using BoxFutureFallibleVoid = BoxFuture<Fallible<void>>;

// We define this the pointer typedef so that cxx-rs can associate it with the same pointer type our
// drop function uses.
using PtrBoxFutureFallibleVoid = BoxFutureFallibleVoid*;

using BoxFutureFulfillerFallibleVoid = BoxFutureFulfiller<Fallible<void>>;

// ---------------------------------------------------------

using BoxFutureFallibleI32 = BoxFuture<Fallible<int32_t>>;

// We define this pointer typedef so that cxx-rs can associate it with the same pointer type our
// drop function uses.
using PtrBoxFutureFallibleI32 = BoxFutureFallibleI32*;

using BoxFutureFulfillerFallibleI32 = BoxFutureFulfiller<Fallible<int32_t>>;

}  // namespace kj_rs::tests
