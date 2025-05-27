use backend::embedding::{load_model_and_tokenizer, compute_embedding};
use backend::classes::{Autor, Livro, Entidade};

#[test]
fn test_embedding_associated_with_livro_entity() {
    // Load model/tokenizer
    let model_id = "neuralmind/bert-base-portuguese-cased";
    let (model, tokenizer) = load_model_and_tokenizer(model_id)
        .expect("Failed to load model/tokenizer");

    // Create author and book
    let autor = Autor::new("Ada Lovelace", 1815, "UK");
    let resumo = "A foundational work in theoretical computing.";
    let livro = Livro::new("Notes on the Analytical Engine", autor, "000-000", 1843, resumo, 1);

    // Compute embedding for the summary
    let embedding = compute_embedding(&model, &tokenizer, livro.manuscrito.get_resumo())
        .expect("Failed to compute embedding");

    println!("{:?}", embedding);
}

