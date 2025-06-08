use std::fmt;
use anyhow::Result;
use limbo::Connection;
use crate::database::insertion::{insert_autor, insert_publicacao_with_author};

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
        let embedding = crate::embedding::compute_embedding(resumo)?; // Generate embedding
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
        insert_publicacao_with_author(conn, self, autor_nome).await
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

#[tokio::main]
async fn main() -> Result<()> {
    let conn = crate::database::initialization::initialize_db_connection("database.db").await?;

    let autor = Autor::new("Machado de Assis", 1839, "Brasil");
    let autor_id = autor.insert(&conn).await?;
    println!("Inserted Autor with ID: {}", autor_id);

    let publicacao = Publicacao::new("Dom Casmurro", 1899, "Um romance sobre ciúmes e traição.")?;
    let (autor_id, publicacao_id) = publicacao.insert(&conn, "Machado de Assis").await?;
    println!(
        "Inserted Publicacao with ID: {} and linked to Autor ID: {}",
        publicacao_id, autor_id
    );

    Ok(())
}
