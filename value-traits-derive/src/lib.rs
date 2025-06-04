/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Inria
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{AngleBracketedGenericArguments, Data, DeriveInput, parse_macro_input, parse2};

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
                impl<'__subslice_impl, #params> SliceByValue for SubsliceImpl<'__subslice_impl, #names> #where_clause {
                    type Value = <#input_ident #ty_generics as SliceByValue>::Value;

                    #[inline]
                    fn len(&self) -> usize {
                        self.end - self.start
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueGet for SubsliceImpl<'__subslice_impl, #names> #where_clause  {
                    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
                        self.slice.get_value_unchecked(index + self.start)
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, '__subslice_gat, #params> SliceByValueSubsliceGat<'__subslice_gat> for SubsliceImpl<'__subslice_impl, #names> #where_clause {
                    type Subslice = SubsliceImpl<'__subslice_gat, #names>;
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::Range<usize>>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::Range<usize>,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.start + range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::RangeFrom<usize>>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeFrom<usize>,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::RangeToInclusive<usize>>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeToInclusive<usize>,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::RangeFull>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        _range: core::ops::RangeFull,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::RangeInclusive<usize>>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeInclusive<usize>,
                    ) -> Subslice<'_, Self> {
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
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::RangeTo<usize>>
                    for SubsliceImpl<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeTo<usize>,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end,
                        }
                    }
                }

                #[automatically_derived]
                pub struct Iter<'__subslice_impl, '__iter_ref, #params> {
                    subslice: &'__iter_ref SubsliceImpl<'__subslice_impl, #names>,
                    index: usize,
                }

                #[automatically_derived]
                impl<'__subslice_impl, '__iter_ref, #params> Iterator for Iter<'__subslice_impl, '__iter_ref, #names> #where_clause {
                    type Item = <#input_ident #ty_generics as SliceByValue>::Value;

                    #[inline]
                    fn next(&mut self) -> Option<Self::Item> {
                        if self.index < self.subslice.len() {
                            let value = unsafe { self.subslice.get_value_unchecked(self.index) };
                            self.index += 1;
                            Some(value)
                        } else {
                            None
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> IterableByValue for SubsliceImpl<'__subslice_impl, #names> #where_clause {
                    type Item = <#input_ident #ty_generics as SliceByValue>::Value;
                    type Iter<'__iter_ref>
                        = Iter<'__subslice_impl, '__iter_ref, #names>
                    where
                        Self: '__iter_ref;

                    #[inline]
                    fn iter_value(&self) -> Self::Iter<'_> {
                        Iter {
                            subslice: self,
                            index: 0,
                        }
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
                        if from > len {
                            panic!(
                                "index out of bounds: the len is {len} but the starting index is {from}"
                            );
                        }
                        Iter {
                            subslice: self,
                            index: from,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceGat<'__subslice_impl> for #input_ident #ty_generics #where_clause  {
                    type Subslice = SubsliceImpl<'__subslice_impl, #names>;
                }

                #[automatically_derived]
                impl #impl_generics SliceByValueSubsliceRange<core::ops::Range<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::Range<usize>,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: &self,
                            start: range.start,
                            end: range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics SliceByValueSubsliceRange<core::ops::RangeFrom<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeFrom<usize>,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: &self,
                            start: range.start,
                            end: self.len(),
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics SliceByValueSubsliceRange<core::ops::RangeToInclusive<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeToInclusive<usize>,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: &self,
                            start: 0,
                            end: range.end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics SliceByValueSubsliceRange<core::ops::RangeFull> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        _range: core::ops::RangeFull,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: &self,
                            start: 0,
                            end: self.len(),
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics SliceByValueSubsliceRange<core::ops::RangeInclusive<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeInclusive<usize>,
                    ) -> Subslice<'_, Self> {
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
                impl #impl_generics SliceByValueSubsliceRange<core::ops::RangeTo<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeTo<usize>,
                    ) -> Subslice<'_, Self> {
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
                impl<'__subslice_impl, #params> SliceByValue for SubsliceImplMut<'__subslice_impl, #names> #where_clause {
                    type Value = <#input_ident #ty_generics as SliceByValue>::Value;

                    #[inline]
                    fn len(&self) -> usize {
                        self.end - self.start
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueGet for SubsliceImplMut<'__subslice_impl, #names> #where_clause  {
                    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
                        self.slice.get_value_unchecked(index + self.start)
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, '__subslice_gat, #params> SliceByValueSubsliceGat<'__subslice_gat> for SubsliceImplMut<'__subslice_impl, #names> #where_clause {
                    type Subslice = SubsliceImpl<'__subslice_gat, #names>;
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::Range<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::Range<usize>,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.start + range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::RangeFrom<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeFrom<usize>,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::RangeToInclusive<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeToInclusive<usize>,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::RangeFull>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        _range: core::ops::RangeFull,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::RangeInclusive<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeInclusive<usize>,
                    ) -> Subslice<'_, Self> {
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
                impl<'__subslice_impl, #params> SliceByValueSubsliceRange<core::ops::RangeTo<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: core::ops::RangeTo<usize>,
                    ) -> Subslice<'_, Self> {
                        SubsliceImpl {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSet for SubsliceImplMut<'__subslice_impl, #names> #where_clause  {
                    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
                        self.slice.set_value_unchecked(index + self.start, value)
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueRepl for SubsliceImplMut<'__subslice_impl, #names> #where_clause  {
                    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
                        self.slice.replace_value_unchecked(index + self.start, value)
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, '__subslice_gat, #params> SliceByValueSubsliceGatMut<'__subslice_gat> for SubsliceImplMut<'__subslice_impl, #names> #where_clause {
                    type Subslice = SubsliceImplMut<'__subslice_gat, #names>;
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRangeMut<core::ops::Range<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: core::ops::Range<usize>,
                    ) -> SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.start + range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRangeMut<core::ops::RangeFrom<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: core::ops::RangeFrom<usize>,
                    ) -> SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self.slice,
                            start: self.start + range.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRangeMut<core::ops::RangeToInclusive<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: core::ops::RangeToInclusive<usize>,
                    ) -> SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRangeMut<core::ops::RangeFull>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        _range: core::ops::RangeFull,
                    ) -> SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self.slice,
                            start: self.start,
                            end: self.end,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceRangeMut<core::ops::RangeInclusive<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: core::ops::RangeInclusive<usize>,
                    ) -> SubsliceMut<'_, Self> {
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
                impl<'__subslice_impl, #params> SliceByValueSubsliceRangeMut<core::ops::RangeTo<usize>>
                    for SubsliceImplMut<'__subslice_impl, #names> #where_clause
                {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: core::ops::RangeTo<usize>,
                    ) -> SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self.slice,
                            start: self.start,
                            end: self.start + range.end,
                        }
                    }
                }
                // TODO: maybe referring directly to the slice?
                #[automatically_derived]
                pub struct IterMut<'__subslice_impl, '__iter_ref, #params> {
                    subslice: &'__iter_ref SubsliceImplMut<'__subslice_impl, #names>,
                    index: usize,
                }

                #[automatically_derived]
                impl<'__subslice_impl, '__iter_ref, #params> Iterator for IterMut<'__subslice_impl, '__iter_ref, #names> #where_clause {
                    type Item = <#input_ident #ty_generics as SliceByValue>::Value;

                    #[inline]
                    fn next(&mut self) -> Option<Self::Item> {
                        if self.index < self.subslice.len() {
                            let value = unsafe { self.subslice.get_value_unchecked(self.index) };
                            self.index += 1;
                            Some(value)
                        } else {
                            None
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> IterableByValue for SubsliceImplMut<'__subslice_impl, #names> #where_clause {
                    type Item = <#input_ident #ty_generics as SliceByValue>::Value;
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
                        if from > len {
                            panic!(
                                "index out of bounds: the len is {len} but the starting index is {from}"
                            );
                        }
                        IterMut {
                            subslice: self,
                            index: from,
                        }
                    }
                }

                #[automatically_derived]
                impl<'__subslice_impl, #params> SliceByValueSubsliceGatMut<'__subslice_impl> for #input_ident #ty_generics #where_clause  {
                    type Subslice = SubsliceImplMut<'__subslice_impl, #names>;
                }

                #[automatically_derived]
                impl #impl_generics SliceByValueSubsliceRangeMut<core::ops::Range<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: core::ops::Range<usize>,
                    ) -> SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self,
                            start: range.start,
                            end: range.end,
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics SliceByValueSubsliceRangeMut<core::ops::RangeFrom<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: core::ops::RangeFrom<usize>,
                    ) -> SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self,
                            start: range.start,
                            end: self.len(),
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics SliceByValueSubsliceRangeMut<core::ops::RangeToInclusive<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: core::ops::RangeToInclusive<usize>,
                    ) -> SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self,
                            start: 0,
                            end: range.end + 1,
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics SliceByValueSubsliceRangeMut<core::ops::RangeFull> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        _range: core::ops::RangeFull,
                    ) -> SubsliceMut<'_, Self> {
                        SubsliceImplMut {
                            slice: self,
                            start: 0,
                            end: self.len(),
                        }
                    }
                }

                #[automatically_derived]
                impl #impl_generics SliceByValueSubsliceRangeMut<core::ops::RangeInclusive<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: core::ops::RangeInclusive<usize>,
                    ) -> SubsliceMut<'_, Self> {
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
                impl #impl_generics SliceByValueSubsliceRangeMut<core::ops::RangeTo<usize>> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: core::ops::RangeTo<usize>,
                    ) -> SubsliceMut<'_, Self> {
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
