/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Inria
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, parse_macro_input, AngleBracketedGenericArguments, Data, DeriveInput};

/// A procedural macro fully implementing subslices on top of a
/// [`SliceByValueGet`].
///
/// The macro defines a structure `SubsliceImpl` that keeps track of a reference
/// to a slice, and of the start and end of the subslice. `SubsliceImpl` then
/// implements [`SliceByValueGet`] and [`SliceByValueSubslice`].
#[proc_macro_derive(Subslices)]
pub fn subslices(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = &input.generics.params;
    let ty_generics_token_stream = ty_generics.clone().into_token_stream();

    let names: proc_macro2::TokenStream = {
        if ty_generics_token_stream.is_empty() {
            // If the original struct has no generics (e.g., struct MyStruct;),
            // then ty_generics is empty, and we want an empty stream.
            proc_macro2::TokenStream::new()
        } else {
            // 2. Parse this TokenStream into a syn::AngleBracketedGenericArguments.
            //    This syn type represents the `T, A, B` arguments enclosed in angle brackets.
            let parsed_args: AngleBracketedGenericArguments =
                parse2(ty_generics_token_stream)
                    .expect("Failed to parse ty_generics into AngleBracketedGenericArguments. This indicates an unexpected structure in the generic parameters.");

            // 3. The `args` field of AngleBracketedGenericArguments is a Punctuated list
            //    (Punctuated<GenericArgument, Comma>) containing just the T, A, B.
            //    When you convert this Punctuated list to a TokenStream, it will
            //    automatically produce the comma-separated tokens without angle brackets.
            parsed_args.args.into_token_stream()
        }
    };
    match input.data {
        Data::Struct(_) => {
            quote! {
                #[automatically_derived]
                pub struct SubsliceImpl<'__subslice_impl, #params> {
                    slice: &'__subslice_impl #input_ident #ty_generics,
                    start: usize,
                    end: usize,
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValue for SubsliceImpl<'__subslice_impl, #names> #where_clause {
                    type Value = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;

                    #[inline]
                    fn len(&self) -> usize {
                        self.end - self.start
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueGet for SubsliceImpl<'__subslice_impl, #names> #where_clause  {
                    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
                        self.slice.get_value_unchecked(index + self.start)
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, '__subslice_gat, #params> ::value_traits::slices::SliceByValueSubsliceGat<'__subslice_gat> for SubsliceImpl<'__subslice_impl, #names> #where_clause {
                    type Subslice = SubsliceImpl<'__subslice_gat, #names>;
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<core::ops::Range<usize>>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::Range<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.start + range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<core::ops::RangeFrom<usize>>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeFrom<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<::core::ops::RangeToInclusive<usize>>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeToInclusive<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<core::ops::RangeFull>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        _range: ::core::ops::RangeFull,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<core::ops::RangeInclusive<usize>>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeInclusive<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        use ::core::{
                            ops::{Bound, RangeBounds},
                            hint::unreachable_unchecked
                        };
                        let start = match range.start_bound() {
                            Bound::Included(s) => *s,
                            // SAFETY: we cannot take this branch
                            _ => unsafe { unreachable_unchecked() },
                        };
                        let end = match range.end_bound() {
                            Bound::Included(s) => *s,
                            // SAFETY: we cannot take this branch
                            _ => unsafe { unreachable_unchecked() },
                        };
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start + start,
                            end: self.start + end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<::core::ops::RangeTo<usize>>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeTo<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceGat<'__subslice_impl> for #input_ident #ty_generics #where_clause  {
                    type Subslice = SubsliceImpl<'__subslice_impl, #names>;
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRange<core::ops::Range<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::Range<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: &self,
                            start: range.start,
                            end: range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRange<core::ops::RangeFrom<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeFrom<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: &self,
                            start: range.start,
                            end: self.len(),
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRange<::core::ops::RangeToInclusive<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeToInclusive<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: &self,
                            start: 0,
                            end: range.end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRange<core::ops::RangeFull> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        _range: ::core::ops::RangeFull,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: &self,
                            start: 0,
                            end: self.len(),
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRange<core::ops::RangeInclusive<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeInclusive<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        use ::core::{
                            ops::{Bound, RangeBounds},
                            hint::unreachable_unchecked
                        };
                        let start = match range.start_bound() {
                            Bound::Included(s) => *s,
                            // SAFETY: we cannot take this branch
                            _ => unsafe { unreachable_unchecked() },
                        };
                        let end = match range.end_bound() {
                            Bound::Included(s) => *s,
                            // SAFETY: we cannot take this branch
                            _ => unsafe { unreachable_unchecked() },
                        };
                        SubsliceImpl {
                            slice: &self,
                            start: start,
                            end: end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRange<::core::ops::RangeTo<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeTo<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: &self,
                            start: 0,
                            end: range.end,
                        }
                    }
                }
            }
        },

        _ => unimplemented!(),
    }
    .into()
}

