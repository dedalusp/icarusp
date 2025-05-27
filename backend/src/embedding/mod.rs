use anyhow::{Error as E, Result};
use candle::Tensor;
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

/// Loads the model and tokenizer using Hugging Face's Candle library.
fn load_model_and_tokenizer(model_id: &str) -> Result<(BertModel, Tokenizer)> {
    let device = candle_examples::device(false)?; // Use GPU if available
    let repo = Repo::with_revision(model_id.to_string(), RepoType::Model, "main".to_string());
    let (config_filename, tokenizer_filename, weights_filename) = {
        let api = Api::new()?;
        let api = api.repo(repo);
        let config = api.get("config.json")?;
        let tokenizer = api.get("tokenizer.json")?;
        let weights = api.get("model.safetensors")?;
        (config, tokenizer, weights)
    };

    let config = std::fs::read_to_string(config_filename)?;
    let mut config: Config = serde_json::from_str(&config)?;
    let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;

    let vb = VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)?;
    let model = BertModel::load(vb, &config)?;
    Ok((model, tokenizer))
}

/// Computes an embedding for a given input text using the model and tokenizer.
fn compute_embedding(model: &BertModel, tokenizer: &Tokenizer, text: &str) -> Result<Tensor> {
    let device = &model.device;
    let tokenizer = tokenizer
        .with_padding(None)
        .with_truncation(None)
        .map_err(E::msg)?;
    let tokens = tokenizer
        .encode(text, true)
        .map_err(E::msg)?
        .get_ids()
        .to_vec();
    let token_ids = Tensor::new(&tokens[..], device)?.unsqueeze(0)?;
    let token_type_ids = token_ids.zeros_like()?;
    let embeddings = model.forward(&token_ids, &token_type_ids, None)?;
    let (_batch_size, n_tokens, _hidden_size) = embeddings.dims3()?;
    let pooled_embedding = embeddings.sum(1)? / (n_tokens as f64);
    Ok(pooled_embedding)
}

/// Computes the cosine similarity between pairs of embeddings.
fn compute_cosine_similarity(embeddings: &[Tensor]) -> Result<Vec<(f32, usize, usize)>> {
    let mut similarities = vec![];
    for i in 0..embeddings.len() {
        let e_i = &embeddings[i];
        for j in (i + 1)..embeddings.len() {
            let e_j = &embeddings[j];
            let sum_ij = (e_i * e_j)?.sum_all()?.to_scalar::<f32>()?;
            let sum_i2 = (e_i * e_i)?.sum_all()?.to_scalar::<f32>()?;
            let sum_j2 = (e_j * e_j)?.sum_all()?.to_scalar::<f32>()?;
            let cosine_similarity = sum_ij / (sum_i2 * sum_j2).sqrt();
            similarities.push((cosine_similarity, i, j));
        }
    }
    similarities.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    Ok(similarities)
}

fn main() -> Result<()> {
    // Example usage
    let model_id = "sentence-transformers/all-MiniLM-L6-v2";
    let (model, tokenizer) = load_model_and_tokenizer(model_id)?;

    let sentences = vec![
        "The cat sits outside",
        "A man is playing guitar",
        "I love pasta",
        "The new movie is awesome",
    ];

    let embeddings = sentences
        .iter()
        .map(|sentence| compute_embedding(&model, &tokenizer, sentence))
        .collect::<Result<Vec<_>>>()?;

    let similarities = compute_cosine_similarity(&embeddings)?;
    for (score, i, j) in similarities.iter().take(5) {
        println!("Score: {:.2} '{}' '{}'", score, sentences[*i], sentences[*j]);
    }

    Ok(())
}