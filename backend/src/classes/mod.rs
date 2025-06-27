use anyhow::Result;
use std::fmt;

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
}

impl fmt::Display for Autor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Autor: {} ({} - {})",
            self.nome, self.ano_nascimento, self.pais
        )
    }
}

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
}

impl fmt::Display for Publicacao {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Publicação: {} ({})", self.titulo, self.ano_publicacao)
    }
}

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
        let publicacao = Publicacao::new(
            "Dom Casmurro",
            1899,
            "Um clássico da literatura brasileira.",
        )
        .unwrap();
        assert_eq!(publicacao.get_titulo(), "Dom Casmurro");
        assert_eq!(publicacao.get_ano_publicacao(), 1899);
        assert_eq!(
            publicacao.get_resumo(),
            "Um clássico da literatura brasileira."
        );
        assert_eq!(publicacao.get_embedding().len(), 512);
    }
}
