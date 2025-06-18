#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedding::embedding::{compute_embedding, compute_cosine_similarity};

    #[test]
    fn test_embedding_length() {
        let text = "testando embedding";
        let embedding = compute_embedding(text).unwrap();
        assert_eq!(embedding.len(), 512);
    }

    #[test]
    fn test_embedding_norm() {
        let text = "abcabcabc";
        let embedding = compute_embedding(text).unwrap();
        let norm = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        // Norm should be 1.0 or very close due to normalization
        assert!((norm - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_embedding_empty() {
        let text = "";
        let embedding = compute_embedding(text).unwrap();
        // All values should be zero
        assert!(embedding.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_embedding_short_text() {
        let text = "ab";
        let embedding = compute_embedding(text).unwrap();
        // All values should be zero because text is shorter than N_GRAM
        assert!(embedding.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let text = "mesma frase";
        let emb1 = compute_embedding(text).unwrap();
        let emb2 = compute_embedding(text).unwrap();
        let sims = compute_cosine_similarity(&[emb1.clone(), emb2.clone()]).unwrap();
        // Only one pair, similarity should be 1.0 or very close
        assert!((sims[0].0 - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_cosine_similarity_different() {
        let emb1 = compute_embedding("gato preto").unwrap();
        let emb2 = compute_embedding("violão azul").unwrap();
        let sims = compute_cosine_similarity(&[emb1, emb2]).unwrap();
        // Similarity should be between -1 and 1
        assert!(sims[0].0 <= 1.0 && sims[0].0 >= -1.0);
    }
}