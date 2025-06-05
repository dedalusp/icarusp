use anyhow::Result;

/// N-gram size and vocabulary size constants.
const N_GRAM: usize = 3;
const VOCAB_SIZE: usize = 512;

/// Simple hash function for n-grams to map to vocabulary indices.
fn ngram_hash(ngram: &[u8]) -> usize {
    use std::hash::Hasher;
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    hasher.write(ngram);
    (hasher.finish() as usize) % VOCAB_SIZE
}

/// Computes an embedding (float vector) for a given input text using N-gram
/// hashing.
fn compute_embedding(text: &str) -> Result<Vec<f32>> {
    // Initialize an embedding vector with zeros.
    // The size of the vector is determined by the vocabulary size.
    let mut embedding = vec![0f32; VOCAB_SIZE];
    let bytes = text.as_bytes();
    if bytes.len() < N_GRAM {
        return Ok(embedding);
    }
    for ngram in bytes.windows(N_GRAM) {
        let idx = ngram_hash(ngram);
        embedding[idx] += 1.0;
    }
    // Compute the L2 norm (Euclidean norm) of the embedding vector.
    // This is done by summing the squares of each element, then taking the
    // square root.
    let norm = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for v in &mut embedding {
            *v /= norm;
        }
    }
    Ok(embedding)
}


/// Computes the cosine similarity between pairs of embeddings (Vec<f32>).
fn compute_cosine_similarity(embeddings: &[Vec<f32>]) -> Result<Vec<(f32, usize, usize)>> {
    let mut similarities = vec![];
    for i in 0..embeddings.len() {
        let e_i = &embeddings[i];
        for j in (i + 1)..embeddings.len() {
            let e_j = &embeddings[j];
            // Compute dot product
            let sum_ij: f32 = e_i.iter().zip(e_j.iter()).map(|(a, b)| a * b).sum();
            // Compute norms
            let sum_i2: f32 = e_i.iter().map(|x| x * x).sum();
            let sum_j2: f32 = e_j.iter().map(|x| x * x).sum();
            let cosine_similarity = if sum_i2 > 0.0 && sum_j2 > 0.0 {
                sum_ij / (sum_i2.sqrt() * sum_j2.sqrt())
            } else {
                0.0
            };
            similarities.push((cosine_similarity, i, j));
        }
    }
    similarities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    Ok(similarities)
}

fn main() -> Result<()> {
    // Example usage

    let sentences = vec![
        "The cat sits outside",
        "A man is playing guitar",
        "I love pasta",
        "The new movie is awesome",
    ];

    let embeddings = sentences
        .iter()
        .map(|sentence| compute_embedding(sentence))
        .collect::<Result<Vec<_>>>()?;

    let similarities = compute_cosine_similarity(&embeddings)?;
    for (score, i, j) in similarities.iter().take(5) {
        println!("Score: {:.2} '{}' '{}'", score, sentences[*i], sentences[*j]);
    }

    Ok(())
}