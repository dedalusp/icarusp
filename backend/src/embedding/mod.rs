use anyhow::Result;

/// N-gram size and vocabulary size constants.
const N_GRAM: usize = 3;
const VOCAB_SIZE: usize = 512;

/// Simple hash function for n-grams to map to vocabulary indices.
fn ngram_hash(ngram: &[u8]) -> usize {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;
    let mut hasher = DefaultHasher::new();
    hasher.write(ngram);
    (hasher.finish() as usize) % VOCAB_SIZE
}

/// Computes an embedding (float vector) for a given input text using N-gram
/// hashing.
pub fn compute_embedding(text: &str) -> Result<Vec<f32>> {
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ngram_hash_consistency() {
        let ngram = b"abc";
        let idx1 = ngram_hash(ngram);
        let idx2 = ngram_hash(ngram);
        assert_eq!(idx1, idx2);
        assert!(idx1 < VOCAB_SIZE);
    }

    #[test]
    fn test_compute_embedding_empty() {
        let emb = compute_embedding("").unwrap();
        assert_eq!(emb.len(), VOCAB_SIZE);
        assert!(emb.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_compute_embedding_short_text() {
        let emb = compute_embedding("ab").unwrap();
        assert_eq!(emb.len(), VOCAB_SIZE);
        assert!(emb.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_compute_embedding_basic() {
        let emb = compute_embedding("abcd").unwrap();
        assert_eq!(emb.len(), VOCAB_SIZE);
        let nonzero: Vec<_> = emb.iter().filter(|&&x| x > 0.0).collect();
        assert!(nonzero.len() > 0);
        let norm = emb.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_compute_cosine_similarity_identical() {
        let emb1 = compute_embedding("hello world").unwrap();
        let emb2 = compute_embedding("hello world").unwrap();
        let sims = compute_cosine_similarity(&[emb1.clone(), emb2.clone()]).unwrap();
        assert_eq!(sims.len(), 1);
        let (sim, i, j) = sims[0];
        assert_eq!(i, 0);
        assert_eq!(j, 1);
        assert!((sim - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_compute_cosine_similarity_different() {
        let emb1 = compute_embedding("hello world").unwrap();
        let emb2 = compute_embedding("rust language").unwrap();
        let sims = compute_cosine_similarity(&[emb1, emb2]).unwrap();
        assert_eq!(sims.len(), 1);
        let (sim, _, _) = sims[0];
        assert!(sim >= -1.0 && sim <= 1.0);
    }

    #[test]
    fn test_compute_cosine_similarity_multiple() {
        let emb1 = compute_embedding("abc").unwrap();
        let emb2 = compute_embedding("abd").unwrap();
        let emb3 = compute_embedding("xyz").unwrap();
        let sims = compute_cosine_similarity(&[emb1, emb2, emb3]).unwrap();
        assert_eq!(sims.len(), 3);
        // Should be sorted by similarity descending
        assert!(sims[0].0 >= sims[1].0 && sims[1].0 >= sims[2].0);
    }
}
