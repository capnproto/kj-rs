#pragma once

#include <kj-rs/future.h>
#include <kj-rs/waker.h>
#include <kj-rs/executor-guarded.h>
#include <kj-rs/linked-group.h>

#include <kj/debug.h>

#include <array>  // We use it in the implementation of KJRS_DEFINE_FUTURE().

namespace kj_rs {

// =======================================================================================
// Opaque Rust types
//
// The following types are defined in lib.rs, and thus in lib.rs.h. lib.rs.h depends on our C++
// headers, including awaiter.h (the file you're currently reading), so we forward-declare some types
// here for use in the C++ headers.

// Wrapper around an `&std::task::Waker`, passed to `RustPromiseAwaiter::poll()`. This indirection
// is required because cxx-rs does not permit us to expose opaque Rust types to C++ defined outside
// of our own crate, like `std::task::Waker`.
struct WakerRef;

// Wrapper around an `Option<std::task::Waker>`. RustPromiseAwaiter calls `set()` with the WakerRef
// passed to `poll()` if RustPromiseAwaiter is unable to find an optimized path for awaiting its
// Promise. Later on, when its Promise becomes ready, RustPromiseAwaiter will use OptionWaker to
// call wake the wrapped Waker.
//
// Otherwise, if RustPromiseAwaiter finds an optimized path for awaiting its Promise, it calls
// `set_none()` on the OptionWaker to ensure it's empty.
struct OptionWaker;

// =======================================================================================
// RustPromiseAwaiter

// RustPromiseAwaiter allows Rust `async` blocks to `.await` KJ promises. Rust code creates one in
// the block's storage at the point where the `.await` expression is evaluated, similar to how
// `kj::_::PromiseAwaiter` is created in the KJ coroutine frame when C++ `co_await`s a promise.
//
// To elaborate, RustPromiseAwaiter is part of the IntoFuture trait implementation for the
// OwnPromiseNode class, and `.await` expressions implicitly call `.into_future()`. So,
// RustPromiseAwaiter can be thought of a "Promise-to-Future" adapter. This also means that
// RustPromiseAwaiter can be constructed outside of `.await` expressions, and potentially _not_
// driven to complete readiness. Our implementation must be able to handle this case.
//
// Rust knows how big RustPromiseAwaiter is because we generate a Rust type of equal size and
// alignment using bindgen. See inside awaiter.c++ for a static_assert to remind us to re-run
// bindgen.
//
// RustPromiseAwaiter has two base classes: KJ Event, and a LinkedObject template instantiation. We
// use the Event to discover when our wrapped Promise is ready. Our Event fire() implementation
// records the fact that we are done, then wakes our Waker or arms the FuturePollEvent, if we
// have one. We access the FuturePollEvent via our LinkedObject base class mixin. It gives us the
// ability to store a weak reference to the FuturePollEvent, if we were last polled by one.
class RustPromiseAwaiter final: public kj::_::Event,
                                public LinkedObject<FuturePollEvent, RustPromiseAwaiter> {
public:
  // The Rust code which constructs RustPromiseAwaiter passes us a pointer to a OptionWaker, which can
  // be thought of as a Rust-native component RustPromiseAwaiter. Its job is to hold a clone of
  // of any non-KJ Waker that we are polled with, and forward calls to `wake()`. Ideally, we could
  // store the clone of the Waker ourselves (it's just two pointers) on the C++ side, so the
  // lifetime safety is more obvious. But, storing a reference works for now.
  RustPromiseAwaiter(OptionWaker& optionWaker, OwnPromiseNode node, kj::SourceLocation location = {});
  ~RustPromiseAwaiter() noexcept(false);
  KJ_DISALLOW_COPY_AND_MOVE(RustPromiseAwaiter);

  // -------------------------------------------------------
  // kj::_::Event API

  kj::Maybe<kj::Own<kj::_::Event>> fire() override;
  void traceEvent(kj::_::TraceBuilder& builder) override;

  // Helper for FuturePollEvent to report what promise it's waiting on.
  void tracePromise(kj::_::TraceBuilder& builder, bool stopAtNextEvent);

  // -------------------------------------------------------
  // API exposed to Rust code
  //
  // Additionally, see GuardedRustPromiseAwaiter below, which mediates access to this API.

  // Poll this Promise for readiness.
  //
  // If the Waker is a KjWaker, you may pass the KjWaker pointer as a second parameter. This may
  // allow the implementation of `poll()` to optimize the wake by arming a KJ Event directly when
  // the wrapped Promise becomes ready.
  //
  // If the Waker is not a KjWaker, the `maybeKjWaker` pointer argument must be nullptr.
  bool poll(const WakerRef& waker, const KjWaker* maybeKjWaker);

  // Release ownership of the OwnPromiseNode. Asserts if called before the Promise is ready; that
  // is, `poll()` must have returned true prior to calling `take_own_promise_node()`.
  OwnPromiseNode take_own_promise_node();

private:
  // The Rust code which instantiates RustPromiseAwaiter does so with a OptionWaker object right
  // next to the RustPromiseAwaiter, such that it is dropped after RustPromiseAwaiter. Thus, our
  // reference to our OptionWaker is stable. We use the OptionWaker to (optionally) store a clone of
  // the Waker with which we were last polled.
  //
  // When we wake our enclosing Future, either with the FuturePollEvent or with OptionWaker, we
  // nullify this Maybe. Therefore, this Maybe being kj::none means our OwnPromiseNode is ready, and
  // it is safe to call `node->get()` on it.
  kj::Maybe<OptionWaker&> maybeOptionWaker;

  kj::UnwindDetector unwindDetector;
  OwnPromiseNode node;
};

// We force Rust to call our `poll()` overloads using this ExecutorGuarded wrapper around the actual
// RustPromiseAwaiter class. This allows us to assume all calls that reach RustPromiseAwaiter itself
// are on the correct thread.
struct GuardedRustPromiseAwaiter: ExecutorGuarded<RustPromiseAwaiter> {
  // We need to inherit constructors or else placement-new will try to aggregate-initialize us.
  using ExecutorGuarded<RustPromiseAwaiter>::ExecutorGuarded;

