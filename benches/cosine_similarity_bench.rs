use simd::{generate_random_vector,cosine_similarity,cosine_similarity_simd};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_cosine_similarity(c: &mut Criterion) {
    let dim = 1536;         //维度
    let first = generate_random_vector(dim);
    let second = generate_random_vector(dim);

    let mut group = c.benchmark_group("Cosine Similarity");
    group.sample_size(10);

    group.bench_function("Non-SIMD", |b| { 
        // 对于原生的计算
        b.iter(|| {
            let _ = cosine_similarity(black_box(&first), black_box(&second));
        })
    });

    group.bench_function("SIMD", |b| {
        // SIMD 单指令多数据计算
        b.iter(|| {
            let _ = cosine_similarity_simd(black_box(&first), black_box(&second));
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_cosine_similarity);
criterion_main!(benches);