/// A procedural macro fully implementing [`IterableByValue`] and
/// [`IterableByValueFrom`] for subslices on top of a the `SubsliceImpl`
/// structure generated by the derive macro [`Subslice`].
///
/// The macro defines a structure `Iter` that keeps track of a mutable reference
/// to a slice, and of a current position, and that is used to implement
/// [`IterableByValue`](crate::iter::IterableByValue) on `SubsliceImpl`.
#[proc_macro_derive(Iterators)]
pub fn iterators(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (_impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = &input.generics.params;
    let ty_generics_token_stream = ty_generics.clone().into_token_stream();

    let names: proc_macro2::TokenStream = {
        if ty_generics_token_stream.is_empty() {
            // If the original struct has no generics (e.g., struct MyStruct;),
            // then ty_generics is empty, and we want an empty stream.
            proc_macro2::TokenStream::new()
        } else {
            // 2. Parse this TokenStream into a syn::AngleBracketedGenericArguments.
            //    This syn type represents the `T, A, B` arguments enclosed in angle brackets.
            let parsed_args: AngleBracketedGenericArguments =
                parse2(ty_generics_token_stream)
                    .expect("Failed to parse ty_generics into AngleBracketedGenericArguments. This indicates an unexpected structure in the generic parameters.");

            // 3. The `args` field of AngleBracketedGenericArguments is a Punctuated list
            //    (Punctuated<GenericArgument, Comma>) containing just the T, A, B.
            //    When you convert this Punctuated list to a TokenStream, it will
            //    automatically produce the comma-separated tokens without angle brackets.
            parsed_args.args.into_token_stream()
        }
    };
    match input.data {
        Data::Struct(_) => {
            quote! {
                #[automatically_derived]
                pub struct Iter<'__subslice_impl, '__iter_ref, #params> {
                    subslice: &'__iter_ref SubsliceImpl<'__subslice_impl, #names>,
                    range: ::core::ops::Range<usize>,
                }

                #[automatically_derived]
                impl<'__subslice_impl, '__iter_ref, #params> Iter<'__subslice_impl, '__iter_ref, #names> #where_clause {
                    pub fn new(subslice: &'__iter_ref SubsliceImpl<'__subslice_impl, #names>) -> Self {
                        Self {
                            subslice,
                            range: 0..subslice.len(),
                        }
                    }
                    pub fn new_from(subslice: &'__iter_ref SubsliceImpl<'__subslice_impl, #names>, from: usize) -> Self {
                        let len = subslice.len();
                        assert!(from <= len, "index out of bounds: the len is {len} but the starting index is {from}");

                        Self {
                            subslice,
                            range: from..len,
                        }
                    }
                }

                #[automatically_derived]
                /// Ideally we would like to also implement [`Iterator::advance_by`], but it is
                /// nightly, and [`Iterator::skip`], [`Iterator::take`], [`Iterator::step_by`],
                /// as we can do it more efficiently, but the [`Iterator`] trait definition
                /// doesn't allow to return an arbitrary type.
                impl<'__subslice_impl, '__iter_ref, #params> Iterator for Iter<'__subslice_impl, '__iter_ref, #names> #where_clause {
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
                    /// [`Iterator::nth`] without needing to consume the first `n` elements.
                    #[inline]
                    fn nth(&mut self, n: usize) -> Option<Self::Item> {
                        if n >= self.range.end {
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
                }

                impl<'__subslice_impl, '__iter_ref, #params> DoubleEndedIterator for Iter<'__subslice_impl, '__iter_ref, #names> #where_clause {
                    #[inline]
                    fn next_back(&mut self) -> Option<Self::Item> {
                        if self.range.is_empty() {
                            return ::core::option::Option::None;
                        }
                        self.range.end -= 1;
                        let value = unsafe { self.subslice.get_value_unchecked(self.range.end) };
                        ::core::option::Option::Some(value)
                    }
                }

                impl<'__subslice_impl, '__iter_ref, #params> ExactSizeIterator for Iter<'__subslice_impl, '__iter_ref, #names> #where_clause {
                    #[inline]
                    fn len(&self) -> usize {
                        self.range.len()
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> IterableByValue for SubsliceImpl<'__subslice_impl, #names> #where_clause {
                    type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;
                    type Iter<'__iter_ref>
                        = Iter<'__subslice_impl, '__iter_ref, #names>
                    where
                        Self: '__iter_ref;

                    #[inline]
                    fn iter_value(&self) -> Self::Iter<'_> {
                        Iter::new(self)
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> IterableByValueFrom for SubsliceImpl<'__subslice_impl, #names> #where_clause {
                    type IterFrom<'__iter_ref>
                        = Iter<'__subslice_impl, '__iter_ref, #names>
                    where
                        Self: '__iter_ref;

                    #[inline]
                    fn iter_value_from(&self, from: usize) -> Self::IterFrom<'_> {
                        let len = self.len();
                        assert!(from <= len, "index out of bounds: the len is {len} but the starting index is {from}");

                        Iter::new_from(self, from)
                    }
                }
            }
        },

        _ => unimplemented!(),
    }
    .into()
}

