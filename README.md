# Traits By Value

[![downloads](https://img.shields.io/crates/d/value-traits)](https://crates.io/crates/value-traits)
[![dependents](https://img.shields.io/librariesio/dependents/cargo/value-traits)](https://crates.io/crates/value-traits/reverse_dependencies)
![GitHub CI](https://github.com/vigna/value-traits-rs/actions/workflows/rust.yml/badge.svg)
![license](https://img.shields.io/crates/l/value-traits)
[![Latest version](https://img.shields.io/crates/v/value-traits.svg)](https://crates.io/crates/value-traits)
[![Documentation](https://docs.rs/value-traits/badge.svg)](https://docs.rs/value-traits)
[![Coverage Status](https://coveralls.io/repos/github/vigna/value-traits-rs/badge.svg?branch=main)](https://coveralls.io/github/vigna/value-traits-rs?branch=main)

## Why

Slices are one of the most pervasive types in Rust—and with good reasons. They
are lightweight, flexible, and represent a basic data structure, the _sequence_,
AKA _random-access list_.

The problem with slices is that, by design, they are accessed by _reference_:
the most used slice trait, [`Index`], gives a reference to an element of the
slice. While this approach is elegant and makes several compiler optimizations
possible, it also means that slices cannot be used as generic random-access
lists, as access by reference means there must be an actual contiguous segment
of memory locations containing explicit representations
of the elements of the lists. However, there are also different list
representations, such as compressed, succinct, functional, implicit, and so on.

For this reason, this crate provides traits parallel to slices and iterators,
but by value, rather than by reference. [`SliceByValue`] simply specify the type
of values of the slice by value and its length. A [`SliceByValueGet`] provides
methods that are exactly analogous of [`std::slice::get`] and [`Index::index`],
but return values, rather than references, and are named [`get_value`] and
[`index_value`] instead. The longer names are necessary to avoid ambiguity, as
all slices of cloneable elements implement our by-value traits. It might be argued
`RandomAccessList` or `Sequence` might be more standard name, but we want to
underline the fact that the read access is closely modeled after slices. Note
that we cannot overload the `[]` operator, as [`Index`] methods must necessarily
return references.

Write access is a different issue because [`SliceByValueSet`] has necessarily a
completely different setup than [`IndexMut::index_mut`]. We also have a
[`SliceByValueReplace`] trait whose setter return the original value, which
might be more efficient in some circumstances.

Finally, like slices, slices by value can provide [subslicing]. Subslicing
traits are distinct traits, as you might be contented, for your application, of
(possibly read-only) access to single elements. Similarly to the access to
single elements, you have methods such as [`get_subslice`] and
[`index_subslice`], which have the same semantics as the corresponding methods
of slices.

The other missing trait contained in this crate is [`IterableByValue`], which
has the same logic for iterators. Rust has presently no trait specifying
that you can iterate by value on some structure without consuming it
as [`IntoIterator`] does. What one usually does is to implement [`IntoIterator`]
on a reference, providing an iterator on references on the element, which
brings back the problem of constraining such implementations to explicit
representations. While it is possible to implement [`IntoIterator`] in such
a way to return values, slices, vectors, etc., already have implementations
returning references, so a different trait is necessary.

Implementing subslices is tricky, so [`Subslices`] is a procedural macro that
provides a complete implementation of subslicing for a type that implements
[`SliceByValueGet`]; [`SubslicesMut`] similarly provides a complete
implementation of subslicing for a type that implements [`SliceByValueSet`] and
[`SliceByValueReplace`]. Note that a custom implementation might be more
efficient if your type can directly represent an inner range. Analogous
procedural macros [`Iterators`] and  [`IteratorsMut`] implement the by-value
iteration traits for the structures created by [`Subslices`] and
[`SubslicesMut`]. All these procedural macros are independent to make
specialized, more efficient implementation possible at every step.

One important difference with slices is that iterating subslicing will lead
to different types. We could not find any way to express in the current Rust
type system the recursive bound that subslices of a subslice should be of
the same type. This is not relevant if you pass the subslice to a function
that accepts a by-value slice, but it is relevant if you want to assign
subslices of different depth to the same variable.

[`SliceByValue`]: <https://docs.rs/value_traits/latest/value_traits/slices/trait.SliceByValue.html>
[`SliceByValueGet`]: <https://docs.rs/value_traits/latest/value_traits/slices/trait.SliceByValueGet.html>
[`SliceByValueSet`]: <https://docs.rs/value_traits/latest/value_traits/slices/trait.SliceByValueSet.html>
[`SliceByValueReplace`]: <https://docs.rs/value_traits/latest/value_traits/slices/trait.SliceByValueReplace.html>
[subslicing]: <https://docs.rs/value-traits/latest/value_traits/slices/trait.SliceByValueSubslice.html>
[`get_value`]: <https://docs.rs/value_traits/latest/value_traits/slices/trait.SliceByValueGet.html#tymethod.get_value>
[`index_value`]: <https://docs.rs/value_traits/latest/value_traits/slices/trait.SliceByValueGet.html#tymethod.index_value>
[`get_subslice`]: <https://docs.rs/value_traits/latest/value_traits/slices/trait.SliceByValueSubslice.html#tymethod.get_subslice>
[`index_subslice`]: <https://docs.rs/value_traits/latest/value_traits/slices/trait.SliceByValueSubslice.html#tymethod.index_subslice>
[`IterableByValue`]: <https://docs.rs/value_traits/latest/value_traits/iter/trait.IterableByValue.html>
[`IntoIterator`]: <https://doc.rust-lang.org/std/iter/trait.IntoIterator.html>
[`std::slice::get`]: <https://doc.rust-lang.org/std/slice/trait.SliceIndex.html#tymethod.get>
[`Index::index`]: <https://doc.rust-lang.org/std/ops/trait.Index.html#tymethod.index>
[`Index`]: <https://doc.rust-lang.org/std/ops/trait.Index.html>
[`IndexMut::index_mut`]: <https://doc.rust-lang.org/std/ops/trait.Index.html#tymethod.index_mut>
[`Subslices`]: <https://docs.rs/value_traits_derive/latest/value_traits_derive/derive.Subslices.html>
[`SubslicesMut`]: <https://docs.rs/value_traits_derive/latest/value_traits_derive/derive.SubslicesMut.html>
[`Iterators`]: <https://docs.rs/value_traits_derive/latest/value_traits_derive/derive.Iterators.html>
[`IteratorsMut`]: <https://docs.rs/value_traits_derive/latest/value_traits_derive/derive.IteratorsMut.html>
