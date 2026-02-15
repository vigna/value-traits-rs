/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Inria
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

#![warn(missing_docs)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]

//! Derive macros for the [`value-traits`](https://docs.rs/value-traits/latest/value_traits/) crate.

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    AngleBracketedGenericArguments, DeriveInput, parse_macro_input, parse2, punctuated::Punctuated,
};

/// Helper function returning the list of parameter names without angle brackets.
fn get_names(ty_generics_token_stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    if ty_generics_token_stream.is_empty() {
        proc_macro2::TokenStream::new()
    } else {
        let parsed_args: AngleBracketedGenericArguments =
            parse2(ty_generics_token_stream)
                .expect("Failed to parse ty_generics into AngleBracketedGenericArguments. This indicates an unexpected structure in the generic parameters.");

        parsed_args.args.into_token_stream()
    }
}

/// Helper function to extract additional bounds from attributes
fn extract_additional_bounds(
    input: &DeriveInput,
    attr_name: &str,
) -> Vec<proc_macro2::TokenStream> {
    let mut additional_bounds = Vec::new();
    for attr in &input.attrs {
        if attr.path().is_ident(attr_name) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("bound") {
                    let bound: syn::LitStr = meta.value()?.parse()?;
                    let bound_tokens: proc_macro2::TokenStream =
                        bound.value().parse().expect("Failed to parse bound");
                    additional_bounds.push(bound_tokens);
                }
                Ok(())
            })
            .unwrap_or_else(|e| panic!("Failed to parse attribute {attr_name}: {e}"));
        }
    }
    additional_bounds
}

/// Helper function to add additional bounds to a where clause
fn add_bounds_to_where_clause(
    generics: &mut syn::Generics,
    additional_bounds: Vec<proc_macro2::TokenStream>,
) {
    if !additional_bounds.is_empty() {
        let where_clause = generics.make_where_clause();
        for bound in additional_bounds {
            let predicate: syn::WherePredicate =
                syn::parse2(bound).expect("Invalid where predicate");
            where_clause.predicates.push(predicate);
        }
    }
}

fn get_params_without_defaults(
    generics: &syn::Generics,
) -> Punctuated<syn::GenericParam, syn::token::Comma> {
    // Remove default type parameters
    let mut params = generics.params.clone();
    params.iter_mut().for_each(|param| match param {
        syn::GenericParam::Type(ty_param) => {
            ty_param.default = None;
        }
        syn::GenericParam::Const(const_param) => {
            const_param.default = None;
        }
        _ => {}
    });
    params
}

