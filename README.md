# Icarus - Sistema de Catalogação e Busca de Livros

Este projeto é baseado no sistema [Dedalus](https://dedalus.usp.br) da USP, com o objetivo de permitir a catalogação e busca de livros de forma simples e eficiente. O sistema conta com dois principais campos de interação para os usuários:

1. **Cadastro de Livros e papers**: O usuário pode cadastrar livros e papers, preenchendo informações como **Autor**, **Título**, **Ano de Publicação** e **Resumo do Conteúdo** (ou descrição do livro). Esses dados são armazenados no sistema para posterior consulta.

2. **Busca de Livros ou papers**: Outro usuário pode buscar livros ou papers inserindo qualquer uma das seguintes informações: **Autor**, **Título**, **Ano de Publicação** ou **Resumo do Conteúdo**. O sistema retorna os 10 livros mais próximos da busca, com base nos critérios fornecidos.

## Funcionalidades

- **Cadastro de Autores**: O usuário pode inserir os dados do autores em um formulário com os seguintes campos:
  - **Autor**: Nome do autor do paper.
  - **Ano de Nascimento**: Ano de nascimento do autor.
  - **País de Nascimento**: País de nascimento do autor.

- **Cadastro de Livros**: O usuário pode inserir os dados do livro em um formulário com os seguintes campos:
  - **Autor**: Nome dos autores do livro.
  - **Título**: Título do livro.
  - **Ano de Publicação**: Ano de publicação do livro.
  - **Resumo do Conteúdo**: Descrição breve sobre o conteúdo do livro.

- **Busca de Livros ou papers**: O usuário pode buscar livros por um ou mais critérios:
  - **Autor**
  - **Título**
  - **Ano de Publicação**
  - **Resumo do Conteúdo** (ou descrição)

  O sistema retorna os 10 livros mais relevantes, de acordo com os parâmetros de busca fornecidos, utilizando um algoritmo de busca **vetorial**.

## Como Funciona

1. **Cadastro**: O usuário preenche os campos de informações do manuscrito e submete o formulário. O manuscrito será salvo no banco de dados para consultas futuras.

2. **Busca**: O outro usuário pode realizar uma busca utilizando um ou mais critérios. O sistema utiliza o algoritmo de busca fuzzy para encontrar os manuscrito mais próximos aos termos de busca inseridos e retorna os 10 mais relevantes.

## Tecnologias Utilizadas

- **Backend**: O sistema utiliza **Rust** para o desenvolvimento do backend.
- **Banco de Dados**: Utiliza **PostgreSQL [pgvector]** como banco de dados relacional leve para armazenar as informações dos livros.
- **Frontend**: A interface de usuário é desenvolvida utilizando **Svelte** com **TypeScript**.
- **Algoritmo de Busca**: O algoritmo de busca é implementado utilizando **busca vetorial**, para encontrar as correspondências mais aproximadas com os termos de pesquisa.

## Instalação e Execução

### Requisitos

- Rust (para o backend)
- PostgreSQL com extensão `pgvector` (para o banco de dados)
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
      brew install rust diesel typescript node postgres pgvector
      brew services start postgresql
      ```

3. **Dependências do Backend (Rust)**:
    - Utilizaremos o **Cargo** para gerenciar as dependências do projeto.
      ```bash
      cd backend
      diesel setup
      cargo +stable install cargo-llvm-cov --locked
      cargo build
      ```
      **Observações:**
      - Certifique-se de alterar o arquivo `.env` com as credenciais do banco de dados.
      - Caso tenha problemas com a instalação, consulte a documentação oficial do [Diesel](https://diesel.rs/) e [PostgreSQL](https://www.postgresql.org/docs/14/index.html)

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
    - Acesse o serviço via `http://localhost:5173`.

Extra. **Configuração do Banco de Dados PostgreSQL**:
    - O banco de dados será automaticamente criado pelo `diesel` utilizando as *migrations*, definidas no diretório `backend/migrations` quando o sistema for executado pela primeira vez. Caso queira realizar alterar o esquema do banco de dados, você pode criar um novo arquivo de migração utilizando o comando `diesel migration generate <nome_da_migração>`. Em seguida, você pode editar o arquivo de migração gerado para realizar as alterações desejadas no esquema do banco de dados.

## Algoritmo de Busca

A busca no sistema é realizada utilizando busca vetorial, que permite encontrar livros que possuam conteúdos semelhantes a um dado livro de referência. Para ajustar o algoritmo, é necessário alterar o algoritmo que faz a codificação dos vetores de características dos livros, uma extensão interessante da abordagem atual poderia ser utilizando modelos de linguagem (caso tenha interesse, recommendamos olhar a biblioteca [Candle](https://github.com/huggingface/candle)).

## Contribuições

Se você deseja contribuir para este projeto, siga os seguintes passos: T

1. Faça um fork do repositório.
2. Crie uma branch para a sua feature (`git checkout -b minha-feature`).
3. Comite suas mudanças (`git commit -am 'Adiciona nova feature'`).
4. Faça o push para a branch (`git push origin minha-feature`).
5. Abra um Pull Request no GitHub.
