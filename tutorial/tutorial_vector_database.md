Bancos de Dados Vetoriais

Um '''banco de dados vetorial''' armazena e gerencia dados como vetores de alta
dimensão, frequentemente gerados como outputs de [[rede neural artificial|redes
neurais]] para representar dados não estruturados (texto, imagens, áudio, etc.)
de forma compacta. Dessa maneira, a similaridade entre dados é interpretada pela
proximidade (distancia) de seus vetores, o que contrasta com a natureza de
bancos de dados tradicionais, onde a relacao entre dados é representada por
um esequema relacional.

Essa abordagem permite utilizare metricas como similaridade de cosseno ou
distancia euclidiana para representar similaridade em diversas aplicacoes, como:

    [[Sistema de recomendação|Sistemas de recomendação]]
    [[Mecanismo de busca|Mecanismos de busca]]
    Detecção de anomalias
    Genômica e bioinformática
    Busca semântica

[[Image: Placeholder for Introduction Image]]

== KNN e ANN ==

=== KNN ===
[[K-vizinhos mais próximos|K-Nearest Neighbors (KNN)]] é um algoritmo que 
encontra os "k" vetores mais próximos a um vetor de consulta em um espaço
multidimensional, baseando-se em métricas de distância para determinar a
similaridade.

[[Image: Placeholder for KNN Image]]

=== ANN ===
Para lidar com grandes volumes de dados, são usadas técnicas de 
[[Busca Aproximada do Vizinho Mais Próximo|Approximate Nearest Neighbors
(ANN)]]. Métodos ANN, como HNSW, fazem um compromisso comum em aplicacoes de
alta volumetria, sacrificando um pouco de precisao para obter maior velocidade
na busca.

{| class="wikitable"
|+ Comparação de Complexidade Computacional
|-
! Algoritmo
! Complexidade Computacional (Busca)
|-
| KNN (Força Bruta)
| O(N*D) - N: número de vetores, D: dimensões
|-
| ANN (Geralmente)
| O(log N) ou O(N log N) - Varia dependendo do algoritmo e implementação
|}

== Abordagens Híbridas: pgvector ==

O '''pgvector''' é uma extensão de [[código aberto]] para o [[PostgreSQL]] que 
adiciona suporte a dados vetoriais e busca de similaridade. Ele permite integrar
busca vetorial com as funcionalidades tradicionais de um banco de dados
relacional, criando uma abordagem híbrida.

=== Instalação ===
A instalação geralmente envolve adicionar a extensão a um banco de dados
PostgreSQL existente.

Para sistemas baseados em Debian/Ubuntu (ajuste a versão do PostgreSQL):
&lt;syntaxhighlight lang="bash">
sudo apt-get install postgresql-{{Versão}}-vector
&lt;/syntaxhighlight>

Para sistemas baseados em Red Hat/CentOS (ajuste a versão):
&lt;syntaxhighlight lang="bash">
sudo yum install postgresql{{Versão}}-contrib
&lt;/syntaxhighlight>

Ou compilando do código fonte:
&lt;syntaxhighlight lang="bash">
make
sudo make install
&lt;/syntaxhighlight>

Após instalar os binários, ative a extensão no banco de dados:
&lt;syntaxhighlight lang="sql">
CREATE EXTENSION vector;
&lt;/syntaxhighlight>

=== Criação de Dados com Vetores ===
Para armazenar vetores, crie uma tabela com uma coluna do tipo vector:
&lt;syntaxhighlight lang="sql">
CREATE TABLE documentos (
id SERIAL PRIMARY KEY,
conteudo TEXT,
embedding vector(1024) -- Exemplo com vetores de 1024 dimensões
);
&lt;/syntaxhighlight>

Inserir dados com vetores:
&lt;syntaxhighlight lang="sql">
INSERT INTO documentos (conteudo, embedding) VALUES
('Texto do primeiro documento', '[0.1, 0.2, ..., 0.9]'),
('Conteúdo do segundo documento', '[0.5, 0.3, ..., 0.1]');
&lt;/syntaxhighlight>

=== Consultas com pgvector ===
Buscar por similaridade (puramente vetorial) usando distância de cosseno:
&lt;syntaxhighlight lang="sql">
SELECT *
FROM documentos
ORDER BY embedding <=> '[0.2, 0.1, ..., 0.8]'
LIMIT 5;
&lt;/syntaxhighlight>
(O operador <-> com vector_cosine_ops calcula a distância de cosseno)

Consulta híbrida (vetorial com filtro SQL):
&lt;syntaxhighlight lang="sql">
SELECT *
FROM documentos
WHERE id > 10
ORDER BY embedding <=> '[0.2, 0.1, ..., 0.8]'
LIMIT 5;
&lt;/syntaxhighlight>
(Combina busca por similaridade com filtro SQL na coluna id)

=== Índices no pgvector ===
O pgvector suporta diferentes tipos de índices para otimizar a busca por
similaridade. O índice HNSW é recomendado para grandes conjuntos de dados.

