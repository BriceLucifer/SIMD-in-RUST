# Rust Vector Utilities

## 简介

这个 Rust 代码仓库包含了一系列用于处理向量的实用函数。主要包括计算两个向量的余弦相似度、内积和平方欧几里得距离。此外，还提供了 SIMD（单指令多数据）版本的余弦相似度计算，以提高处理大型向量数据时的性能。

## 功能

- `cosine_similarity(a: &[f32], b: &[f32]) -> f32`: 计算两个向量的余弦相似度。
- `generate_random_vector(dim: i32) -> Vec<f32>`: 生成一个指定维度的随机浮点向量。
- `cosine_similarity_simd(a: &[f32], b: &[f32]) -> f32`: 使用 SIMD 指令集计算向量的余弦相似度。
- `inner_product(a: &[f32], b: &[f32]) -> f32`: 计算两个向量的内积。
- `squared_euclidean_distance(a: &[f32], b: &[f32]) -> f32`: 计算两个向量之间的平方欧几里得距离。

## 使用方法

首先，确保你的 Rust 环境已经安装并配置好。然后，你可以将这些函数集成到你的 Rust 项目中。

### 示例

记得先在src文件夹，创建一个main.rs 这个仓库多是lib.rs，benches文件夹里是bench.rs文件
以下是使用 `cosine_similarity` 函数的一个示例：

```rust
use simd::cosine_similarity;

fn main() {
    let vec1 = vec![1.0, 2.0, 3.0];
    let vec2 = vec![4.0, 5.0, 6.0];

    let similarity = cosine_similarity(&vec1, &vec2);
    println!("余弦相似度: {}", similarity);
}
