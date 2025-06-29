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
}