  bool poll(const WakerRef& waker, const KjWaker* maybeKjWaker) {
    return get().poll(waker, maybeKjWaker);
  }
  OwnPromiseNode take_own_promise_node() {
    return get().take_own_promise_node();
  }
};

using PtrGuardedRustPromiseAwaiter = GuardedRustPromiseAwaiter*;

void guarded_rust_promise_awaiter_new_in_place(
    PtrGuardedRustPromiseAwaiter, OptionWaker*, OwnPromiseNode);
void guarded_rust_promise_awaiter_drop_in_place(PtrGuardedRustPromiseAwaiter);

// =======================================================================================
// FuturePollEvent

// Base class for `FutureAwaiter<F>`. `FutureAwaiter<F>` implements the type-specific
// `Event::fire()` override which actually polls the Future; this class implements all other base
// class virtual functions.
//
// A FuturePollEvent contains an optional ArcWakerPromiseAwaiter and a list of zero or more
// RustPromiseAwaiters. These "sub-Promise awaiters" all wrap a KJ Promise of some sort, and arrange
// to arm the FuturePollEvent when their Promises become ready.
//
// The PromiseNode base class is a hack to implement async tracing. That is, we only implement the
// `tracePromise()` function, and decide which Promise to trace into if/when the coroutine calls our
// `tracePromise()` implementation. This primarily makes the lifetimes easier to manage: our
// RustPromiseAwaiter LinkedObjects have independent lifetimes from the FuturePollEvent, so we
// mustn't leave references to them, or their members, lying around in the Coroutine class.
class FuturePollEvent: public kj::_::Event,
                       public kj::_::PromiseNode,
                       public LinkedGroup<FuturePollEvent, RustPromiseAwaiter> {
public:
  FuturePollEvent(kj::SourceLocation location = {}): Event(location) {}

  // -------------------------------------------------------
  // PromiseNode API
  //
  // HACK: We only implement this interface for `tracePromise()`, which is the only function
  // CoroutineBase uses on its `promiseNodeForTrace` reference.

  void destroy() override {}  // No-op because we are allocated inside the coroutine frame
  void onReady(kj::_::Event* event) noexcept override;
  void get(kj::_::ExceptionOrValue& output) noexcept override;
  void tracePromise(kj::_::TraceBuilder& builder, bool stopAtNextEvent) override;

protected:
  // PollScope is a LazyArcWaker which is associated with a specific FuturePollEvent, allowing
  // optimized Promise `.await`s. Additionally, PollScope's destructor arranges to await any
  // ArcWaker promise which was lazily created.
  //
  // Used by FutureAwaiter<T>, our derived class.
  class PollScope;

private:
  // Private API for PollScope.
  void enterPollScope() noexcept;
  void exitPollScope(kj::Maybe<kj::Promise<void>> maybeLazyArcWakerPromise);

  kj::Maybe<OwnPromiseNode> arcWakerPromise;
};

class FuturePollEvent::PollScope: public LazyArcWaker {
public:
  // `futurePollEvent` is the FuturePollEvent responsible for calling `Future::poll()`, and must
  // outlive this PollScope.
  PollScope(FuturePollEvent& futurePollEvent);
  ~PollScope() noexcept(false);
  KJ_DISALLOW_COPY_AND_MOVE(PollScope);

