// This file was originally based on cxx-async/macro/src/lib.rs from the `cxx-async` crate, which
// is subject to the following copyright:
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

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::Error as SynError;
use syn::Ident;
use syn::ImplItem;
use syn::ItemImpl;
use syn::Lit;
use syn::LitStr;
use syn::Path;
use syn::Result as SynResult;
use syn::Token;
use syn::Type;
use syn::TypePath;
use syn::Visibility;

/// Defines a future type that can be awaited from both Rust and C++.
///
/// Invoke this macro like so:
///
/// ```ignore
/// #[kj_rs::bridge_future]
/// unsafe impl Future for RustFutureString {
///     type Output = Result<String, Infallible>;
/// }
/// ```
///
/// This expands to a concrete definition of `RustFutureString` (a wrapper around a `BoxFuture<T>`)
/// along with implementations of its `Future` and `Drop` traits. Some additional FFI glue is
/// generated to allow passing it across cxx-rs-defined interfaces, and to allow C++ to poll it.
///
/// The associated `Output` type must be a `Result<T, E>`, where `T: Unpin` and `E: Error`. The
/// result type `T` will be converted to the appropriate type in C++, following `cxx-rs` rules. The
/// example above demonstrates using the `Infallible` `Error` type, from the `std::convert` module,
/// to signify that there is no code path which can fail. For `Output` types which _can_ fail, the
/// error is converted to a C++ exception with `to_string()`.
///
/// To use this new type in `cxx::bridge` FFI modules, declare it in an `extern "C++" { ... }` block
/// like so:
///
/// ```ignore
/// #[cxx::bridge]
/// mod ffi {
///     unsafe extern "C++" {
///         include!("futures.h");
///         type RustFutureString = crate::RustFutureString;
///     }
/// }
/// ```
///
/// The "futures.h" file must exist, and contain something like the following:
///
/// ```ignore
/// #include <kj-rs/awaiter.h>
/// #include <rust/cxx.h>
/// KJRS_DEFINE_FUTURE(rust::String, RustFutureString);
/// ```
///
/// The `KJRS_DEFINE_FUTURE()` macro defines a C++ type identical in size and alignment to the Rust
/// type defined using the `#[kj_rs::bridge_future]` macro, allowing C++ to own futures.
///
/// If the future is inside a C++ namespace, add a `namespace = ...` attribute to the
/// `#[kj_rs::bridge_future]` attribute like so:
///
/// ```ignore
/// #[cxx::bridge]
/// #[namespace = mycompany::myproject]
/// mod ffi {
///     extern "Rust" {
///         type RustFutureStringNamespaced;
///     }
/// }
///
/// #[kj_rs::bridge(namespace = mycompany::myproject)]
/// unsafe impl Future for RustFutureStringNamespaced {
///     type Output = Result<String, Infallible>;
/// }
/// ```
///
/// To convert an arbitrary `Future` type into the future type you have defined, use
/// `Box::pin(future).into()`.
///
/// ```ignore
/// fn return_future() -> RustFutureString {
///     Box::pin(async {
///         "result"
///     }).into()
/// }
/// ```
// TODO(someday): Support non-Result Output types? The Result requirement is mostly to make the
//   implementation simpler.
#[proc_macro_attribute]
pub fn bridge_future(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let AstPieces {
        future,
        qualified_name,
        output,
        trait_path,
        drop_in_place_ident,
        drop_in_place_link_name,
        poll_ident,
        poll_link_name,
    } = match AstPieces::from_token_streams(attribute, item) {
        Err(error) => return error.into_compile_error().into(),
        Ok(pieces) => pieces,
    };

    (quote! {
        /// A future shared between Rust and C++.
        #[repr(transparent)]
        pub struct #future(::kj_rs::BoxFuture<#output>);

        // Define a Drop implementation so that end users don't. If end users are allowed to define
        // Drop, that could make our use of pinning unsound.
        impl Drop for #future {
            fn drop(&mut self) {}
        }

        // Allow automatic conversion from `Box::pin(future).into()`.
        impl<F> ::std::convert::From<::std::pin::Pin<::std::boxed::Box<F>>> for #future
        where
            F: ::std::future::Future<Output = #output> + Send + 'static
        {
            fn from(value: ::std::pin::Pin<::std::boxed::Box<F>>) -> Self {
                Self(value.into())
            }
        }

        // Implement the Rust Future trait.
        impl #trait_path for #future {
            type Output = #output;
            fn poll(self: ::std::pin::Pin<&mut Self>, cx: &mut ::std::task::Context) -> std::task::Poll<Self::Output> {
                // Our wrapped BoxFuture<T> is structurally pinned.
                //
                // Safety:
                // 1. We do not implement Unpin for #future.
                // 2. We implement the Drop trait, and it does not violate the pinned destruction
                //    guarantee. Additionally, #future is not declared #[repr(packed)].
                // 3. Our Drop trait is trivial, deferring all work to our one struct member,
                //    meaning there is no opportunity to fail to drop this member later.
                // 4. We offer no ability to move our wrapped BoxFuture<T> out of this type.
                //
                // https://doc.rust-lang.org/std/pin/index.html#choosing-pinning-to-be-structural-for-field
                let pinned = unsafe { self.map_unchecked_mut(|s| &mut s.0) };
                pinned.poll(cx)
            }
        }

        // Make sure that the future type can be returned by value.
        // See: https://github.com/dtolnay/cxx/pull/672
        unsafe impl ::cxx::ExternType for #future {
            type Id = ::cxx::type_id!(#qualified_name);
            type Kind = ::cxx::kind::Trivial;
        }

        // Define the C++ -> Rust drop() FFI forwarding function.
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[export_name = #drop_in_place_link_name]
        pub unsafe extern "C" fn #drop_in_place_ident(ptr: *mut #future) {
            ::std::ptr::drop_in_place(ptr);
        }

        // Define the C++ -> Rust poll() FFI forwarding function.
        #[doc(hidden)]
        #[allow(non_snake_case)]
        #[export_name = #poll_link_name]
        pub unsafe extern "C" fn #poll_ident(
            future: ::std::pin::Pin<&mut #future>,
            waker: &::kj_rs::KjWaker,
            result: *mut (),
        ) -> ::kj_rs::FuturePollStatus {
            ::kj_rs::box_future_poll(future, waker, result)
        }
    })
    .into()
}