/// A derive macro fully implementing subslices on top of a
/// [`SliceByValue`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValue.html).
///
/// The macro defines a structure `<YOUR TYPE>SubsliceImpl` that keeps track of
/// a reference to a slice, and of the start and end of the subslice.
/// `<YOUR TYPE>SubsliceImpl` then implements
/// [`SliceByValue`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValue.html)
/// and
/// [`SliceByValueSubslice`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSubslice.html).
///
/// ## Additional Bounds
///
/// Since this macro has no knowledge of the bounds of the generic
/// parameters in the implementations of
/// [`SliceByValue`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValue.html),
/// additional bounds with respect to the type declaration must be specified
/// using the `#[value_traits_subslices(bound = "<BOUND>")]` attribute. Multiple bounds can
/// be specified with multiple attributes.
#[proc_macro_derive(Subslices, attributes(value_traits_subslices))]
pub fn subslices(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    // Extract and add additional bounds
    let additional_bounds = extract_additional_bounds(&input, "value_traits_subslices");
    add_bounds_to_where_clause(&mut input.generics, additional_bounds);

    let input_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = get_params_without_defaults(&input.generics);
    let ty_generics_token_stream = ty_generics.clone().into_token_stream();

    let names = get_names(ty_generics_token_stream);
    let subslice_impl = quote::format_ident!("{}SubsliceImpl", input_ident);
    let mut res = quote! {
        #[automatically_derived]
        pub struct #subslice_impl<'__subslice_impl, #params> {
            slice: &'__subslice_impl #input_ident #ty_generics,
            range: ::core::ops::Range<usize>,
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValue for #subslice_impl<'__subslice_impl, #names> #where_clause {
            type Value = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;

            #[inline]
            fn len(&self) -> usize {
                self.range.len()
            }

            unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
                self.slice.get_value_unchecked(index + self.range.start)
            }
        }

        #[automatically_derived]
        impl<'__subslice_impl, '__subslice_gat, #params> ::value_traits::slices::SliceByValueSubsliceGat<'__subslice_gat> for #subslice_impl<'__subslice_impl, #names> #where_clause {
            type Subslice = #subslice_impl<'__subslice_gat, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceGat<'__subslice_impl> for #input_ident #ty_generics #where_clause  {
            type Subslice = #subslice_impl<'__subslice_impl, #names>;
        }
    };

    for range_type in [
        quote! { ::core::ops::Range<usize> },
        quote! { ::core::ops::RangeFrom<usize> },
        quote! { ::core::ops::RangeToInclusive<usize> },
        quote! { ::core::ops::RangeFull },
        quote! { ::core::ops::RangeInclusive<usize> },
        quote! { ::core::ops::RangeTo<usize> },
    ] {
        res.extend(quote! {
            #[automatically_derived]
            impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRange<#range_type> for #input_ident #ty_generics #where_clause {
                unsafe fn get_subslice_unchecked(
                    &self,
                    range: #range_type,
                ) -> ::value_traits::slices::Subslice<'_, Self> {
                    #subslice_impl {
                        slice: &self,
                        range: ::value_traits::slices::ComposeRange::compose(&range, 0..self.len()),
                    }
                }
            }
            #[automatically_derived]
            impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<#range_type>
                for #subslice_impl<'__subslice_impl, #names> #where_clause
            {
                unsafe fn get_subslice_unchecked(
                    &self,
                    range: #range_type,
                ) -> ::value_traits::slices::Subslice<'_, Self> {
                    #subslice_impl {
                        slice: self.slice,
                        range: ::value_traits::slices::ComposeRange::compose(&range, self.range.clone()),
                    }
                }
            }
        });
    }

    res.into()
}