  // The Event which is using this PollScope to poll() a Future. Waking this FuturePollEvent's
  // PollScope arms this Event (possibly via a cross-thread promise fulfiller). We also arm the
  // Event directly in the RustPromiseAwaiter class, to more optimally `.await` KJ Promises from
  // within Rust. If the current thread's kj::Executor is not the same as the one which owns the
  // FuturePollEvent, this function returns kj::none.
  kj::Maybe<FuturePollEvent&> tryGetFuturePollEvent() const override;

private:
  struct FuturePollEventHolder {
    FuturePollEvent& futurePollEvent;
  };
  ExecutorGuarded<FuturePollEventHolder> holder;
};

// =======================================================================================
// FutureAwaiter, LazyFutureAwaiter, and operator co_await implementations

// FutureAwaiter<T> is a Future poll() Event, and is the inner implementation of our co_await
// syntax. It wraps a Future and captures a reference to its enclosing KJ coroutine, arranging
// to continuously call `Future::poll()` on the KJ event loop until the Future produces a
// result, after which it arms the enclosing KJ coroutine's Event.
template <Future F>
class FutureAwaiter final: public FuturePollEvent {
public:
  FutureAwaiter(
      kj::_::CoroutineBase& coroutine,
      F future,
      kj::SourceLocation location = {})
      : FuturePollEvent(location),
        coroutine(coroutine),
        future(kj::mv(future)) {}
  ~FutureAwaiter() noexcept(false) {
    coroutine.clearPromiseNodeForTrace();
  }
  KJ_DISALLOW_COPY_AND_MOVE(FutureAwaiter);

  // Poll the wrapped Future, returning false if we should _not_ suspend, true if we should suspend.
  bool awaitSuspendImpl() {
    // TODO(perf): Check if we already have an ArcWaker from a previous suspension and give it to
    //   LazyArcWaker for cloning if we have the last reference to it at this point. This could save
    //   memory allocations, but would depend on making XThreadFulfiller and XThreadPaf resettable
    //   to really benefit.

    {
      PollScope pollScope(*this);

      if (future.poll(pollScope, result)) {
        // Future is ready, we're done.
        return false;
      }
    }

    // Integrate with our enclosing coroutine's tracing.
    coroutine.setPromiseNodeForTrace(promiseNodeForTrace);

    return true;
  }

  auto awaitResumeImpl() {
    coroutine.clearPromiseNodeForTrace();
    return kj::_::convertToReturn(kj::mv(result));
  }

  // -------------------------------------------------------
  // Event API

  void traceEvent(kj::_::TraceBuilder& builder) override {
    // Just defer to our enclosing Coroutine. It will immediately call our CoAwaitWaker's
    // `tracePromise()` implementation.
    static_cast<Event&>(coroutine).traceEvent(builder);
  }

private:
  kj::Maybe<kj::Own<kj::_::Event>> fire() override {
    if (!awaitSuspendImpl()) {
      coroutine.armDepthFirst();
    }
    return kj::none;
  }

