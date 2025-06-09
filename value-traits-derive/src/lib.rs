/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Inria
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// A procedural macro fully implementing subslices on top of a
/// [`SliceByValueGet`].
///
/// The macro defines a structure `SubsliceImpl` that keeps track of a reference
/// to a slice, and of the start and end of the subslice. `SubsliceImpl` then
/// implements [`SliceByValueGet`] and [`SliceByValueSubslice`]. Finally, a
/// structure `Iter` is used to implement
/// [`IterableByValue`](crate::iter::IterableByValue) on `SubsliceImpl`.
#[proc_macro_derive(Subslices)]
pub fn subslices(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (_impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = &input.generics.params;

    match input.data {
        Data::Struct(_) => {
            quote! {
                #[automatically_derived]
                impl<'__subslice_impl, #params> value_traits::slices::SliceByValueSubsliceGat<'__subslice_impl> for #input_ident #ty_generics #where_clause  {
                    type Subslice = value_traits::helpers::SubsliceImpl<'__subslice_impl, Self>;
                }

                #[automatically_derived]
                impl<#params, R: core::ops::RangeBounds<usize> + value_traits::slices::RangeCheck> value_traits::slices::SliceByValueSubsliceRange<R> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked(
                        &self,
                        range: R,
                    ) -> value_traits::slices::Subslice<'_, Self> {
                        let base = 0..self.len();
                        value_traits::helpers::SubsliceImpl::new(
                            &self,
                            value_traits::helpers::range_compose(&base, range),
                        )
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
/// [`SliceByValueSubsliceMut`]. Finally, a structure `IterMut` is used to
/// implement [`IterableByValue`](crate::iter::IterableByValue) on
/// `SubsliceImplMut`.
///
/// Note that since `IterMut` provides iterators by value, it cannot use to
/// mutate the subslice. Moreover, non-mutable subslicing on a
/// `SubsliceImplMut` will yield a `SubsliceImpl`.
#[proc_macro_derive(SubslicesMut)]
pub fn subslices_mut(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let input_ident = input.ident;
    input.generics.make_where_clause();
    let (_impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let params = &input.generics.params;

    match input.data {
        Data::Struct(_) => {
            quote! {
                #[automatically_derived]
                impl<'__subslice_impl, #params> value_traits::slices::SliceByValueSubsliceGatMut<'__subslice_impl> for #input_ident #ty_generics #where_clause  {
                    type Subslice = value_traits::helpers::SubsliceImplMut<'__subslice_impl, Self>;
                }

                #[automatically_derived]
                impl<#params, R: core::ops::RangeBounds<usize> + value_traits::slices::RangeCheck> value_traits::slices::SliceByValueSubsliceRangeMut<R> for #input_ident #ty_generics #where_clause {
                    unsafe fn get_subslice_unchecked_mut(
                        &mut self,
                        range: R,
                    ) -> value_traits::slices::SubsliceMut<'_, Self> {
                        let base = 0..self.len();
                        value_traits::helpers::SubsliceImplMut::new(
                            self,
                            value_traits::helpers::range_compose(&base, range),
                        )
                    }
                }
            }
        },

        _ => unimplemented!(),
    }
    .into()
}
