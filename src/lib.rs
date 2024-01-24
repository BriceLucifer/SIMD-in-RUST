use rand::Rng;
use std::arch::x86_64::*;

pub fn cosine_similarity(a:&[f32],b:&[f32]) -> f32{
    let dot_product: f32 = a.iter().zip(b).map(|(a,b) | a * b).sum() ;
    let norm_a : f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt() ;
    let norm_b : f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt() ;
    let result = dot_product / norm_a * norm_b;
    result
}

pub fn generate_random_vector(dim : i32) -> Vec<f32>{
    let mut vec = Vec::new();
    for _ in 0..dim{
        vec.push(rand::thread_rng().gen::<f32>());
    }
    vec
}

pub fn cosine_similarity_simd(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must be of the same length");

    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    let chunks_a = a.chunks_exact(4);
    let chunks_b = b.chunks_exact(4);
    let remainder_a = chunks_a.remainder();
    let remainder_b = chunks_b.remainder();

    unsafe {
        for (chunk_a, chunk_b) in chunks_a.zip(chunks_b) {
            let a_chunk = _mm_loadu_ps(chunk_a.as_ptr());
            let b_chunk = _mm_loadu_ps(chunk_b.as_ptr());

            let dp = _mm_dp_ps(a_chunk, b_chunk, 0xF1);
            dot_product += _mm_cvtss_f32(dp);

            let na = _mm_dp_ps(a_chunk, a_chunk, 0xF1);
            norm_a += _mm_cvtss_f32(na);

            let nb = _mm_dp_ps(b_chunk, b_chunk, 0xF1);
            norm_b += _mm_cvtss_f32(nb);
        }
    }

    for (&a, &b) in remainder_a.iter().zip(remainder_b) {
        dot_product += a * b;
        norm_a += a * a;
        norm_b += b * b;
    }

    dot_product / (norm_a.sqrt() * norm_b.sqrt())
}


pub fn inner_product(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must be of the same length");
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

pub fn squared_euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must be of the same length");
    
    a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum()
    //powi = powerinteger 
    // 用来计算浮点数幂直接转换为 整数次幂
}