  kj::_::CoroutineBase& coroutine;
  // HACK: FuturePollEvent implements the PromiseNode interface to integrate with the Coroutine
  // class' current tracing implementation.
  OwnPromiseNode promiseNodeForTrace { this };
  typename F::ExceptionOrValue result;
  F future;
};

// LazyFutureAwaiter<T> is the outer implementation of our co_await syntax, providing the
// await_ready(), await_suspend(), await_resume() facade expected by the compiler.
//
// LazyFutureAwaiter is a type with two stages. At first, it merely wraps a Future. Once
// its await_suspend() function is called, it transitions to wrap a FutureAwaiter<T>, our inner
// awaiter implementation. We do this because we don't get a reference to our enclosing
// coroutine until await_suspend() is called, and our awaiter implementation is greatly simplified
// if we can avoid using a Maybe. So, we defer the real awaiter instantiation to await_suspend().
template <Future F>
class LazyFutureAwaiter {
public:
  LazyFutureAwaiter(F&& future): impl(kj::mv(future)) {}

  // Always return false, so our await_suspend() is guaranteed to be called.
  bool await_ready() const { return false; }

  // Initialize our wrapped Awaiter and forward to `FutureAwaiter<T>::awaitSuspendImpl()`.
  template <typename U> requires (kj::canConvert<U&, kj::_::CoroutineBase&>())
  bool await_suspend(kj::_::stdcoro::coroutine_handle<U> handle) {
    auto future = kj::mv(KJ_ASSERT_NONNULL(impl.template tryGet<F>()));
    return impl.template init<FutureAwaiter<F>>(handle.promise(), kj::mv(future))
        .awaitSuspendImpl();
  }

  // Forward to our wrapped `FutureAwaiter<T>::awaitResumeImpl()`.
  auto await_resume() {
    return KJ_ASSERT_NONNULL(impl.template tryGet<FutureAwaiter<F>>()).awaitResumeImpl();
  }


private:
  kj::OneOf<F, FutureAwaiter<F>> impl;
};

}  // namespace kj_rs

// The following file was originally based on cxx-async/include/rust/cxx_async.h from the `cxx-async` crate,
// which is subject to the following copyright:
//
// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// https://github.com/pcwalton/cxx-async
//
// The `cxx-async` crate is dual-licensed under both the MIT and Apache 2.0 licenses. You will find
// a copy of one of those licenses in the file named LICENSE in this repository's root directory.
//
// Subsequent changes are subject to the following copyright:
//
// Copyright (c) 2025 Cloudflare, Inc.

// Warning! Preprocessor abuse follows!

// This is a hack to do variadic arguments in macros.
// See: https://stackoverflow.com/a/3048361

#define KJRS_JOIN_NAMESPACE_1(a0) a0
#define KJRS_JOIN_NAMESPACE_2(a0, a1) a0::a1
#define KJRS_JOIN_NAMESPACE_3(a0, a1, a2) a0::a1::a2
#define KJRS_JOIN_NAMESPACE_4(a0, a1, a2, a3) a0::a1::a2::a3
#define KJRS_JOIN_NAMESPACE_5(a0, a1, a2, a3, a4) a0::a1::a2::a3::a4
#define KJRS_JOIN_NAMESPACE_6(a0, a1, a2, a3, a4, a5) a0::a1::a2::a3::a4::a5
#define KJRS_JOIN_NAMESPACE_7(a0, a1, a2, a3, a4, a5, a6) \
  a0::a1::a2::a3::a4::a5::a6
#define KJRS_JOIN_NAMESPACE_8(a0, a1, a2, a3, a4, a5, a6, a7) \
  a0::a1::a2::a3::a4::a5::a6::a7

#define KJRS_JOIN_DOLLAR_1(a0) a0
#define KJRS_JOIN_DOLLAR_2(a0, a1) a0##$##a1
#define KJRS_JOIN_DOLLAR_3(a0, a1, a2) a0##$##a1##$##a2
#define KJRS_JOIN_DOLLAR_4(a0, a1, a2, a3) a0##$##a1##$##a2##$##a3
#define KJRS_JOIN_DOLLAR_5(a0, a1, a2, a3, a4) \
  a0##$##a1##$##a2##$##a3##$##a4
#define KJRS_JOIN_DOLLAR_6(a0, a1, a2, a3, a4, a5) \
  a0##$##a1##$##a2##$##a3##$##a4##$##a5
#define KJRS_JOIN_DOLLAR_7(a0, a1, a2, a3, a4, a5, a6) \
  a0##$##a1##$##a2##$##a3##$##a4##$##a5##$##a6
