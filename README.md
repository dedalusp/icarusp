# Icarus - Sistema de Catalogação e Busca de Livros

Este projeto é baseado no sistema [Dedalus](https://dedalus.usp.br) da USP, com o objetivo de permitir a catalogação e busca de livros de forma simples e eficiente. O sistema conta com dois principais campos de interação para os usuários:

1. **Cadastro de Livros**: O usuário pode cadastrar livros, preenchendo informações como **Autor**, **Título**, **Ano de Publicação** e **Resumo do Conteúdo** (ou descrição do livro). Esses dados são armazenados no sistema para posterior consulta.

2. **Busca de Livros**: Outro usuário pode buscar livros inserindo qualquer uma das seguintes informações: **Autor**, **Título**, **Ano de Publicação** ou **Resumo do Conteúdo**. O sistema retorna os 10 livros mais próximos da busca, com base nos critérios fornecidos.

## Funcionalidades

- **Cadastro de Livros**: O usuário pode inserir os dados do livro em um formulário com os seguintes campos:
  - **Autor**: Nome do autor do livro.
  - **Título**: Título do livro.
  - **Ano de Publicação**: Ano de publicação do livro.
  - **Resumo do Conteúdo**: Descrição breve sobre o conteúdo do livro.

- **Busca de Livros**: O usuário pode buscar livros por um ou mais critérios:
  - **Autor**
  - **Título**
  - **Ano de Publicação**
  - **Resumo do Conteúdo** (ou descrição)

  O sistema retorna os 10 livros mais relevantes, de acordo com os parâmetros de busca fornecidos, utilizando um algoritmo de busca **fuzzy**.

## Como Funciona

1. **Cadastro**: O usuário preenche os campos de informações do livro e submete o formulário. O livro será salvo no banco de dados para consultas futuras.

2. **Busca**: O outro usuário pode realizar uma busca utilizando um ou mais critérios. O sistema utiliza o algoritmo de busca fuzzy para encontrar os livros mais próximos aos termos de busca inseridos e retorna os 10 mais relevantes.

## Tecnologias Utilizadas

- **Backend**: O sistema utiliza **Rust** para o desenvolvimento do backend.
- **Banco de Dados**: Utiliza **SQLite** como banco de dados relacional leve para armazenar as informações dos livros.
- **Frontend**: A interface de usuário é desenvolvida utilizando **Svelte** com **TypeScript**.
- **Algoritmo de Busca**: O algoritmo de busca é implementado utilizando **fuzzy search**, para encontrar as correspondências mais aproximadas com os termos de pesquisa.

## Instalação e Execução

### Requisitos

- Rust (para o backend)
- SQLite (para o banco de dados)
- Node.js (para o frontend)
- npm ou yarn (para gerenciamento de pacotes)
- Pacotes de dependência (específicos para o backend e frontend)

### Passos para Instalação

1. **Clone o repositório**:
    ```bash
    git clone https://github.com/dedalusp/icarusp.git
    cd icarusp
    ```

2. **Instale as dependências do Projeto**:
    - Primeiro, instale o gerenciador de pacotes [brew](https://brew.sh/),
    dessa forma, temos um instalador comum tanto para sistemas UNIX (tanto
    Linux quanto OSX).
    - Em seguida, instale **Rust**, **TypeScript** e outras dependências do projeto:
      ```bash
      brew install rust typescript node sqlite
      ```

3. **Dependências do Backend (Rust)**:
    - Utilizaremos o **Cargo** para gerenciar as dependências do projeto.
      ```bash
      cd backend
      cargo build
      ```

3. **Dependências do Frontend (TypeScript)**:
    - Para o frontend, utilizaremos o [npm](https://www.npmjs.com/) para instalar as dependências:
      ```bash
      cd frontend
      npm install
      ```

4. **Iniciar os Servidores (Backend e Frontend)**:
    - Para rodar o backend, utilize o comando:
      ```bash
      cargo run
      ```
    - Em outra aba do terminal, rode o frontend:
      ```bash
      cd frontend
      npm run dev
      ```
    - Acesse o frontend via `http://localhost:3000` e o backend (se necessário) via `http://localhost:5000`.

Extra. **Configure o Banco de Dados SQLite**:
    - O banco de dados será automaticamente criado quando o sistema for executado pela primeira vez. Caso queira realizar a configuração manual, crie um banco de dados SQLite e configure as credenciais no arquivo de configuração do backend.

## Algoritmo de Busca

A busca no sistema é realizada utilizando busca vetorial, que permite encontrar livros que possuam conteúdos semelhantes a um dado livro de referência. Para
ajustar o algoritmo, é necessário alterar o modelo de linguagem utilizado, que
pode ser encontrado no arquivo `backend/models/model.rs`; assim como o caminho
para este novo modelo no arquivo `backend/main.rs`.

## Contribuições

Se você deseja contribuir para este projeto, siga os seguintes passos: T

1. Faça um fork do repositório.
2. Crie uma branch para a sua feature (`git checkout -b minha-feature`).
3. Comite suas mudanças (`git commit -am 'Adiciona nova feature'`).
4. Faça o push para a branch (`git push origin minha-feature`).
5. Abra um Pull Request no GitHub.
