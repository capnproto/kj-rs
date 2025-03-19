#pragma once

#include <kj-rs/waker.h>

#include <kj/debug.h>

#include <concepts>
#include <cstdint>

namespace kj_rs {

// Tri-state returned from `box_future_poll()`, indicating the state of its output parameter.
//
// Serves the same purpose as `cxx-async`'s FuturePollStatus:
// https://github.com/pcwalton/cxx-async/blob/ac98030dd6e5090d227e7fadca13ec3e4b4e7be7/cxx-async/include/rust/cxx_async.h#L422
enum class FuturePollStatus: uint8_t {
  // `box_future_poll()` returns Pending to indicate it did not write anything to its output
  // parameter.
  Pending,
  // `box_future_poll()` returns Complete to indicate it wrote a value to its output
  // parameter.
  Complete,
  // `box_future_poll()` returns Error to indicate it wrote an error to its output parameter.
  Error,
};

// A class with space for a `T` or a `rust::String`, whichever is larger.
template <typename T>
class BoxFuturePoller {
public:
  BoxFuturePoller() {}
  ~BoxFuturePoller() noexcept(false) {}

  // Call `pollFunc()` with a pointer to space to which a `T` (successful result) or a
  // `rust::String` (error result) may be written, then propagate the result or error to `output`
  // depending on the return value of `pollFunc()`.
  template <typename F>
  bool poll(F&& pollFunc, kj::_::ExceptionOr<T>& output) {
    switch (pollFunc(&result)) {
      case ::kj_rs::FuturePollStatus::Pending:
        return false;
      case ::kj_rs::FuturePollStatus::Complete:
        output.value = toResult();
        return true;
      case ::kj_rs::FuturePollStatus::Error: {
        output.addException(toException());
        return true;
      }
    }

    KJ_UNREACHABLE;
  }

private:
  T toResult() {
    auto ret = kj::mv(result);
    kj::dtor(result);
    return ret;
  }

  kj::Exception toException() {
    auto description = ::kj::ArrayPtr<const char>(error.data(), error.size());
    auto exception = KJ_EXCEPTION(FAILED, kj::str(description));
    kj::dtor(error);
    return exception;
  }

  union {
    T result;
    ::rust::String error;
  };
};

template <typename F>
concept Future = requires(F f) {
  typename F::ExceptionOrValue;
  { f.poll(kj::instance<const KjWaker&>(), kj::instance<typename F::ExceptionOrValue&>()) } -> std::same_as<bool>;
};

}  // namespace kj_rs
