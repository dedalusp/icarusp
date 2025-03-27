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
    git clone TODO
    ```

2. **Instale as dependências do Backend (Rust)**: TODO revisar
    - Primeiro, instale o **Rust** caso não tenha:
      - [Instalar Rust](https://www.rust-lang.org/learn/get-started)
      
    - Instale as dependências do projeto:
      ```bash
      cargo build
      ```

3. **Instale as dependências do Frontend (TypeScript + Svelte)**: TODO revisar
    - Primeiramente, instale o **Node.js** caso não tenha:
      - [Instalar Node.js](https://nodejs.org/)
      
    - Navegue até o diretório do frontend:
      ```bash
      cd frontend
      ```

    - Instale as dependências do frontend:
      ```bash
      npm install
      ```

4. **Configure o Banco de Dados SQLite**: TODO revisar
    - O banco de dados será automaticamente criado quando o sistema for executado pela primeira vez. Caso queira realizar a configuração manual, crie um banco de dados SQLite e configure as credenciais no arquivo de configuração do backend.

5. **Inicie o servidor Backend (Rust)**: TODO revisar
    - Execute o servidor com o seguinte comando:
      ```bash
      cargo run
      ```

6. **Inicie o servidor Frontend (Svelte + TypeScript)**: TODO revisar
    - Para o frontend, execute o comando:
      ```bash
      npm run dev
      ```

7. **Acesse a aplicação**: TODO revisar
    - A aplicação estará disponível no navegador, geralmente em `http://localhost:5000` para o backend e `http://localhost:3000` para o frontend.

## Algoritmo de Busca Fuzzy

A busca no sistema é realizada utilizando o algoritmo **fuzzy search**, que permite encontrar livros que correspondem de forma aproximada aos termos de busca, mesmo que haja pequenas diferenças na digitação ou no formato das palavras.

## Contribuições 

Se você deseja contribuir para este projeto, siga os seguintes passos: TODO revisar

1. Faça um fork do repositório.
2. Crie uma branch para a sua feature (`git checkout -b minha-feature`).
3. Comite suas mudanças (`git commit -am 'Adiciona nova feature'`).
4. Faça o push para a branch (`git push origin minha-feature`).
5. Abra um Pull Request no GitHub.

## Licença

TODO

---