/// A procedural macro fully implementing mutable subslices on top of a
/// [`SliceByValueSet`]/[`SliceByValueRepl`] for which the derive macro
/// [`Subslice`] has been already applied.
///
/// The macro defines a structure `SubsliceImplMut` that keeps track of a
/// mutable reference to a slice, and of the start and end of the subslice.
/// `SubsliceImplMut` then implements [`SliceByValueGet`], [`SliceByValueSet`],
/// [`SliceByValueRepl`], [`SliceByValueSubslice`], and
/// [`SliceByValueSubsliceMut`].
#[proc_macro_derive(SubslicesMut)]
pub fn subslices_mut(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = &input.generics.params;
    let ty_generics_token_stream = ty_generics.clone().into_token_stream();

    let names: proc_macro2::TokenStream = {
        if ty_generics_token_stream.is_empty() {
            // If the original struct has no generics (e.g., struct MyStruct;),
            // then ty_generics is empty, and we want an empty stream.
            proc_macro2::TokenStream::new()
        } else {
            // 2. Parse this TokenStream into a syn::AngleBracketedGenericArguments.
            //    This syn type represents the `T, A, B` arguments enclosed in angle brackets.
            let parsed_args: AngleBracketedGenericArguments =
                parse2(ty_generics_token_stream)
                    .expect("Failed to parse ty_generics into AngleBracketedGenericArguments. This indicates an unexpected structure in the generic parameters.");

            // 3. The `args` field of AngleBracketedGenericArguments is a Punctuated list
            //    (Punctuated<GenericArgument, Comma>) containing just the T, A, B.
            //    When you convert this Punctuated list to a TokenStream, it will
            //    automatically produce the comma-separated tokens without angle brackets.
            parsed_args.args.into_token_stream()
        }
    };
    match input.data {
        Data::Struct(_) => {
            quote! {
                #[automatically_derived]
                pub struct SubsliceImplMut<'__subslice_impl, #params> {
                    slice: &'__subslice_impl mut #input_ident #ty_generics,
                    start: usize,
                    end: usize,
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValue for SubsliceImplMut<'__subslice_impl, #names> #where_clause {
                    type Value = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;

                    #[inline]
                    fn len(&self) -> usize {
                        self.end - self.start
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueGet for SubsliceImplMut<'__subslice_impl, #names> #where_clause  {
                    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
                        self.slice.get_value_unchecked(index + self.start)
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, '__subslice_gat, #params> ::value_traits::slices::SliceByValueSubsliceGat<'__subslice_gat> for SubsliceImplMut<'__subslice_impl, #names> #where_clause {
                    type Subslice = SubsliceImpl<'__subslice_gat, #names>;
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<core::ops::Range<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::Range<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.start + range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<core::ops::RangeFrom<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeFrom<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<::core::ops::RangeToInclusive<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeToInclusive<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<core::ops::RangeFull>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        _range: ::core::ops::RangeFull,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<core::ops::RangeInclusive<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeInclusive<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        use core::ops::{Bound, RangeBounds};
                        use std::hint::unreachable_unchecked;
                        let start = match range.start_bound() {
                            Bound::Included(s) => *s,
                            // SAFETY: we cannot take this branch
                            _ => unsafe { unreachable_unchecked() },
                        };
                        let end = match range.end_bound() {
                            Bound::Included(s) => *s,
                            // SAFETY: we cannot take this branch
                            _ => unsafe { unreachable_unchecked() },
                        };
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start + start,
                            end: self.start + end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRange<::core::ops::RangeTo<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: ::core::ops::RangeTo<usize>,
                    ) -> ::value_traits::slices::Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSet for SubsliceImplMut<'__subslice_impl, #names> #where_clause  {
                    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
                        self.slice.set_value_unchecked(index + self.start, value)
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueRepl for SubsliceImplMut<'__subslice_impl, #names> #where_clause  {
                    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
                        self.slice.replace_value_unchecked(index + self.start, value)
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, '__subslice_gat, #params> ::value_traits::slices::SliceByValueSubsliceGatMut<'__subslice_gat> for SubsliceImplMut<'__subslice_impl, #names> #where_clause {
                    type Subslice = SubsliceImplMut<'__subslice_gat, #names>;
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRangeMut<core::ops::Range<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: ::core::ops::Range<usize>,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.start + range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRangeMut<core::ops::RangeFrom<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: ::core::ops::RangeFrom<usize>,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRangeMut<::core::ops::RangeToInclusive<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: ::core::ops::RangeToInclusive<usize>,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRangeMut<core::ops::RangeFull>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        _range: ::core::ops::RangeFull,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self.slice,
                            start: self.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRangeMut<core::ops::RangeInclusive<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: ::core::ops::RangeInclusive<usize>,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        use core::ops::{Bound, RangeBounds};
                        use std::hint::unreachable_unchecked;
                        let start = match range.start_bound() {
                            Bound::Included(s) => *s,
                            // SAFETY: we cannot take this branch
                            _ => unsafe { unreachable_unchecked() },
                        };
                        let end = match range.end_bound() {
                            Bound::Included(s) => *s,
                            // SAFETY: we cannot take this branch
                            _ => unsafe { unreachable_unchecked() },
                        };
                        SubsliceImplMut {
                            slice: self.slice,
                            start: self.start + start,
                            end: self.start + end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceRangeMut<::core::ops::RangeTo<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: ::core::ops::RangeTo<usize>,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceGatMut<'__subslice_impl> for #input_ident #ty_generics #where_clause  {
                    type Subslice = SubsliceImplMut<'__subslice_impl, #names>;
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRangeMut<core::ops::Range<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: ::core::ops::Range<usize>,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self,
                            start: range.start,
                            end: range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRangeMut<core::ops::RangeFrom<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: ::core::ops::RangeFrom<usize>,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        let end = self.len();
                        SubsliceImplMut {
                            slice: self,
                            start: range.start,
                            end
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRangeMut<::core::ops::RangeToInclusive<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: ::core::ops::RangeToInclusive<usize>,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self,
                            start: 0,
                            end: range.end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRangeMut<core::ops::RangeFull> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        _range: ::core::ops::RangeFull,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        let end = self.len();
                        SubsliceImplMut {
                            slice: self,
                            start: 0,
                            end,
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRangeMut<core::ops::RangeInclusive<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: ::core::ops::RangeInclusive<usize>,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        use core::ops::{Bound, RangeBounds};
                        use std::hint::unreachable_unchecked;

                        let start = match range.start_bound() {
                            Bound::Included(s) => *s,
                            // SAFETY: we cannot take this branch
                            _ => unsafe { unreachable_unchecked() },
                        };
                        let end = match range.end_bound() {
                            Bound::Included(s) => *s,
                            // SAFETY: we cannot take this branch
                            _ => unsafe { unreachable_unchecked() },
                        };
                        SubsliceImplMut {
                            slice: self,
                            start: start,
                            end: end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics ::value_traits::slices::SliceByValueSubsliceRangeMut<::core::ops::RangeTo<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: ::core::ops::RangeTo<usize>,
                    ) -> ::value_traits::slices::SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self,
                            start: 0,
                            end: range.end,
                        }
                    }
                }
            }
        },

        _ => unimplemented!(),
    }
    .into()
}

