use std::fmt;

// Estrutura Autor
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

// Superclasse Manuscrito por composição
pub struct Manuscrito {
    pub titulo: String,
    pub autor: Autor,
    pub ano_publicacao: u32,
    pub resumo: String,
}

impl Manuscrito {
    pub fn new(titulo: &str, autor: Autor, ano_publicacao: u32, resumo: &str) -> Self {
        Manuscrito {
            titulo: titulo.to_string(),
            autor,
            ano_publicacao,
            resumo: resumo.to_string(),
        }
    }
    pub fn get_titulo(&self) -> &str {
        &self.titulo
    }
    pub fn get_autor(&self) -> &Autor {
        &self.autor
    }
    pub fn get_ano_publicacao(&self) -> u32 {
        self.ano_publicacao
    }
    pub fn get_resumo(&self) -> &str {
        &self.resumo
    }
}

// Estrutura Livro, herdando Manuscrito por composição
pub struct Livro {
    pub manuscrito: Manuscrito,
    pub isbn: String,
    pub edicao: u32,
}

impl Livro {
    pub fn new(titulo: &str, autor: Autor, isbn: &str, ano_publicacao: u32, resumo: &str, edicao: u32) -> Self {
        Livro {
            manuscrito: Manuscrito::new(titulo, autor, ano_publicacao, resumo),
            isbn: isbn.to_string(),
            edicao,
        }
    }
}

// Estrutura Paper, herdando Manuscrito por composição
pub struct Paper {
    pub manuscrito: Manuscrito,
    pub doi: String,
    pub bibliografia: String,
}

impl Paper {
    pub fn new(titulo: &str, autor: Autor, doi: &str, ano_publicacao: u32, resumo: &str, bibliografia: &str) -> Self {
        Paper {
            manuscrito: Manuscrito::new(titulo, autor, ano_publicacao, resumo),
            doi: doi.to_string(),
            bibliografia: bibliografia.to_string(),
        }
    }
}

// Classe Entidade que engloba Manuscrito e Autor
pub enum Entidade {
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