/// A derive macro fully implementing mutable subslices on top of a
/// [`SliceByValueMut`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueMut.html)
/// for which the derive macro [`Subslices`] has been already applied.
///
/// The macro defines a structure `<YOUR TYPE>SubsliceImplMut` that keeps track
/// of a mutable reference to a slice, and of the start and end of the subslice.
/// `<YOUR TYPE>SubsliceImplMut` then implements
/// [`SliceByValueMut`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueMut.html),
/// [`SliceByValueSubslice`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSubslice.html),
/// and
/// [`SliceByValueSubsliceMut`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSubsliceMut.html).
///
/// Note that
/// [`SliceByValueSubslice`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSubslice.html)
/// methods will return the `<YOUR TYPE>SubsliceImpl` structure generated by the
/// [`Subslices`] macro.
///
/// ## Chunks
///
/// Presently, [`try_chunks_mut`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueMut.html#method.try_chunks_mut)
/// is not supported.
///
/// ## Additional Bounds
///
/// Since this macro has no knowledge of the bounds of the generic parameters in
/// the implementations of
/// [`SliceByValue`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValue.html)
/// and
/// [`SliceByValueMut`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueMut.html),
/// additional bounds with respect to the type declaration must be specified
/// using the `#[value_traits_subslices_mut(bound = "<BOUND>")]` attribute.
/// Multiple bounds can be specified with multiple attributes.
#[proc_macro_derive(SubslicesMut, attributes(value_traits_subslices_mut))]
pub fn subslices_mut(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    // Extract and add additional bounds
    let additional_bounds = extract_additional_bounds(&input, "value_traits_subslices_mut");
    add_bounds_to_where_clause(&mut input.generics, additional_bounds);

    let input_ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = get_params_without_defaults(&input.generics);
    let ty_generics_token_stream = ty_generics.clone().into_token_stream();

    let names = get_names(ty_generics_token_stream);
    let subslice_impl = quote::format_ident!("{}SubsliceImpl", input_ident);
    let subslice_impl_mut = quote::format_ident!("{}SubsliceImplMut", input_ident);
    let mut res = quote! {
        #[automatically_derived]
        pub struct #subslice_impl_mut<'__subslice_impl, #params> {
            slice: &'__subslice_impl mut #input_ident #ty_generics,
            range: ::core::ops::Range<usize>,
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValue for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            type Value = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;

            #[inline]
            fn len(&self) -> usize {
                self.range.len()
            }

            unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
                self.slice.get_value_unchecked(index + self.range.start)
            }
        }


        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueMut for #subslice_impl_mut<'__subslice_impl, #names> #where_clause  {
            unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
                self.slice.set_value_unchecked(index + self.range.start, value)
            }

            unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
                self.slice.replace_value_unchecked(index + self.range.start, value)
            }

            type ChunksMut<'a> = ::core::iter::Empty<&'a mut Self>
            where
                Self: 'a;

            type ChunksMutError = ::value_traits::slices::ChunksMutNotSupported;

            fn try_chunks_mut(&mut self, _chunk_size: usize) -> Result<Self::ChunksMut<'_>, Self::ChunksMutError> {
                // Derived subslice types cannot provide mutable chunks
                Err(::value_traits::slices::ChunksMutNotSupported)
            }
        }

        #[automatically_derived]
        impl<'__subslice_impl, '__subslice_gat, #params> ::value_traits::slices::SliceByValueSubsliceGat<'__subslice_gat> for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            type Subslice = #subslice_impl<'__subslice_gat, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, '__subslice_gat, #params> ::value_traits::slices::SliceByValueSubsliceGatMut<'__subslice_gat> for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            type SubsliceMut = #subslice_impl_mut<'__subslice_gat, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceGatMut<'__subslice_impl> for #input_ident #ty_generics #where_clause  {
            type SubsliceMut = #subslice_impl_mut<'__subslice_impl, #names>;
        }

    };

    for range_type in [
        quote! { ::core::ops::Range<usize> },
        quote! { ::core::ops::RangeFrom<usize> },
        quote! { ::core::ops::RangeToInclusive<usize> },
        quote! { ::core::ops::RangeFull },
        quote! { ::core::ops::RangeInclusive<usize> },
        quote! { ::core::ops::RangeTo<usize> },
    ] {
        // Impl subslice mut traits for the original type
        res.extend(quote!{
            #[automatically_derived]
            impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRangeMut<#range_type> for #input_ident #ty_generics #where_clause {
                unsafe fn get_subslice_unchecked_mut(
                    &mut self,
                    range: #range_type,
                ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                    let len = self.len();
                    #subslice_impl_mut {
                        slice: self,
                        range: ::value_traits::slices::ComposeRange::compose(&range, 0..len),
                    }
                }
            }
            #[automatically_derived]
            impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<#range_type>
                for #subslice_impl_mut<'__subslice_impl, #names> #where_clause
            {
                unsafe fn get_subslice_unchecked(
                    &self,
                    range: #range_type,
                ) -> ::value_traits::slices::Subslice<'_, Self> {
                    #subslice_impl {
                        slice: &*self.slice,
                        range: ::value_traits::slices::ComposeRange::compose(&range, self.range.clone()),
                    }
                }
            }
            #[automatically_derived]
            impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRangeMut<#range_type>
                for #subslice_impl_mut<'__subslice_impl, #names> #where_clause
            {
                unsafe fn get_subslice_unchecked_mut(
                    &mut self,
                    range: #range_type,
                ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                    #subslice_impl_mut {
                        slice: self.slice,
                        range: ::value_traits::slices::ComposeRange::compose(&range, self.range.clone()),
                    }
                }
            }
        });
    }

    res.into()
}