// AST pieces generated for the `#[bridge_future]` macro.
struct AstPieces {
    // The name of the future type.
    future: Ident,
    // The fully-qualified name (i.e. including C++ namespace if any) of the future type, as a
    // quoted string.
    qualified_name: Lit,
    // The output type of the future.
    output: Type,
    // The path to the trait being implemented, which must be `std::future::Future`.
    trait_path: Path,
    // The internal Rust symbol name of the future's drop function.
    drop_in_place_ident: Ident,
    // The external C++ link name of the future's drop function.
    drop_in_place_link_name: String,
    // The internal Rust symbol name of the future's poll function.
    poll_ident: Ident,
    // The external C++ link name of the future's poll function.
    poll_link_name: String,
}

impl AstPieces {
    // Parses the macro arguments and returns the pieces, returning a `syn::Error` on error.
    fn from_token_streams(attribute: TokenStream, item: TokenStream) -> SynResult<AstPieces> {
        let namespace: NamespaceAttribute = syn::parse(attribute).map_err(|error| {
            SynError::new(error.span(), "expected possible namespace attribute")
        })?;

        let impl_item: ItemImpl = syn::parse(item)
            .map_err(|error| SynError::new(error.span(), "expected implementation of `Future`"))?;
        if impl_item.unsafety.is_none() {
            return Err(SynError::new(
                impl_item.span(),
                "implementation must be marked `unsafe`",
            ));
        }
        if impl_item.defaultness.is_some() {
            return Err(SynError::new(
                impl_item.span(),
                "implementation must not be marked default",
            ));
        }
        if !impl_item.generics.params.is_empty() {
            return Err(SynError::new(
                impl_item.generics.params[0].span(),
                "generic bridged futures are unsupported",
            ));
        }
        if let Some(where_clause) = impl_item.generics.where_clause {
            return Err(SynError::new(
                where_clause.where_token.span,
                "generic bridged futures are unsupported",
            ));
        }

        // We don't check to make sure that `path` is `std::future::Future` or `futures::Stream`
        // here, even though that's ultimately a requirement, because we would have to perform name
        // resolution here in the macro to do that. Instead, we take advantage of the fact that
        // we're going to be generating an implementation of the appropriate trait anyway and simply
        // supply whatever the user wrote as the name of the trait to be implemented in our final
        // macro expansion. That way, the Rust compiler ends up checking that the trait that the
        // user wrote is the right one.
        let trait_path = match impl_item.trait_ {
            Some((None, ref path, _)) => (*path).clone(),
            _ => {
                return Err(SynError::new(
                    impl_item.span(),
                    "must implement the `Future` trait",
                ));
            }
        };

        if impl_item.items.len() != 1 {
            return Err(SynError::new(
                impl_item.span(),
                "expected implementation to contain a single item, `type Output = ...`",
            ));
        }

        let output;
        match impl_item.items[0] {
            ImplItem::Type(ref impl_type) => {
                if !impl_type.attrs.is_empty() {
                    return Err(SynError::new(
                        impl_type.attrs[0].span(),
                        "attributes on the `type Output = ...` or declaration are not supported",
                    ));
                }
                match impl_type.vis {
                    Visibility::Inherited => {}
                    _ => {
                        return Err(SynError::new(
                            impl_type.vis.span(),
                            "`pub` or `crate` visibility modifiers on the `type Output = ...` \
                            declaration are not supported",
                        ));
                    }
                }
                if let Some(defaultness) = impl_type.defaultness {
                    return Err(SynError::new(
                        defaultness.span(),
                        "`default` specifier on the `type Output = ...` declaration is not \
                        supported",
                    ));
                }
                if !impl_type.generics.params.is_empty() {
                    return Err(SynError::new(
                        impl_type.generics.params[0].span(),
                        "generics on the `type Output = ...` declaration are not supported",
                    ));
                }

                if impl_type.ident != "Output" {
                    return Err(SynError::new(
                        impl_type.ident.span(),
                        "implementation must contain an associated type definition named \
                        `Output`",
                    ));
                };
                output = impl_type.ty.clone();
            }
            _ => {
                return Err(SynError::new(
                    impl_item.span(),
                    "expected implementation to contain a single item, `type Output = ...`",
                ));
            }
        };

        let future = match *impl_item.self_ty {
            Type::Path(TypePath { qself: None, path }) => {
                path.get_ident().cloned().ok_or_else(|| {
                    SynError::new(
                        path.span(),
                        "expected `impl` declaration to implement a single type",
                    )
                })?
            }
            _ => {
                return Err(SynError::new(
                    impl_item.self_ty.span(),
                    "expected `impl` declaration to implement a single type",
                ));
            }
        };

        let qualified_name = Lit::Str(LitStr::new(
            &format!(
                "{}{}",
                namespace
                    .0
                    .iter()
                    .fold(String::new(), |acc, piece| acc + piece + "::"),
                future
            ),
            future.span(),
        ));

        let drop_in_place_ident = Ident::new(
            &format!(
                "kjrs_{}{}_drop_in_place",
                namespace
                    .0
                    .iter()
                    .fold(String::new(), |acc, piece| acc + piece + "_"),
                future
            ),
            future.span(),
        );
        let drop_in_place_link_name = format!(
            "kjrs_{}{}_drop_in_place",
            namespace
                .0
                .iter()
                .fold(String::new(), |acc, piece| acc + piece + "$"),
            future
        );

        let poll_ident = Ident::new(
            &format!(
                "kjrs_{}{}_poll",
                namespace
                    .0
                    .iter()
                    .fold(String::new(), |acc, piece| acc + piece + "_"),
                future
            ),
            future.span(),
        );
        let poll_link_name = format!(
            "kjrs_{}{}_poll",
            namespace
                .0
                .iter()
                .fold(String::new(), |acc, piece| acc + piece + "$"),
            future
        );

        Ok(AstPieces {
            future,
            qualified_name,
            output,
            trait_path,
            drop_in_place_ident,
            drop_in_place_link_name,
            poll_ident,
            poll_link_name,
        })
    }
}

mod keywords {
    use syn::custom_keyword;
    custom_keyword!(namespace);
}

struct NamespaceAttribute(Vec<String>);

impl Parse for NamespaceAttribute {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        if input.is_empty() {
            return Ok(NamespaceAttribute(vec![]));
        }
        input.parse::<keywords::namespace>()?;
        input.parse::<Token![=]>()?;
        let path = input.call(Path::parse_mod_style)?;
        Ok(NamespaceAttribute(
            path.segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect(),
        ))
    }
}
