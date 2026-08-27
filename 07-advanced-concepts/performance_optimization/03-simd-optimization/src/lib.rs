// Note: This is a stable Rust compatible version demonstrating performance optimization concepts
// The original used std::simd which is unstable; this version uses well-optimized scalar operations
// For actual SIMD on stable, consider using the `packed_simd` or `wide` crates

// Scalar dot product
pub fn dot_product_scalar(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .sum()
}

// Optimized dot product with manual loop unrolling
pub fn dot_product_simd4(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());

    let chunks = a.len() / 4;
    let mut sum0 = 0.0;
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    let mut sum3 = 0.0;

    for i in 0..chunks {
        let idx = i * 4;
        sum0 += a[idx] * b[idx];
        sum1 += a[idx + 1] * b[idx + 1];
        sum2 += a[idx + 2] * b[idx + 2];
        sum3 += a[idx + 3] * b[idx + 3];
    }

    let mut result = sum0 + sum1 + sum2 + sum3;

    let remainder: f32 = a[chunks * 4..]
        .iter()
        .zip(&b[chunks * 4..])
        .map(|(x, y)| x * y)
        .sum();

    result + remainder
}

// Optimized dot product with 8-way loop unrolling
pub fn dot_product_simd8(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());

    let chunks = a.len() / 8;
    let mut sum0 = 0.0;
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    let mut sum3 = 0.0;
    let mut sum4 = 0.0;
    let mut sum5 = 0.0;
    let mut sum6 = 0.0;
    let mut sum7 = 0.0;

    for i in 0..chunks {
        let idx = i * 8;
        sum0 += a[idx] * b[idx];
        sum1 += a[idx + 1] * b[idx + 1];
        sum2 += a[idx + 2] * b[idx + 2];
        sum3 += a[idx + 3] * b[idx + 3];
        sum4 += a[idx + 4] * b[idx + 4];
        sum5 += a[idx + 5] * b[idx + 5];
        sum6 += a[idx + 6] * b[idx + 6];
        sum7 += a[idx + 7] * b[idx + 7];
    }

    let mut result = sum0 + sum1 + sum2 + sum3 + sum4 + sum5 + sum6 + sum7;

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

// Vector addition - optimized with loop unrolling
pub fn vec_add_simd(a: &[f32], b: &[f32]) -> Vec<f32> {
    assert_eq!(a.len(), b.len());

    let mut result = vec![0.0; a.len()];
    let chunks = a.len() / 8;

    for i in 0..chunks {
        let idx = i * 8;
        result[idx] = a[idx] + b[idx];
        result[idx + 1] = a[idx + 1] + b[idx + 1];
        result[idx + 2] = a[idx + 2] + b[idx + 2];
        result[idx + 3] = a[idx + 3] + b[idx + 3];
        result[idx + 4] = a[idx + 4] + b[idx + 4];
        result[idx + 5] = a[idx + 5] + b[idx + 5];
        result[idx + 6] = a[idx + 6] + b[idx + 6];
        result[idx + 7] = a[idx + 7] + b[idx + 7];
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

// Sum of squares - optimized with loop unrolling
pub fn sum_of_squares_simd(data: &[f32]) -> f32 {
    let chunks = data.len() / 8;
    let mut sum0 = 0.0;
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    let mut sum3 = 0.0;
    let mut sum4 = 0.0;
    let mut sum5 = 0.0;
    let mut sum6 = 0.0;
    let mut sum7 = 0.0;

    for i in 0..chunks {
        let idx = i * 8;
        sum0 += data[idx] * data[idx];
        sum1 += data[idx + 1] * data[idx + 1];
        sum2 += data[idx + 2] * data[idx + 2];
        sum3 += data[idx + 3] * data[idx + 3];
        sum4 += data[idx + 4] * data[idx + 4];
        sum5 += data[idx + 5] * data[idx + 5];
        sum6 += data[idx + 6] * data[idx + 6];
        sum7 += data[idx + 7] * data[idx + 7];
    }

    let mut result = sum0 + sum1 + sum2 + sum3 + sum4 + sum5 + sum6 + sum7;

    for i in chunks * 8..data.len() {
        result += data[i] * data[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![5.0, 6.0, 7.0, 8.0];

        assert_eq!(dot_product_scalar(&a, &b), 70.0);
        assert_eq!(dot_product_simd4(&a, &b), 70.0);
        assert_eq!(dot_product_simd8(&a, &b), 70.0);
    }

    #[test]
    fn test_vec_add() {
        let a = vec![1.0, 2.0, 3.0, 4.0];
        let b = vec![5.0, 6.0, 7.0, 8.0];
        let expected = vec![6.0, 8.0, 10.0, 12.0];

        assert_eq!(vec_add_scalar(&a, &b), expected);
        assert_eq!(vec_add_simd(&a, &b), expected);
    }

    #[test]
    fn test_sum_of_squares() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let expected = 30.0;

        assert_eq!(sum_of_squares_scalar(&data), expected);
        assert_eq!(sum_of_squares_simd(&data), expected);
    }
}
