# Elicitação de Requisitos - Icarus

## 1. Visão Geral do Sistema

O **Icarus** é um sistema inspirado no Dedalus da USP, desenvolvido para permitir a catalogação e busca eficiente de livros. Ele possibilita que os usuários cadastrem livros com informações detalhadas e realizem buscas utilizando critérios específicos.

---

## 2. Requisitos Funcionais
### 2.1. Cadastro
O sistema deve permitir que o usuário cadastre autores e livros; armazenando essas informações cadastradas em um banco de dados.

#### 2.1.1 Autores
  - **Nome** (Obrigatório - Chave Primária)
  - **Ano de Nascimento** (Obrigatório)
  - **País de Nascimento** (Obrigatório)

#### 2.1.2 Livros
  - **ISBN** (Obrigatório - Chave Primária)
  - **Título** (Obrigatório)
  - **Autor** (Obrigatório - Chave Estrangeira)
  - **Ano de Publicação** (Obrigatório)
  - **Resumo de Conteúdo** (Obrigatório)
  - **Número de Edições** (Obrigatório)

#### 2.1.3 Papers
  - **DOI** (Obrigatório - Chave Primária)
  - **Título** (Obrigatório)
  - **Autor** (Obrigatório - Chave Estrangeira)
  - **Ano de Publicação** (Obrigatório)
  - **Abstract** (Obrigatório)
  - **Bibliografia** (Obrigatório)

**Obs**: autores devem ter sido previamente cadastrados antes de cadastar um livro ou paper.

### 2.2. Busca

#### 2.2.1 Autor
O sistema deve permitir que os usuários busque todos os livros ou papers publicados por um mesmo autor (dentre os autores presentes na base).

#### 2.2.2 Livros & Papers
O sistema deve permitir que os usuários busquem livros utilizando um ou mais dos seguintes critérios:
  - **Título**: o sistema deve retornar **(até) 10 livros/papers mais similares**, com base nos seus títulos, recuperando informações relacionadas ao livro/paper.
  - **Conteúdo**: o sistema deve retornar **(até) 10 livros/papers mais similares**, com base nos seus resumos de conteúdos ou abstracts.

#### 2.2.3 Ano
O sistema deve permitir que os usuários busque todos os livros publicados no mesmo ano. E todos os papers publicados no mesmo ano.

#### 2.2.4 Nacionalidade Autor
O sistema deve permitir que os usuários busque todos os livros publicados por autores que possuam uma mesma nacionalidade.


### 2.3. Interface de Usuário
O sistema deve oferecer uma interface gráfica que possibilite cadastro de autores e livros, assim como as buscas listadas anteriormente.

### Observação
O sistema não possui como requisito funcional a alteração de dados cadastrados.

---

## 3. Requisitos Não Funcionais
### 3.1. Tecnologias Utilizadas
- **Backend**: Desenvolvido em **Rust**.
- **Banco de Dados**: Utiliza **SQLite** ou variante ([Limbo](https://github.com/tursodatabase/limbo))
- **Frontend**: Desenvolvido com **Svelte + TypeScript**.

### 3.2 Interface para Consultas

Os campos de busca devem utilizar **fuzzy search** para facilitar a seleção por parte do usuário.

#### 3.3 Busca vetorial

A busca por similaridade entre livros deve ser implementada via **busca vetorial**, utilizando um *embedding* do conteúdo do livro para guiar a busca.

### 3.4. Desempenho e Eficiência
- O sistema deve ser capaz de cadastrar e buscar livros de forma rápida e eficiente, de modo que resposta dessas ações sejam retornadas em **tempo real** ao usuário:
- No **máximo** 1 segundo de latência para operações de escrita e consulta de autores;
- No **máximo** 10 segundos de latência para operações de escrita e consulta de livros, dado que a conversão de **Resumo de Conteúdo** em embedding (e sua busca vetorial) pode tomar mais tempo, a depender das especificações do sistema do usuário.

### 3.5. Segurança
- O sistema deve garantir a integridade dos dados armazenados no banco de dados.
- A comunicação entre frontend e backend deve ser segura.

### 3.6. Portabilidade
- O sistema deve ser compatível com os principais navegadores modernos.
- Deve ser possível executar o backend em diferentes sistemas operacionais compatíveis com Rust.

---

## 4. Requisitos de Instalação e Execução
O usuário deve possuir as seguintes dependências instaladas:
  - **Rust**
    - Instalar pacotes Rust seguindo um `cargo.toml`
  - **Node.js**
  - **npm ou yarn** (para gerenciamento de pacotes)

O sistema deve permitir que o usuário instale e execute os serviços backend e frontend com os seguintes passos:
  1. Clonar o repositório.
  2. Instalar dependências do backend e frontend.
  3. Configurar e iniciar o banco de dados.
  4. Executar o backend e frontend.
  5. Acessar a aplicação pelo navegador.