#define KJRS_JOIN_DOLLAR_8(a0, a1, a2, a3, a4, a5, a6, a7) \
  a0##$##a1##$##a2##$##a3##$##a4##$##a5##$##a6##$##a7

#define KJRS_STRIP_NAMESPACE_1(a0) a0
#define KJRS_STRIP_NAMESPACE_2(a0, a1) a1
#define KJRS_STRIP_NAMESPACE_3(a0, a1, a2) a2
#define KJRS_STRIP_NAMESPACE_4(a0, a1, a2, a3) a3
#define KJRS_STRIP_NAMESPACE_5(a0, a1, a2, a3, a4) a4
#define KJRS_STRIP_NAMESPACE_6(a0, a1, a2, a3, a4, a5) a5
#define KJRS_STRIP_NAMESPACE_7(a0, a1, a2, a3, a4, a5, a6) a6
#define KJRS_STRIP_NAMESPACE_8(a0, a1, a2, a3, a4, a5, a6, a7) a7

#define KJRS_OPEN_NAMESPACE_1(a0)
#define KJRS_OPEN_NAMESPACE_2(a0, a1) namespace a0 {
#define KJRS_OPEN_NAMESPACE_3(a0, a1, a2) namespace a0::a1 {
#define KJRS_OPEN_NAMESPACE_4(a0, a1, a2, a3) namespace a0::a1::a2 {
#define KJRS_OPEN_NAMESPACE_5(a0, a1, a2, a3, a4) namespace a0::a1::a2::a3 {
#define KJRS_OPEN_NAMESPACE_6(a0, a1, a2, a3, a4, a5) \
  namespace a0::a1::a2::a3::a4 {
#define KJRS_OPEN_NAMESPACE_7(a0, a1, a2, a3, a4, a5, a6) \
  namespace a0::a1::a2::a3::a4::a5 {
#define KJRS_OPEN_NAMESPACE_8(a0, a1, a2, a3, a4, a5, a6, a7) \
  namespace a0::a1::a2::a3::a4::a5::a6 {

#define KJRS_CLOSE_NAMESPACE_1(a0)
#define KJRS_CLOSE_NAMESPACE_2(a0, a1) }
#define KJRS_CLOSE_NAMESPACE_3(a0, a1, a2) }
#define KJRS_CLOSE_NAMESPACE_4(a0, a1, a2, a3) }
#define KJRS_CLOSE_NAMESPACE_5(a0, a1, a2, a3, a4) }
#define KJRS_CLOSE_NAMESPACE_6(a0, a1, a2, a3, a4, a5) }
#define KJRS_CLOSE_NAMESPACE_7(a0, a1, a2, a3, a4, a5, a6) }
#define KJRS_CLOSE_NAMESPACE_8(a0, a1, a2, a3, a4, a5, a6, a7) }

// Need a level of indirection here because of
// https://stackoverflow.com/a/1489985
#define KJRS_CONCAT_3_IMPL(a, b, c) a##b##c
#define KJRS_CONCAT_3(a, b, c) KJRS_CONCAT_3_IMPL(a, b, c)

// Concatenates `prefix` to the number of variadic arguments supplied (e.g.
// `prefix_1`, `prefix_2`, etc.)
#define KJRS_DISPATCH_VARIADIC_IMPL(                 \
    prefix, unused, a0, a1, a2, a3, a4, a5, a6, a7, ...) \
  prefix##a7
#define KJRS_DISPATCH_VARIADIC(prefix, ...) \
  KJRS_DISPATCH_VARIADIC_IMPL(prefix, __VA_ARGS__, 8, 7, 6, 5, 4, 3, 2, 1, )

#define KJRS_DISPATCH_OPTIONAL_IMPL(prefix, unused, a0, a1, ...) prefix##a1
#define KJRS_DISPATCH_OPTIONAL(prefix, ...) \
  KJRS_DISPATCH_OPTIONAL_IMPL(prefix, __VA_ARGS__, 1, 0, )

#define KJRS_JOIN_NAMESPACE(...) \
  KJRS_DISPATCH_VARIADIC(KJRS_JOIN_NAMESPACE_, __VA_ARGS__)(__VA_ARGS__)
#define KJRS_JOIN_DOLLAR(...) \
  KJRS_DISPATCH_VARIADIC(KJRS_JOIN_DOLLAR_, __VA_ARGS__)(__VA_ARGS__)
#define KJRS_STRIP_NAMESPACE(...)                                \
  KJRS_DISPATCH_VARIADIC(KJRS_STRIP_NAMESPACE_, __VA_ARGS__) \
  (__VA_ARGS__)
#define KJRS_OPEN_NAMESPACE(...) \
  KJRS_DISPATCH_VARIADIC(KJRS_OPEN_NAMESPACE_, __VA_ARGS__)(__VA_ARGS__)
#define KJRS_CLOSE_NAMESPACE(...)                                \
  KJRS_DISPATCH_VARIADIC(KJRS_CLOSE_NAMESPACE_, __VA_ARGS__) \
  (__VA_ARGS__)

// Define the C++ type corresponding to a bridged Rust Future. That is, for every Rust Future type
// which you have defined like so:
//
// ```
// #[kj_rs::bridge_future(namespace = demo)]
// unsafe impl Future for MyFuture {
//     type Output = Result<T, _>;
// }
// #[cxx::bridge(namespace = "demo")]
// mod ffi {
//     unsafe extern "C++" {
//         include!("your-header.h");
//         type MyFuture = crate::MyFuture;
//     }
// }
// ```
//
// There must be a corresponding C++ header file named "your-header.h" containing:
//
// ```
// #include <kj-rs/awaiter.h>
// KJRS_DEFINE_FUTURE(T, demo, MyFuture);
// ```
#define KJRS_DEFINE_FUTURE(type, ...)      \
  KJRS_OPEN_NAMESPACE(__VA_ARGS__)                                      \
  class KJRS_STRIP_NAMESPACE(__VA_ARGS__);                             \
  extern "C" void KJRS_CONCAT_3( \
      kjrs_, KJRS_JOIN_DOLLAR(__VA_ARGS__), _drop_in_place \
    )(KJRS_STRIP_NAMESPACE(__VA_ARGS__)* ptr); \
  extern "C" ::kj_rs::FuturePollStatus KJRS_CONCAT_3( \
      kjrs_, KJRS_JOIN_DOLLAR(__VA_ARGS__), _poll \
    )(KJRS_STRIP_NAMESPACE(__VA_ARGS__)& future, const ::kj_rs::KjWaker& waker, void* result); \
  class KJRS_STRIP_NAMESPACE(__VA_ARGS__) { \
  public: \
    KJRS_STRIP_NAMESPACE(__VA_ARGS__)(KJRS_STRIP_NAMESPACE(__VA_ARGS__)&& other) noexcept: repr(other.repr) { \
      other.repr = {0, 0}; \
    } \
    ~KJRS_STRIP_NAMESPACE(__VA_ARGS__)() noexcept { \
      if (repr != std::array<std::uintptr_t, 2>{0, 0}) { \
        KJRS_CONCAT_3( \
          kjrs_, KJRS_JOIN_DOLLAR(__VA_ARGS__), _drop_in_place \
        )(this); \
      } \
    } \
    \
    using ExceptionOrValue = ::kj::_::ExceptionOr<::kj::_::FixVoid<type>>; \
    bool poll(const ::kj_rs::KjWaker& waker, ExceptionOrValue& output) noexcept { \
      ::kj_rs::BoxFuturePoller<::kj::_::FixVoid<type>> poller; \
      return poller.poll([this, &waker](void* result) { \
        return KJRS_CONCAT_3( \
          kjrs_, KJRS_JOIN_DOLLAR(__VA_ARGS__), _poll \
        )(*this, waker, result); \
      }, output); \
    } \
    using IsRelocatable = ::std::true_type; \
  private: \
    ::std::array<std::uintptr_t, 2> repr; \
  }; \
  static inline ::kj_rs::LazyFutureAwaiter<KJRS_STRIP_NAMESPACE(__VA_ARGS__)> \
  operator co_await(KJRS_STRIP_NAMESPACE(__VA_ARGS__)& future) { \
    return ::kj::mv(future); \
  } \
  static inline ::kj_rs::LazyFutureAwaiter<KJRS_STRIP_NAMESPACE(__VA_ARGS__)> \
  operator co_await(KJRS_STRIP_NAMESPACE(__VA_ARGS__)&& future) { \
    return ::kj::mv(future); \
  } \
  KJRS_CLOSE_NAMESPACE(__VA_ARGS__)
