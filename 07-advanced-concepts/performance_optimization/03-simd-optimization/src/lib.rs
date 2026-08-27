use std::simd::{f32x4, f32x8, num::SimdFloat};

// Scalar dot product
pub fn dot_product_scalar(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .sum()
}

// SIMD dot product (4-wide)
pub fn dot_product_simd4(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());

    let chunks = a.len() / 4;
    let mut sum = f32x4::splat(0.0);

    for i in 0..chunks {
        let a_vec = f32x4::from_slice(&a[i * 4..]);
        let b_vec = f32x4::from_slice(&b[i * 4..]);
        sum += a_vec * b_vec;
    }

    let result = sum.reduce_sum();

    let remainder: f32 = a[chunks * 4..]
        .iter()
        .zip(&b[chunks * 4..])
        .map(|(x, y)| x * y)
        .sum();

    result + remainder
}

// SIMD dot product (8-wide)
pub fn dot_product_simd8(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());

    let chunks = a.len() / 8;
    let mut sum = f32x8::splat(0.0);

    for i in 0..chunks {
        let a_vec = f32x8::from_slice(&a[i * 8..]);
        let b_vec = f32x8::from_slice(&b[i * 8..]);
        sum += a_vec * b_vec;
    }

    let result = sum.reduce_sum();

    let remainder: f32 = a[chunks * 8..]
        .iter()
        .zip(&b[chunks * 8..])
        .map(|(x, y)| x * y)
        .sum();

    result + remainder
}

// Vector addition - scalar
pub fn vec_add_scalar(a: &[f32], b: &[f32]) -> Vec<f32> {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x + y)
        .collect()
}

// Vector addition - SIMD
pub fn vec_add_simd(a: &[f32], b: &[f32]) -> Vec<f32> {
    assert_eq!(a.len(), b.len());

    let mut result = vec![0.0; a.len()];
    let chunks = a.len() / 8;

    for i in 0..chunks {
        let a_vec = f32x8::from_slice(&a[i * 8..]);
        let b_vec = f32x8::from_slice(&b[i * 8..]);
        let sum = a_vec + b_vec;
        sum.copy_to_slice(&mut result[i * 8..]);
    }

    for i in chunks * 8..a.len() {
        result[i] = a[i] + b[i];
    }

    result
}

// Sum of squares - scalar
pub fn sum_of_squares_scalar(data: &[f32]) -> f32 {
    data.iter().map(|&x| x * x).sum()
}

// Sum of squares - SIMD
pub fn sum_of_squares_simd(data: &[f32]) -> f32 {
    let chunks = data.len() / 8;
    let mut sum = f32x8::splat(0.0);

    for i in 0..chunks {
        let vec = f32x8::from_slice(&data[i * 8..]);
        sum += vec * vec;
    }

    let result = sum.reduce_sum();

    let remainder: f32 = data[chunks * 8..]
        .iter()
        .map(|&x| x * x)
        .sum();

    result + remainder
}
