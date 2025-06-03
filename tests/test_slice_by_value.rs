/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use value_traits::slices::*;

const EXPECTED: [i32; 5] = [1, 2, 3, 4, 5];

#[test]
fn test_array() {
    generic_get(EXPECTED, &EXPECTED);
    generic_slice(EXPECTED, &EXPECTED);
    generic_mut(EXPECTED);
    generic_slice_mut(EXPECTED);
}

#[test]
fn test_slice() {
    generic_get(EXPECTED.as_slice(), &EXPECTED);
    generic_slice(EXPECTED.as_slice(), &EXPECTED);
}

#[test]
fn test_slice_mut() {
    let mut x = EXPECTED.to_vec();
    generic_get(x.as_mut_slice(), &EXPECTED);
    generic_slice(x.as_mut_slice(), &EXPECTED);
    generic_mut(x.as_mut_slice());
    generic_slice_mut(x.as_mut_slice());
}

#[test]
#[cfg(feature = "alloc")]
fn test_vecs() {
    generic_get(EXPECTED.to_vec(), &EXPECTED);
    generic_slice(EXPECTED.to_vec(), &EXPECTED);
    generic_mut(EXPECTED.to_vec());
    generic_slice_mut(EXPECTED.to_vec());
}

#[test]
#[cfg(feature = "std")]
fn test_rc() {
    use std::rc::Rc;
    let x = <Rc<[i32]>>::from(EXPECTED);
    generic_get(x.clone(), &EXPECTED);
    generic_slice(x.clone(), &EXPECTED);
    // no muts
}

#[test]
#[cfg(feature = "std")]
fn test_arc() {
    use std::sync::Arc;
    let x = <Arc<[i32]>>::from(EXPECTED);
    generic_get(x.clone(), &EXPECTED);
    generic_slice(x.clone(), &EXPECTED);
    // no muts
}

#[test]
#[cfg(feature = "alloc")]
fn test_boxed_slice() {
    let x = EXPECTED.to_vec().into_boxed_slice();
    generic_get(x.clone(), &EXPECTED);
    generic_slice(x.clone(), &EXPECTED);
    generic_mut(x.clone());
    generic_slice_mut(x.clone());
}

fn generic_get<S>(s: S, expected: &[i32])
where
    S: SliceByValue<Value = i32> + SliceByValueGet,
{
    assert_eq!(SliceByValue::len(&s), expected.len());

    for i in 0..expected.len() {
        assert_eq!(SliceByValueGet::get_value(&s, i), Some(expected[i]));
        assert_eq!(SliceByValueGet::get_value(&s, i + expected.len()), None);

        assert_eq!(SliceByValueGet::index_value(&s, i), expected[i]);

        unsafe {
            assert_eq!(SliceByValueGet::get_value_unchecked(&s, i), expected[i]);
        }
    }
}

fn generic_slice<S>(s: S, expected: &[i32])
where
    S: SliceByValue<Value = i32>
        + SliceByValueGet
        + SliceByValueSubslice
        + for<'b> SliceByValueSubsliceGat<'b, Subslice = &'b [i32]>,
{
    assert_eq!(SliceByValue::len(&s), expected.len());

    let subslice = s.index_subslice(0..2);
    assert_eq!(subslice, &expected[0..2]);

    let subslice = s.index_subslice(1..3);
    assert_eq!(subslice, &expected[1..3]);

    assert_eq!(s.index_subslice(1..).index_subslice(..2), [2, 3].as_ref());

    let r = ..;
    generic_get(s.index_subslice(r), expected);
    generic_get(s.get_subslice(r).unwrap(), expected);
    generic_get(unsafe { s.get_subslice_unchecked(r) }, expected);

    let r = 1..;
    generic_get(s.index_subslice(r.clone()), &expected[r.clone()]);
    generic_get(s.get_subslice(r.clone()).unwrap(), &expected[r.clone()]);
    generic_get(
        unsafe { s.get_subslice_unchecked(r.clone()) },
        &expected[r.clone()],
    );

    let r = 1..4;
    generic_get(s.index_subslice(r.clone()), &expected[r.clone()]);
    generic_get(s.get_subslice(r.clone()).unwrap(), &expected[r.clone()]);
    generic_get(
        unsafe { s.get_subslice_unchecked(r.clone()) },
        &expected[r.clone()],
    );

    let r = ..3;
    generic_get(s.index_subslice(r.clone()), &expected[r.clone()]);
    generic_get(s.get_subslice(r.clone()).unwrap(), &expected[r.clone()]);
    generic_get(
        unsafe { s.get_subslice_unchecked(r.clone()) },
        &expected[r.clone()],
    );

    let r = ..=3;
    generic_get(s.index_subslice(r.clone()), &expected[r.clone()]);
    generic_get(s.get_subslice(r.clone()).unwrap(), &expected[r.clone()]);
    generic_get(
        unsafe { s.get_subslice_unchecked(r.clone()) },
        &expected[r.clone()],
    );

    let r = 1..=4;
    generic_get(s.index_subslice(r.clone()), &expected[r.clone()]);
    generic_get(s.get_subslice(r.clone()).unwrap(), &expected[r.clone()]);
    generic_get(
        unsafe { s.get_subslice_unchecked(r.clone()) },
        &expected[r.clone()],
    );
}

