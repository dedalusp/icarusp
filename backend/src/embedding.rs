use tch::{CModule, Tensor, Device};
use std::fs;

fn computa_embedding() {
    // Caminho do modelo salvo em TorchScript
    let model_path = "model.pt";
    
    // Carregar modelo
    let model = CModule::load(model_path).expect("Falha ao carregar o modelo");
    
    // Entrada de texto (máx. 255 caracteres)
    let input_text = "Este é um exemplo de texto para gerar um embedding.";
    
    // Garantir que o texto tem no máximo 255 caracteres
    let truncated_text = &input_text[..std::cmp::min(255, input_text.len())];
    
    // Converter string para tensor (exemplo simples: converte caracteres para valores ASCII)
    let input_tensor = Tensor::of_slice(&truncated_text.bytes().map(|b| b as i64).collect::<Vec<_>>())
        .unsqueeze(0) // Adicionar dimensão batch
        .to(Device::cuda_if_available());
    
    // Executar modelo para obter embedding
    let output = model.forward_ts(&[input_tensor]).expect("Erro na inferência");
    
    // Exibir embedding resultante
    println!("Embedding gerado: {:?}", output);
}
