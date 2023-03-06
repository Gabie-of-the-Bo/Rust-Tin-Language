pub fn sum_i64(v: &Vec<i64>) -> i64 {
    let simd_res = v.chunks_exact(4)
    .map(|i| wide::i64x4::new([i[0], i[1], i[2], i[3]]))
    .reduce(|acc, e| acc + e).unwrap_or(wide::i64x4::ZERO);
    
    let rem = v.chunks_exact(4).remainder().into_iter().sum::<i64>();

    return simd_res.as_array_ref().into_iter().sum::<i64>() + rem;
}

pub fn sum_f64(v: &Vec<f64>) -> f64 {
    let simd_res = v.chunks_exact(4)
    .map(|i| wide::f64x4::new([i[0], i[1], i[2], i[3]]))
    .reduce(|acc, e| acc + e).unwrap_or(wide::f64x4::ZERO);
    
    let rem = v.chunks_exact(4).remainder().into_iter().sum::<f64>();

    return simd_res.as_array_ref().into_iter().sum::<f64>() + rem;
}

pub fn product_i64(v: &Vec<i64>) -> i64 {
    let simd_res = v.chunks_exact(4)
    .map(|i| wide::i64x4::new([i[0], i[1], i[2], i[3]]))
    .reduce(|acc, e| acc * e).unwrap_or(wide::i64x4::ONE);
    
    let rem = v.chunks_exact(4).remainder().into_iter().product::<i64>();

    return simd_res.as_array_ref().into_iter().product::<i64>() * rem;
}

pub fn product_f64(v: &Vec<f64>) -> f64 {
    let simd_res = v.chunks_exact(4)
    .map(|i| wide::f64x4::new([i[0], i[1], i[2], i[3]]))
    .reduce(|acc, e| acc * e).unwrap_or(wide::f64x4::ONE);
    
    let rem = v.chunks_exact(4).remainder().into_iter().product::<f64>();

    return simd_res.as_array_ref().into_iter().product::<f64>() * rem;
}