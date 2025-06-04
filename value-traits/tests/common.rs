/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use value_traits::slices::*;

pub fn generic_get<S>(s: S, expected: &[i32])
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

pub fn generic_slice<S>(s: S, expected: &[i32])
where
    S: SliceByValue<Value = i32> + SliceByValueGet + SliceByValueSubslice,
{
    assert_eq!(SliceByValue::len(&s), expected.len());

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

pub fn generic_mut<S>(mut s: S)
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

pub fn generic_slice_mut<S>(mut s: S)
where
    S: SliceByValue<Value = i32>
        + SliceByValueGet
        + SliceByValueSet
        + SliceByValueRepl
        + SliceByValueSubsliceMut,
    for<'a> <S as SliceByValueSubsliceGatMut<'a>>::Subslice:
        SliceByValue<Value = i32> + SliceByValueGet + SliceByValueSet + SliceByValueRepl,
{
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

    assert!(s.get_subslice_mut(1..usize::MAX).is_none());
    assert!(s.get_subslice_mut(1..=usize::MAX).is_none());
    assert!(s.get_subslice_mut(..=usize::MAX).is_none());
    assert!(s.get_subslice_mut(..usize::MAX).is_none());
    assert!(s.get_subslice_mut(usize::MAX..).is_none());
}
