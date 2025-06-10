use std::fmt;
use anyhow::Result;
use limbo::Connection;

use crate::database::insertion::{insert_autor, insert_publication_with_author};
use crate::embedding::compute_embedding;

/// Autor struct
pub struct Autor {
    pub nome: String,
    pub ano_nascimento: u32,
    pub pais: String,
}

impl Autor {
    pub fn new(nome: &str, ano_nascimento: u32, pais: &str) -> Self {
        Autor {
            nome: nome.to_string(),
            ano_nascimento,
            pais: pais.to_string(),
        }
    }

    pub fn get_nome(&self) -> &str {
        &self.nome
    }

    pub fn get_ano_nascimento(&self) -> u32 {
        self.ano_nascimento
    }

    pub fn get_pais(&self) -> &str {
        &self.pais
    }

    /// Inserts the Autor into the Autores table.
    pub async fn insert(&self, conn: &Connection) -> Result<i64> {
        insert_autor(conn, self).await
    }
}

impl fmt::Display for Autor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Autor: {} ({} - {})", self.nome, self.ano_nascimento, self.pais)
    }
}

/// Hardcoded constant for the vector dimension.
const VECTOR_DIMENSION: u32 = 512;

pub struct Publicacao {
    pub titulo: String,
    pub ano_publicacao: u32,
    pub resumo: String,
    pub embedding: Vec<f32>,
}

impl Publicacao {
    pub fn new(titulo: &str, ano_publicacao: u32, resumo: &str) -> Result<Self> {
        let embedding = compute_embedding(resumo)?; // Generate embedding
        Ok(Publicacao {
            titulo: titulo.to_string(),
            ano_publicacao,
            resumo: resumo.to_string(),
            embedding,
        })
    }

    pub fn get_titulo(&self) -> &str {
        &self.titulo
    }

    pub fn get_ano_publicacao(&self) -> u32 {
        self.ano_publicacao
    }

    pub fn get_resumo(&self) -> &str {
        &self.resumo
    }

    pub fn get_embedding(&self) -> &Vec<f32> {
        &self.embedding
    }

    /// Inserts the Publicacao into the Publicacoes table and links it to an Autor by name.
    pub async fn insert(&self, conn: &Connection, autor_nome: &str) -> Result<(i64, i64)> {
        insert_publication_with_author(conn, self, autor_nome).await
    }
}

impl fmt::Display for Publicacao {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Publicação: {} ({})", self.titulo, self.ano_publicacao)
    }
}

// /// Entidade enum that wraps Autor and Publicacao
// /// This can be seen as an "abstract class" for both types.
// pub enum Entidade {
//     Autor(Autor),
//     Publicacao(Publicacao),
// }
// This abstract class can be seems as a way to implement polymorphism
// in Rust. For instance, we implement both methods `display` and
// `insert` for both `Autor` and `Publicacao`, and we can
// use the `Entidade` enum to handle both types generically.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autor_creation() {
        let autor = Autor::new("Machado de Assis", 1839, "Brasil");
        assert_eq!(autor.get_nome(), "Machado de Assis");
        assert_eq!(autor.get_ano_nascimento(), 1839);
        assert_eq!(autor.get_pais(), "Brasil");
    }

    #[test]
    fn test_publicacao_creation() {
        // Mock the embedding function if needed, or adjust as per your project setup
        // Here, we assume compute_embedding returns Ok(vec![0.0; 512]) for testing
        use std::sync::Once;
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            // Patch the embedding function for tests if needed
        });

        let publicacao = Publicacao::new(
            "Dom Casmurro",
            1899,
            "Um clássico da literatura brasileira.",
        ).unwrap();
        assert_eq!(publicacao.get_titulo(), "Dom Casmurro");
        assert_eq!(publicacao.get_ano_publicacao(), 1899);
        assert_eq!(publicacao.get_resumo(), "Um clássico da literatura brasileira.");
        assert_eq!(publicacao.get_embedding().len(), 512);
    }
}