/// A derive macro fully implementing
/// [`IterateByValue`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterateByValue.html)
/// and
/// [`IterateByValueFrom`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterateByValueFrom.html)
/// for subslices on top of a the `<YOUR TYPE>SubsliceImpl` structure generated
/// by the derive macro [`Subslices`].
///
/// The macro defines a structure `<YOUR TYPE>Iter` that keeps track of a
/// mutable reference to a slice and of a current iteration range; the structure
/// is used to implement
/// [`IterateByValue`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterateByValue.html)
/// and
/// [`IterateByValueFrom`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterateByValueFrom.html)
/// on `<YOUR TYPE>SubsliceImpl`.
///
/// ## Additional Bounds
///
/// Since this macro has no knowledge of the bounds of the generic
/// parameters in the implementations of
/// [`SliceByValue`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValue.html),
/// additional bounds with respect to the type declaration must be specified
/// using the `#[value_traits_iterators(bound = "<BOUND>")]` attribute. Multiple bounds can
/// be specified with multiple attributes.
#[proc_macro_derive(Iterators, attributes(value_traits_iterators))]
pub fn iterators(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    // Extract and add additional bounds
    let additional_bounds = extract_additional_bounds(&input, "value_traits_iterators");
    add_bounds_to_where_clause(&mut input.generics, additional_bounds);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (_impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = get_params_without_defaults(&input.generics);
    let ty_generics_token_stream = ty_generics.clone().into_token_stream();

    let names = get_names(ty_generics_token_stream);
    let subslice_impl = quote::format_ident!("{}SubsliceImpl", input_ident);
    let iter = quote::format_ident!("{}Iter", input_ident);
    quote! {
        #[automatically_derived]
        pub struct #iter<'__iter_ref, #params> {
            subslice: &'__iter_ref #input_ident #ty_generics,
            range: ::core::ops::Range<usize>,
        }

        #[automatically_derived]
        impl<'__iter_ref, #params> #iter<'__iter_ref, #names> #where_clause {
            pub fn new(subslice: &'__iter_ref #input_ident #ty_generics) -> Self {
                let len = subslice.len();
                Self {
                    subslice,
                    range: 0..len,
                }
            }
            pub fn new_with_range(subslice: &'__iter_ref #input_ident #ty_generics, range: ::core::ops::Range<usize>) -> Self {
                Self {
                    subslice,
                    range,
                }
            }
        }

        /*#[automatically_derived]
        impl<#params> ::value_traits::iter::IterateByValue for #input_ident #ty_generics #where_clause {
            type Item = <Self as ::value_traits::slices::SliceByValue>::Value;
            type Iter<'__iter_ref>
                = #iter<'__iter_ref, #names>
            where
                Self: '__iter_ref;

            #[inline]
            fn iter_value(&self) -> Self::Iter<'_> {
                #iter::new(self)
            }
        }

        #[automatically_derived]
        impl<#params> ::value_traits::iter::IterateByValueFrom for #input_ident #ty_generics #where_clause {
            type IterFrom<'__iter_ref>
                = #iter<'__iter_ref, #names>
            where
                Self: '__iter_ref;

            #[inline]
            fn iter_value_from(&self, from: usize) -> Self::IterFrom<'_> {
                let len = self.len();
                assert!(from <= len, "index out of bounds: the len is {len} but the starting index is {from}");
                #iter::new_with_range(self, from..len)
            }
        }*/

        #[automatically_derived]
        /// Ideally we would like to also implement [`::core::iter::Iterator::advance_by`], but it is
        /// nightly, and [`::core::iter::Iterator::skip`], [`::core::iter::Iterator::take`], [`::core::iter::Iterator::step_by`],
        /// as we can do it more efficiently, but the [`::core::iter::Iterator`] trait definition
        /// doesn't allow to return an arbitrary type.
        impl<'__iter_ref, #params> ::core::iter::Iterator for #iter<'__iter_ref, #names> #where_clause {
            type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                if self.range.is_empty() {
                    return ::core::option::Option::None;
                }
                let value = unsafe { self.subslice.get_value_unchecked(self.range.start) };
                self.range.start += 1;
                ::core::option::Option::Some(value)
            }

            /// Since we are indexing into a subslice, we can implement
            /// [`::core::iter::Iterator::nth`] without needing to consume the first `n` elements.
            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                if n >= self.range.len() {
                    self.range.start = self.range.end; // consume the iterator
                    return ::core::option::Option::None;
                }
                let value = unsafe { self.subslice.get_value_unchecked(self.range.start + n) };
                self.range.start += n + 1;
                ::core::option::Option::Some(value)
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.range.len();
                (len, Some(len))
            }

            #[inline]
            fn count(self) -> usize {
                self.range.len()
            }

            #[inline]
            fn last(self) -> ::core::option::Option<Self::Item> {
                if self.range.is_empty() {
                    return ::core::option::Option::None;
                }
                ::core::option::Option::Some(unsafe { self.subslice.get_value_unchecked(self.range.end - 1) })
            }

            fn fold<__FoldB, __FoldF>(self, init: __FoldB, mut f: __FoldF) -> __FoldB
            where
                __FoldF: FnMut(__FoldB, Self::Item) -> __FoldB,
            {
                let subslice = self.subslice;
                let mut acc = init;
                for idx in self.range {
                    acc = f(acc, unsafe { subslice.get_value_unchecked(idx) });
                }
                acc
            }

            fn for_each<__ForEachF>(self, mut f: __ForEachF)
            where
                __ForEachF: FnMut(Self::Item),
            {
                let subslice = self.subslice;
                for idx in self.range {
                    f(unsafe { subslice.get_value_unchecked(idx) });
                }
            }
        }

        impl<'__iter_ref, #params> ::core::iter::DoubleEndedIterator for #iter<'__iter_ref, #names> #where_clause {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                if self.range.is_empty() {
                    return ::core::option::Option::None;
                }
                self.range.end -= 1;
                let value = unsafe { self.subslice.get_value_unchecked(self.range.end) };
                ::core::option::Option::Some(value)
            }

            #[inline]
            fn nth_back(&mut self, n: usize) -> ::core::option::Option<Self::Item> {
                if n >= self.range.len() {
                    self.range.end = self.range.start;
                    return ::core::option::Option::None;
                }
                self.range.end -= n + 1;
                ::core::option::Option::Some(unsafe { self.subslice.get_value_unchecked(self.range.end) })
            }

            fn rfold<__RFoldB, __RFoldF>(self, init: __RFoldB, mut f: __RFoldF) -> __RFoldB
            where
                __RFoldF: FnMut(__RFoldB, Self::Item) -> __RFoldB,
            {
                let subslice = self.subslice;
                let mut acc = init;
                for idx in self.range.rev() {
                    acc = f(acc, unsafe { subslice.get_value_unchecked(idx) });
                }
                acc
            }
        }

        impl<'__iter_ref, #params> ::core::iter::ExactSizeIterator for #iter<'__iter_ref, #names> #where_clause {
            #[inline]
            fn len(&self) -> usize {
                self.range.len()
            }
        }

        impl<'__iter_ref, #params> ::core::iter::FusedIterator for #iter<'__iter_ref, #names> #where_clause {}

        #[automatically_derived]
        impl<'__subslice_impl, '__iter_ref, #params> ::value_traits::iter::IterateByValueGat<'__iter_ref> for #subslice_impl<'__subslice_impl, #names> #where_clause {
            type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;
            type Iter = #iter<'__iter_ref, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::iter::IterateByValue for #subslice_impl<'__subslice_impl, #names> #where_clause {
            #[inline]
            fn iter_value(&self) -> ::value_traits::iter::Iter<'_, Self> {
                #iter::new_with_range(self.slice, self.range.clone())
            }
        }

        #[automatically_derived]
        impl<'__subslice_impl, '__iter_ref,#params> ::value_traits::iter::IterateByValueFromGat<'__iter_ref> for #subslice_impl<'__subslice_impl, #names> #where_clause {
            type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;
            type IterFrom = #iter<'__iter_ref, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::iter::IterateByValueFrom for #subslice_impl<'__subslice_impl, #names> #where_clause {
            #[inline]
            fn iter_value_from(&self, from: usize) -> ::value_traits::iter::IterFrom<'_, Self> {
                let len = self.len();
                assert!(from <= len, "index out of bounds: the len is {len} but the starting index is {from}");
                let range = ::value_traits::slices::ComposeRange::compose(&(from..), self.range.clone());
                #iter::new_with_range(self.slice, range)
            }
        }
    }.into()
}

