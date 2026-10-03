/// A max function for f64's without NaNs
pub fn max(vals: &[f64]) -> f64 {
    if vals.len() == 1 {
        // SAFETY: length checked above.
        return unsafe { *vals.get_unchecked(0) };
    }
    let mut result = vals[0];
    for &v in &vals[1..] {
        if v.partial_cmp(&result).unwrap() == std::cmp::Ordering::Greater {
            result = v;
        }
    }
    result
}

/// A min function for f64's without NaNs
pub fn min(vals: &[f64]) -> f64 {
    if vals.len() == 1 {
        // SAFETY: length checked above.
        return unsafe { *vals.get_unchecked(0) };
    }
    let mut result = vals[0];
    for &v in &vals[1..] {
        if v.partial_cmp(&result).unwrap() == std::cmp::Ordering::Less {
            result = v;
        }
    }
    result
}

#[test]
fn test_max() {
    let assert_float_eq = |a: f64, b: f64| {
        assert!((a - b).abs() < f64::EPSILON);
    };

    assert_float_eq(1.0, max(&[1.0]));
    assert_float_eq(-1.0, max(&[-1.0]));
    assert_float_eq(-1.0, max(&[-2.0, -1.0]));
    assert_float_eq(1.0, max(&[-1.0, 1.0]));
    assert_float_eq(1.0, max(&[-1.0, 1.0, 0.0]));
}
