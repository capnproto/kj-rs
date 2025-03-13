#pragma once

#include <kj-rs/waker.h>

#include <concepts>

namespace kj_rs {

// Corresponds to Result on the Rust side. Does not currently wrap an exception, though maybe it
// should.
template <typename T>
class Fallible {
public:
  template <typename... U>
  Fallible(U&&... args): value(kj::fwd<U>(args)...) {}
  operator T&() { return value; }
private:
  T value;
};

template <>
class Fallible<void> {};


template <typename T>
struct RemoveFallible_ {
  using Type = T;
};
template <typename T>
struct RemoveFallible_<Fallible<T>> {
  using Type = T;
};
template <typename T>
using RemoveFallible = typename RemoveFallible_<T>::Type;

template <typename T>
class BoxFutureFulfiller {
public:
  BoxFutureFulfiller(kj::_::ExceptionOr<RemoveFallible<T>>& resultRef): result(resultRef) {}
  void fulfill(RemoveFallible<T> value) { result.value = kj::mv(value); }
private:
  kj::_::ExceptionOr<RemoveFallible<T>>& result;
};

template <>
class BoxFutureFulfiller<void> {
public:
  BoxFutureFulfiller(kj::_::ExceptionOr<kj::_::Void>& resultRef): result(resultRef) {}
  void fulfill(kj::_::Void value) { result.value = kj::mv(value); }
  // For Rust, which doesn't know about our kj::_::Void type.
  void fulfill() { fulfill({}); }
private:
  kj::_::ExceptionOr<kj::_::Void>& result;
};

template <>
class BoxFutureFulfiller<Fallible<void>>: public BoxFutureFulfiller<void> {};

template <typename F>
concept Future = requires(F f) {
  typename F::ExceptionOrValue;
  { f.poll(kj::instance<const KjWaker&>(), kj::instance<typename F::ExceptionOrValue&>()) } -> std::same_as<bool>;
};

}  // namespace kj_rs