/// A derive macro that implements
/// [`IterateByValue`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterateByValue.html)
/// and
/// [`IterateByValueFrom`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterateByValueFrom.html)
/// for mutable subslices on top of the `<YOUR TYPE>SubsliceImplMut` structure
/// generated by the derive macro [`SubslicesMut`].
///
/// To call this macro, you first need to derive both [`SubslicesMut`] and
/// [`Iterators`] on the same struct, as this macro uses the `<YOUR TYPE>Iter`
/// structure defined by [`Iterators`].
///
/// ## Additional Bounds
///
/// Since this macro has no knowledge of the bounds of the generic parameters in
/// the implementations of
/// [`SliceByValue`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValue.html) and
/// [`SliceByValueMut`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueMut.html),
/// additional bounds with respect to the type declaration must be specified
/// using the `#[value_traits_iterators_mut(bound = "<BOUND>")]` attribute.
/// Multiple bounds can be specified with multiple attributes.
#[proc_macro_derive(IteratorsMut, attributes(value_traits_iterators_mut))]
pub fn iterators_mut(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    // Extract and add additional bounds
    let additional_bounds = extract_additional_bounds(&input, "value_traits_iterators_mut");
    add_bounds_to_where_clause(&mut input.generics, additional_bounds);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (_impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = get_params_without_defaults(&input.generics);
    let ty_generics_token_stream = ty_generics.clone().into_token_stream();

    let names = get_names(ty_generics_token_stream);
    let subslice_impl_mut = quote::format_ident!("{}SubsliceImplMut", input_ident);
    let iter = quote::format_ident!("{}Iter", input_ident);
    quote!{
        #[automatically_derived]
        impl<'__subslice_impl, '__iter_ref, #params> ::value_traits::iter::IterateByValueGat<'__iter_ref> for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;
            type Iter = #iter<'__iter_ref, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::iter::IterateByValue for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            fn iter_value(&self) -> ::value_traits::iter::Iter<'_, Self> {
                #iter::new_with_range(self.slice, self.range.clone())
            }
        }

        #[automatically_derived]
        impl<'__subslice_impl, '__iter_ref, #params> ::value_traits::iter::IterateByValueFromGat<'__iter_ref> for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;
            type IterFrom = #iter<'__iter_ref, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::iter::IterateByValueFrom for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            fn iter_value_from(&self, from: usize) -> ::value_traits::iter::IterFrom<'_, Self> {
                let len = self.len();
                assert!(from <= len, "index out of bounds: the len is {len} but the starting index is {from}");
                let range = ::value_traits::slices::ComposeRange::compose(&(from..), self.range.clone());
                #iter::new_with_range(self.slice, range)
            }
        }
    }.into()
}