/// A procedural macro fully implementing [`IterableByValue`] and
/// [`IterableByValueFrom`] for subslices on top of a the `SubsliceImplMut`
/// structure generated by the derive macro [`SubsliceMut`].
///
/// The macro defines a structure `IterMut` that keeps track of a mutable reference
/// to a slice, and of a current position, and that is used to implement
/// [`IterableByValue`](crate::iter::IterableByValue) on `SubsliceImpl`.
///
/// Note that since `IterMut` provides iterators by value, it cannot use to
/// mutate the subslice. Moreover, non-mutable subslicing on a
/// `SubsliceImplMut` will yield a `SubsliceImpl`.
#[proc_macro_derive(IteratorsMut)]
pub fn iterators_mut(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (_impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = &input.generics.params;
    let ty_generics_token_stream = ty_generics.clone().into_token_stream();

    let names: proc_macro2::TokenStream = {
        if ty_generics_token_stream.is_empty() {
            // If the original struct has no generics (e.g., struct MyStruct;),
            // then ty_generics is empty, and we want an empty stream.
            proc_macro2::TokenStream::new()
        } else {
            // 2. Parse this TokenStream into a syn::AngleBracketedGenericArguments.
            //    This syn type represents the `T, A, B` arguments enclosed in angle brackets.
            let parsed_args: AngleBracketedGenericArguments =
                parse2(ty_generics_token_stream)
                    .expect("Failed to parse ty_generics into AngleBracketedGenericArguments. This indicates an unexpected structure in the generic parameters.");

            // 3. The `args` field of AngleBracketedGenericArguments is a Punctuated list
            //    (Punctuated<GenericArgument, Comma>) containing just the T, A, B.
            //    When you convert this Punctuated list to a TokenStream, it will
            //    automatically produce the comma-separated tokens without angle brackets.
            parsed_args.args.into_token_stream()
        }
    };
    match input.data {
        Data::Struct(_) => {
            quote! {
                // TODO: maybe referring directly to the slice?
                #[automatically_derived]
                pub struct IterMut<'__subslice_impl, '__iter_ref, #params> {
                    subslice: &'__iter_ref SubsliceImplMut<'__subslice_impl, #names>,
                    index: usize,
                }

                #[automatically_derived]
                impl<'__subslice_impl, '__iter_ref, #params> Iterator for IterMut<'__subslice_impl, '__iter_ref, #names> #where_clause {
                    type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;

                    #[inline]
                    fn next(&mut self) -> Option<Self::Item> {
                        if self.index < self.subslice.len() {
                            let value = unsafe { self.subslice.get_value_unchecked(self.index) };
                            self.index += 1;
                            ::core::option::Option::Some(value)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> IterableByValue for SubsliceImplMut<'__subslice_impl, #names> #where_clause {
                    type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;
                    type Iter<'__iter_ref>
                        = IterMut<'__subslice_impl, '__iter_ref, #names>
                    where
                        Self: '__iter_ref;

                    #[inline]
                    fn iter_value(&self) -> Self::Iter<'_> {
                        IterMut {
                            subslice: self,
                            index: 0,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> IterableByValueFrom for SubsliceImplMut<'__subslice_impl, #names> #where_clause {
                    type IterFrom<'__iter_ref>
                        = IterMut<'__subslice_impl, '__iter_ref, #names>
                    where
                        Self: '__iter_ref;

                    #[inline]
                    fn iter_value_from(&self, from: usize) -> Self::IterFrom<'_> {
                        let len = self.len();
                        assert!(from <= len, "index out of bounds: the len is {len} but the starting index is {from}");
                        
                        IterMut {
                            subslice: self,
                            index: from,
                        }
                    }
                }
            }
        },

        _ => unimplemented!(),
    }
    .into()
}
