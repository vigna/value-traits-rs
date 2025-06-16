/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Inria
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! Procedural macros for the [`value-traits`](https://docs.rs/value-traits/latest/value_traits/) crate.

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, parse_macro_input, AngleBracketedGenericArguments, DeriveInput};

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

/// A procedural macro fully implementing subslices on top of a
/// [`SliceByValueGet`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueGet.html).
///
/// The macro defines a structure `<YOUR TYPE>SubsliceImpl` that keeps track of
/// a reference to a slice, and of the start and end of the subslice.
/// `<YOUR TYPE>SubsliceImpl` then implements
/// [`SliceByValueGet`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueGet.html)
/// and
/// [`SliceByValueSubslice`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSubslice.html).
///
/// Note that since this macro has no knowledge of the bounds of the generic
/// parameters in the implementations of
/// [`SliceByValue`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValue.html)
/// and
/// [`SliceByValueGet`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueGet.html),
/// such bounds must appear in the definition of the type.
#[proc_macro_derive(Subslices)]
pub fn subslices(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = &input.generics.params;
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
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueGet for #subslice_impl<'__subslice_impl, #names> #where_clause  {
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
        quote! { core::ops::Range<usize> },
        quote! { core::ops::RangeFrom<usize> },
        quote! { core::ops::RangeToInclusive<usize> },
        quote! { core::ops::RangeFull },
        quote! { core::ops::RangeInclusive<usize> },
        quote! { core::ops::RangeTo<usize> },
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

/// A procedural macro fully implementing mutable subslices on top of a
/// [`SliceByValueSet`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSet.html)/[`SliceByValueRepl`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueRepl.html)
/// for which the derive macro [`Subslices`] has been already applied.
///
/// The macro defines a structure `<YOUR TYPE>SubsliceImplMut` that keeps track
/// of a mutable reference to a slice, and of the start and end of the subslice.
/// `<YOUR TYPE>SubsliceImplMut` then implements
/// [`SliceByValueGet`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueGet.html),
/// [`SliceByValueSet`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSet.html),
/// [`SliceByValueRepl`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueRepl.html),
/// [`SliceByValueSubslice`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSubslice.html),
/// and
/// [`SliceByValueSubsliceMut`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSubsliceMut.html).
///
/// Note that
/// [`SliceByValueSubslice`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSubslice.html)
/// methods will return the `<YOUR TYPE>SubsliceImpl` structure generated by the
/// [`Subslices`] macro.
///
/// Moreover, since this macro has no knowledge of the bounds of the generic
/// parameters in the implementations of
/// [`SliceByValueSet`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSet.html)
/// and
/// [`SliceByValueRepl`](https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueRepl.html),
/// such bounds must appear in the definition of the type.
#[proc_macro_derive(SubslicesMut)]
pub fn subslices_mut(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = &input.generics.params;
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
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueGet for #subslice_impl_mut<'__subslice_impl, #names> #where_clause  {
            unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
                self.slice.get_value_unchecked(index + self.range.start)
            }
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSet for #subslice_impl_mut<'__subslice_impl, #names> #where_clause  {
            unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
                self.slice.set_value_unchecked(index + self.range.start, value)
            }
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueRepl for #subslice_impl_mut<'__subslice_impl, #names> #where_clause  {
            unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
                self.slice.replace_value_unchecked(index + self.range.start, value)
            }
        }

        #[automatically_derived]
        impl<'__subslice_impl, '__subslice_gat, #params> ::value_traits::slices::SliceByValueSubsliceGat<'__subslice_gat> for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            type Subslice = #subslice_impl<'__subslice_gat, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, '__subslice_gat, #params> ::value_traits::slices::SliceByValueSubsliceGatMut<'__subslice_gat> for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            type Subslice = #subslice_impl_mut<'__subslice_gat, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::slices::SliceByValueSubsliceGatMut<'__subslice_impl> for #input_ident #ty_generics #where_clause  {
            type Subslice = #subslice_impl_mut<'__subslice_impl, #names>;
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

