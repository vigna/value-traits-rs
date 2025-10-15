# Change Log

## [0.2.0]

### New

* Radical trait simplification: `SliceByValue` now contains the former
  `SliceByValueCore`; `SliceByValueSet` and `SliceByValueRepl` have been merged
  into `SliceByValueMut`.

* Several new default (mostly default) methods in `SliceByValueMut`, borrowed
  from the `BitFieldSlice` trait of `sux`.

### Changed

* We moved to the 2024 edition.

### Fixed

* `VecDeque` implementations now are gated by the `std` feature, rather than
  the `alloc` feature.

## [0.1.4] 2025-06-21

### New

* Implemented access and iteration traits for `VecDeque`.

### Fixed

* `IterateByValueFrom` delegations were missing a `?Sized`.

## [0.1.3] 2025-06-20

### New

* Four attributes makes it possible to add trait bounds to the
  derive-based implementations of subslices and iterators.

### Improved

* Derive macros are now re-exported from the `value-traits` crate.

## [0.1.2] 2025-06-18

### New

* First release.
