use crate::classes::{Autor, Manuscrito, Livro, Paper, Entidade};

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
    fn test_manuscrito_creation() {
        let autor = Autor::new("Machado de Assis", 1839, "Brasil");
        let manuscrito = Manuscrito::new(
            "Dom Casmurro",
            autor,
            1899,
            "Um clássico da literatura brasileira.",
        );
        assert_eq!(manuscrito.get_titulo(), "Dom Casmurro");
        assert_eq!(manuscrito.get_ano_publicacao(), 1899);
        assert_eq!(manuscrito.get_resumo(), "Um clássico da literatura brasileira.");
    }

    #[test]
    fn test_livro_creation() {
        let autor = Autor::new("Machado de Assis", 1839, "Brasil");
        let livro = Livro::new(
            "Dom Casmurro",
            autor,
            "123-456",
            1899,
            "Um clássico da literatura brasileira.",
            1,
        );
        assert_eq!(livro.manuscrito.get_titulo(), "Dom Casmurro");
        assert_eq!(livro.isbn, "123-456");
        assert_eq!(livro.edicao, 1);
    }

    #[test]
    fn test_paper_creation() {
        let autor = Autor::new("Albert Einstein", 1879, "Alemanha");
        let paper = Paper::new(
            "Relatividade",
            autor,
            "10.1234/relativity",
            1905,
            "Um artigo revolucionário sobre a teoria da relatividade.",
            "Einstein, A. (1905). Relatividade.",
        );
        assert_eq!(paper.manuscrito.get_titulo(), "Relatividade");
        assert_eq!(paper.doi, "10.1234/relativity");
        assert_eq!(
            paper.bibliografia,
            "Einstein, A. (1905). Relatividade."
        );
    }

    #[test]
    fn test_entidade_display() {
        let autor = Autor::new("Machado de Assis", 1839, "Brasil");
        let livro = Livro::new(
            "Dom Casmurro",
            autor,
            "123-456",
            1899,
            "Um clássico da literatura brasileira.",
            1,
        );
        let entidade = Entidade::Livro(livro);
        assert_eq!(format!("{}", entidade), "Livro: Dom Casmurro");
    }
}