/// A procedural macro fully implementing
/// [`IterableByValue`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterableByValue.html)
/// and
/// [`IterableByValueFrom`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterableByValueFrom.html)
/// for subslices on top of a the `<YOUR TYPE>SubsliceImpl` structure generated
/// by the derive macro [`Subslices`].
///
/// The macro defines a structure `<YOUR TYPE>Iter` that keeps track of a
/// mutable reference to a slice and of a current iteration range; the structure
/// is used to implement
/// [`IterableByValue`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterableByValue.html)
/// and
/// [`IterableByValueFrom`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterableByValueFrom.html)
/// on `<YOUR TYPE>SubsliceImpl`.
#[proc_macro_derive(Iterators)]
pub fn iterators(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (_impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = &input.generics.params;
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
        impl<#params> ::value_traits::iter::IterableByValue for #input_ident #ty_generics #where_clause {
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
        impl<#params> ::value_traits::iter::IterableByValueFrom for #input_ident #ty_generics #where_clause {
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
                if n >= self.range.end {
                    self.range.start = self.range.end; // consume the ::core::iter::iterator
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
        }

        impl<'__iter_ref, #params> ::core::iter::ExactSizeIterator for #iter<'__iter_ref, #names> #where_clause {
            #[inline]
            fn len(&self) -> usize {
                self.range.len()
            }
        }

        #[automatically_derived]
        impl<'__subslice_impl, '__iter_ref, #params> ::value_traits::iter::IterableByValueGat<'__iter_ref> for #subslice_impl<'__subslice_impl, #names> #where_clause {
            type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;
            type Iter = #iter<'__iter_ref, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::iter::IterableByValue for #subslice_impl<'__subslice_impl, #names> #where_clause {
            #[inline]
            fn iter_value(&self) -> ::value_traits::iter::Iter<'_, Self> {
                #iter::new(self.slice)
            }
        }

        #[automatically_derived]
        impl<'__subslice_impl, '__iter_ref,#params> ::value_traits::iter::IterableByValueFromGat<'__iter_ref> for #subslice_impl<'__subslice_impl, #names> #where_clause {
            type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;
            type IterFrom = #iter<'__iter_ref, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::iter::IterableByValueFrom for #subslice_impl<'__subslice_impl, #names> #where_clause {
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

/// A procedural macro that implements
/// [`IterableByValue`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterableByValue.html)
/// and
/// [`IterableByValueFrom`](https://docs.rs/value-traits/latest/value_traits/iter/trait.IterableByValueFrom.html)
/// for mutable subslices on top of the `<YOUR TYPE>SubsliceImplMut` structure
/// generated by the derive macro [`SubslicesMut`].
///
/// To call this macro, you first need to derive both [`SubslicesMut`] and
/// [`Iterators`] on the same struct, as this macro uses the `<YOUR TYPE>Iter`
/// structure defined by [`Iterators`].
#[proc_macro_derive(IteratorsMut)]
pub fn iterators_mut(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (_impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = &input.generics.params;
    let ty_generics_token_stream = ty_generics.clone().into_token_stream();

    let names = get_names(ty_generics_token_stream);
    let subslice_impl_mut = quote::format_ident!("{}SubsliceImplMut", input_ident);
    let iter = quote::format_ident!("{}Iter", input_ident);
    quote!{
        #[automatically_derived]
        impl<'__subslice_impl, '__iter_ref, #params> ::value_traits::iter::IterableByValueGat<'__iter_ref> for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;
            type Iter = #iter<'__iter_ref, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::iter::IterableByValue for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            fn iter_value(&self) -> ::value_traits::iter::Iter<'_, Self> {
                #iter::new(self.slice)
            }
        }

        #[automatically_derived]
        impl<'__subslice_impl, '__iter_ref, #params> ::value_traits::iter::IterableByValueFromGat<'__iter_ref> for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            type Item = <#input_ident #ty_generics as ::value_traits::slices::SliceByValue>::Value;
            type IterFrom = #iter<'__iter_ref, #names>;
        }

        #[automatically_derived]
        impl<'__subslice_impl, #params> ::value_traits::iter::IterableByValueFrom for #subslice_impl_mut<'__subslice_impl, #names> #where_clause {
            fn iter_value_from(&self, from: usize) -> ::value_traits::iter::IterFrom<'_, Self> {
                let len = self.len();
                assert!(from <= len, "index out of bounds: the len is {len} but the starting index is {from}");
                let range = ::value_traits::slices::ComposeRange::compose(&(from..), self.range.clone());
                #iter::new_with_range(self.slice, range)
            }
        }
    }.into()
}