{| class="wikitable"
|+ Índices Tradicionais e Características
|-
! Índice
! Complexidade de Consulta
! Aplicações Típicas
|-
| B-tree
| O(log N)
| Consultas de igualdade e faixa em dados ordenados
|-
| Hash Index
| O(1) em média
| Consultas de igualdade
|}

{| class="wikitable"
|+ Índices Vetoriais no pgvector e Trade-offs
|-
! Índice
! Descrição
! Trade-offs
|-
| Força Bruta
| Busca linear completa
| Lento para grandes datasets, preciso
|-
| HNSW (Hierarchical Navigable Small World)
| Grafo hierárquico para busca aproximada
| Rápido, boa precisão, maior custo de construção
|-
| IVF (Inverted File Index)
| Particiona o espaço vetorial
| Mais rápido que Força Bruta e HNSW, mas menos preciso que ambos. Depende de um conjunto de dados de treinamento
|}
&lt;syntaxhighlight lang="sql">
-- Exemplo de criação de índice HNSW
CREATE INDEX ON documentos USING hnsw (embedding vector_cosine_ops);
&lt;/syntaxhighlight>

== Comparação de Bancos de Dados Vetoriais ==

{| class="wikitable"
|+ Comparação de Plataformas de Banco de Dados Vetoriais
|-
! Plataforma
! Principais Características
! Tipo
|-
| [[Pinecone]]
| Indexação automática, atualizações em tempo real, escalabilidade, serverless
| Gerenciado
|-
| [[Milvus]]
| Indexação com vários algoritmos, busca eficiente, amplo suporte da comunidade
| Código Aberto
|-
| [[Weaviate]]
| Busca contextual, integração com grafos de conhecimento, busca porsimilaridade semântica
| Código Aberto
|-
| [[Qdrant]]
| Foco em velocidade e flexibilidade, indexação HNSW filtrável
| Código Aberto
|-
| [[Elasticsearch]] Vector Search
| Integração com o ecossistema Elasticsearch, escalabilidade, recursos de busca
| Híbrido (Search/Analytics com Vetor)
|-
| [[Amazon Web Services|AWS]] Kendra/OpenSearch
| Serviços AWS com recursos de busca vetorial
| Cloud / Híbrido
|-
| [[Microsoft Azure|Azure]] AI Search
| Serviço Azure com suporte a busca vetorial
| Cloud / Híbrido
|-
| [[Google Cloud Platform|Google Cloud]] Vertex AI Vector Search
| Busca de similaridade em grande escala, integrado ao Google Cloud AI
| Cloud
|-
| [[Oracle Database]] Vector Database
| Funcionalidade integrada ao Oracle Database para armazenar e consultar dados vetoriais
| Proprietário / Híbrido
|}

== Referências ==
{
  {reflist|
    [https://learn.microsoft.com/pt-br/dotnet/ai/conceptual/vector-databases 1. Microsoft Learn: Bancos de dados vetoriais]
    [https://www.oracle.com/br/database/vector-database/ 2. Oracle Brasil: Banco de Dados Vetorial]
    [https://www.ibm.com/br-pt/topics/vector-database 3. IBM Brasil: O que é um banco de dados vetorial?]
    [https://www.datacamp.com/pt/blog/the-top-5-vector-databases 4. DataCamp: The Top 5 Vector Databases]
    [https://www.ionos.com/pt-br/digitalguide/servidor/conhecimento/banco-de-dados-vetorial/ 5. IONOS: Banco de Dados Vetorial]
    [https://blog-pt.lac.tdsynnex.com/banco-de-dados-vetorial-o-que-e-quais-sao-as-vantagens 6. LAC TDC Synnex: Banco de dados vetorial]
    [https://www.ibm.com/think/topics/vector-database#:~:text=A%20vector%20database%20stores%2C%20manages,it%20ideal%20for%20AI%20applications. 7. IBM Think: What is a vector database?]
    [https://www.oracle.com/il-en/database/vector-database/ 8. Oracle Israel: Vector Database]
    [https://www.elastic.co/what-is/vector-database 9. Elastic: What is a vector database?]
    [https://www.ibm.com/think/topics/vector-database 10. IBM Think: Vector Database]
    [https://www.instaclustr.com/education/vector-database/vector-databases-explained-use-cases-algorithms-and-key-features/ 11. Instaclustr: Vector databases explained]
    [https://www.pinecone.io/learn/vector-database/ 12. Pinecone: Learn Vector Database]
    [https://qdrant.tech/articles/what-is-a-vector-database/ 13. Qdrant: What is a vector database?]
    [https://www.datastax.com/guides/what-is-a-vector-database 14. DataStax: What is a vector database?]
    [https://www.decube.io/post/vector-database-concept 15. Decube: Vector database concept]
    [https://aws.amazon.com/what-is/vector-databases/ 16. AWS: What is a vector database?]
    [https://www.databricks.com/glossary/vector-database 17. Databricks: Vector Database Glossary]
    [https://www.elastic.co/pt/what-is/vector-database 18. Elastic Brasil: O que é um banco de dados vetorial?]
  }
}