fn generic_mut<S>(mut s: S)
where
    S: SliceByValue<Value = i32> + SliceByValueGet + SliceByValueSet + SliceByValueRepl,
{
    for i in 0..s.len() {
        let old_value = SliceByValueGet::index_value(&s, i);
        let new_value = old_value + 1;

        // Test set_value
        SliceByValueSet::set_value(&mut s, i, new_value);
        assert_eq!(SliceByValueGet::index_value(&s, i), new_value);

        // Test replace_value
        let replaced_value = SliceByValueRepl::replace_value(&mut s, i, old_value);
        assert_eq!(replaced_value, new_value);
        assert_eq!(SliceByValueGet::index_value(&s, i), old_value);

        // test replace_value_unchecked
        let replaced_value =
            unsafe { SliceByValueRepl::replace_value_unchecked(&mut s, i, new_value) };
        assert_eq!(replaced_value, old_value);
        assert_eq!(SliceByValueGet::index_value(&s, i), new_value);

        // Test unsafe set_value_unchecked
        unsafe {
            SliceByValueSet::set_value_unchecked(&mut s, i, new_value);
            assert_eq!(SliceByValueGet::index_value(&s, i), new_value);
        }

        s.set_value(i, old_value);
    }
}

fn generic_slice_mut<S>(mut s: S)
where
    S: SliceByValue<Value = i32>
        + SliceByValueGet
        + SliceByValueSet
        + SliceByValueRepl
        + SliceByValueSubsliceMut
        + for<'b> SliceByValueSubsliceGatMut<'b, Subslice = &'b mut [i32]>,
{
    let subslice = s.index_subslice_mut(0..2);
    assert_eq!(subslice, &mut [1, 2]);

    let r = 1..3;
    let subslice = s.index_subslice_mut(r.clone());
    assert_eq!(subslice, &mut [2, 3]);

    let r = ..;
    generic_mut(s.index_subslice_mut(r));
    generic_mut(s.get_subslice_mut(r).unwrap());
    generic_mut(unsafe { s.get_subslice_unchecked_mut(r) });

    let r = 1..;
    generic_mut(s.index_subslice_mut(r.clone()));
    generic_mut(s.get_subslice_mut(r.clone()).unwrap());
    generic_mut(unsafe { s.get_subslice_unchecked_mut(r.clone()) });

    let r = 1..4;
    generic_mut(s.index_subslice_mut(r.clone()));
    generic_mut(s.get_subslice_mut(r.clone()).unwrap());
    generic_mut(unsafe { s.get_subslice_unchecked_mut(r.clone()) });

    let r = ..3;
    generic_mut(s.index_subslice_mut(r.clone()));
    generic_mut(s.get_subslice_mut(r.clone()).unwrap());
    generic_mut(unsafe { s.get_subslice_unchecked_mut(r.clone()) });

    let r = ..=3;
    generic_mut(s.index_subslice_mut(r.clone()));
    generic_mut(s.get_subslice_mut(r.clone()).unwrap());
    generic_mut(unsafe { s.get_subslice_unchecked_mut(r.clone()) });

    let r = 1..=4;
    generic_mut(s.index_subslice_mut(r.clone()));
    generic_mut(s.get_subslice_mut(r.clone()).unwrap());
    generic_mut(unsafe { s.get_subslice_unchecked_mut(r.clone()) });
}
