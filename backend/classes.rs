use std::fmt;

// Estrutura Autor
struct Autor {
    nome: String,
    ano_nascimento: u32,
    pais: String,
}

impl Autor {
    fn new(nome: &str, ano_nascimento: u32, pais: &str) -> Self {
        Autor {
            nome: nome.to_string(),
            ano_nascimento,
            pais: pais.to_string(),
        }
    }
    fn get_nome(&self) -> &str {
        &self.nome
    }
    fn get_ano_nascimento(&self) -> u32 {
        self.ano_nascimento
    }
    fn get_pais(&self) -> &str {
        &self.pais
    }
}

// Superclasse Manuscrito por composição
struct Manuscrito {
    titulo: String,
    autor: Autor,
    ano_publicacao: u32,
    resumo: String,
}

impl Manuscrito {
    fn new(titulo: &str, autor: Autor, ano_publicacao: u32, resumo: &str) -> Self {
        Manuscrito {
            titulo: titulo.to_string(),
            autor,
            ano_publicacao,
            resumo: resumo.to_string(),
        }
    }
    fn get_titulo(&self) -> &str {
        &self.titulo
    }
    fn get_autor(&self) -> &Autor {
        &self.autor
    }
    fn get_ano_publicacao(&self) -> u32 {
        self.ano_publicacao
    }
    fn get_resumo(&self) -> &str {
        &self.resumo
    }
}

// Estrutura Livro, herdando Manuscrito por composição
struct Livro {
    manuscrito: Manuscrito,
    isbn: String,
    edicao: u32,
}

impl Livro {
    fn new(titulo: &str, autor: Autor, isbn: &str, ano_publicacao: u32, resumo: &str, edicao: u32) -> Self {
        Livro {
            manuscrito: Manuscrito::new(titulo, autor, ano_publicacao, resumo),
            isbn: isbn.to_string(),
            edicao,
        }
    }
}

// Estrutura Paper, herdando Manuscrito por composição
struct Paper {
    manuscrito: Manuscrito,
    doi: String,
    bibliografia: String,
}

impl Paper {
    fn new(titulo: &str, autor: Autor, doi: &str, ano_publicacao: u32, resumo: &str, bibliografia: &str) -> Self {
        Paper {
            manuscrito: Manuscrito::new(titulo, autor, ano_publicacao, resumo),
            doi: doi.to_string(),
            bibliografia: bibliografia.to_string(),
        }
    }
}

// Classe Entidade que engloba Manuscrito e Autor
enum Entidade {
    Livro(Livro),
    Paper(Paper),
    Autor(Autor),
}

impl fmt::Display for Entidade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Entidade::Livro(l) => write!(f, "Livro: {}", l.manuscrito.get_titulo()),
            Entidade::Paper(p) => write!(f, "Paper: {}", p.manuscrito.get_titulo()),
            Entidade::Autor(a) => write!(f, "Autor: {}", a.get_nome()),
        }
    }
}

fn main() {
    let autor = Autor::new("Machado de Assis", 1839, "Brasil");
    let livro = Livro::new("Dom Casmurro", autor, "123-456", 1899, "Um clássico da literatura brasileira.", 1);
    let entidade = Entidade::Livro(livro);
    println!("{}", entidade);
}
