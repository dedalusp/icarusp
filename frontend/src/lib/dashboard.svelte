<script lang="ts">
    let selectedTab: 'insercao' | 'consulta' = 'insercao';
    let selectedInsercaoTab: 'autor' | 'livro' | 'artigo' = 'autor';
    let valorParaBusca: string = ""; // Valor digitado pelo usuário
    let campoDeBusca: 'autor' | 'titulo' | 'conteudo' | 'ano' | 'nacionalidade' = 'autor'; // Campo selecionado
    let resultado: { titulo: string; nome_autor: string; ano_publicacao: number; resumo: string; nacionalidade: string } | null = null;

    let nome = "";
    let anoNascimento: number | null = null;
    let pais = "";
    let isbn = "";
    let titulo = "";
    let autor = "";
    let anoPublicacao: number | null = null;
    let resumo = "";
    let edicoes: number | null = null;
    let doi = "";
    let abstractText = "";
    let bibliografia = "";

    async function buscar() {
        try {
            const response = await fetch(`http://127.0.0.1:8080/buscar?valor_para_busca=${encodeURIComponent(valorParaBusca)}&campo_de_busca=${encodeURIComponent(campoDeBusca)}`);
            if (!response.ok) {
                throw new Error("Erro ao buscar dados");
            }
            resultado = await response.json();
        } catch (error) {
            console.error("Erro ao buscar dados:", error);
            resultado = null;
        }
    }

    async function enviarDados() {
        const dados = {
            nome,
            ano_nascimento: anoNascimento,
            pais,
            isbn,
            titulo,
            autor,
            ano_publicacao: anoPublicacao,
            resumo,
            edicoes,
            doi,
            abstract_text: abstractText,
            bibliografia,
        };

        try {
            const response = await fetch("http://127.0.0.1:8080/inserir", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(dados),
            });

            if (!response.ok) {
                throw new Error("Erro ao enviar dados");
            }

            const resultado = await response.text();
            console.log("Resposta do backend:", resultado);
        } catch (error) {
            console.error("Erro ao enviar dados:", error);
        }
    }
</script>

<main class="container">
    <nav class="tabs">
        <button class="tab" on:click={() => selectedTab = 'insercao'} class:selected={selectedTab === 'insercao'}>
            Inserção
        </button>
        <button class="tab" on:click={() => selectedTab = 'consulta'} class:selected={selectedTab === 'consulta'}>
            Consulta
        </button>
    </nav>
    
    <section>
        {#if selectedTab === 'insercao'}
            <div class="panel insercao">
                <h2>Inserção de Dados</h2>
                <p>Escolha entre adicionar um novo Autor, Livro ou Artigo.</p>
                <div class="tabs">
                    <button class="tab" on:click={() => selectedInsercaoTab = 'autor'} class:selected={selectedInsercaoTab === 'autor'}>
                        Novo Autor
                    </button>
                    <button class="tab" on:click={() => selectedInsercaoTab = 'livro'} class:selected={selectedInsercaoTab === 'livro'}>
                        Novo Livro
                    </button>
                    <button class="tab" on:click={() => selectedInsercaoTab = 'artigo'} class:selected={selectedInsercaoTab === 'artigo'}>
                        Novo Artigo
                    </button>
                </div>
                {#if selectedInsercaoTab === 'autor'}
                    <p class="bold">Insira os dados do novo autor:</p>
                    <input type="text" placeholder="Nome" class="input-box" bind:value={nome} />
                    <input type="number" placeholder="Ano de Nascimento" class="input-box" bind:value={anoNascimento} />
                    <input type="text" placeholder="País de Nascimento" class="input-box" bind:value={pais} />
                {:else if selectedInsercaoTab === 'livro'}
                    <p class="bold">Insira os dados do novo livro:</p>
                    <input type="text" placeholder="ISBN" class="input-box" bind:value={isbn} />
                    <input type="text" placeholder="Título" class="input-box" bind:value={titulo} />
                    <input type="text" placeholder="Autor" class="input-box" bind:value={autor} />
                    <input type="number" placeholder="Ano de Publicação" class="input-box" bind:value={anoPublicacao} />
                    <textarea placeholder="Resumo de Conteúdo" class="input-box content-box" bind:value={resumo}></textarea>
                    <input type="number" placeholder="Número de Edições" class="input-box" bind:value={edicoes} />
                {:else if selectedInsercaoTab === 'artigo'}
                    <p class="bold">Insira os dados do novo artigo:</p>
                    <input type="text" placeholder="DOI" class="input-box" bind:value={doi} />
                    <input type="text" placeholder="Título" class="input-box" bind:value={titulo} />
                    <input type="text" placeholder="Autor" class="input-box" bind:value={autor} />
                    <input type="number" placeholder="Ano de Publicação" class="input-box" bind:value={anoPublicacao} />
                    <textarea placeholder="Abstract" class="input-box content-box" bind:value={abstractText}></textarea>
                    <textarea placeholder="Bibliografia" class="input-box content-box" bind:value={bibliografia}></textarea>
                {/if}
                <button on:click={enviarDados} class="busca">Enviar</button>
            </div>
        {:else}
        <section>
            <div class="panel">
                <h2>Consulta de Dados</h2>
                <p>Escolha o campo e insira o valor para buscar registros existentes.</p>
                <select bind:value={campoDeBusca} class="input-box">
                    <option value="autor">Autor</option>
                    <option value="titulo">Título</option>
                    <option value="conteudo">Conteúdo</option>
                    <option value="ano">Ano de Publicação</option>
                    <option value="nacionalidade">Nacionalidade</option>
                </select>
                <div class="input-group">
                    <input
                        type="text"
                        placeholder="Digite o valor para busca"
                        class="input-box"
                        bind:value={valorParaBusca} />
                    <button on:click={buscar} class="busca">Buscar</button>
                </div>
            </div>
            <div class="panel">
                {#if resultado}
                    <h3>Resultado da Busca</h3>
                    <p><strong>Título:</strong> {resultado.titulo}</p>
                    <p><strong>Autor:</strong> {resultado.nome_autor}</p>
                    <p><strong>Ano de Publicação:</strong> {resultado.ano_publicacao}</p>
                    <p><strong>Resumo:</strong> {resultado.resumo}</p>
                    <p><strong>Nacionalidade:</strong> {resultado.nacionalidade}</p>
                {:else}
                    <p>Nenhum resultado encontrado.</p>
                {/if}
        </section>
        {/if}
    </section>
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 20px;
    }

    /* Style for tabs inside the <nav> */
    .tabs {
        display: flex;
        gap: 10px;
        margin-bottom: 20px;
    }
    .tabs > .tab {
        padding: 10px 20px;
        border: none;
        background: lightgray;
        cursor: pointer;
        font-size: 16px;
        border-radius: 5px;
    }
    .tabs > .tab.selected {
        background: darkgray;
        color: white;
    }

    /* Style for tabs inside the panels */
    .panel .tabs {
        display: flex;
        gap: 10px;
        margin-top: 10px;
    }
    .panel .tabs .tab {
        padding: 10px;
        border: none;
        background: #007bff;
        color: white;
        border-radius: 5px;
        cursor: pointer;
    }
    .panel .tabs .tab:hover {
        background: #0056b3;
    }
    
    .panel .tabs .tab.selected {
        background: #0056b3; /* Change background color when selected */
        color: white;
    }

    .panel {
        padding: 40px;
        border: 1px solid #ccc;
        border-radius: 10px;
        width: 100%;
        max-width: 1200px;
        text-align: center;
        margin: 20px auto;
    }

    /* Make the panel bigger for Inserção */
    .panel.insercao {
        max-width: 1000px; /* Increase the width */
        padding: 39px; /* Add more padding */
    }

    .input-box {
        width: 100%;
        max-width: 400px;
        padding: 10px;
        margin-top: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
        font-size: 16px;
    }

    .input-box:focus {
        outline: none;
        border-color: #007bff;
        box-shadow: 0 0 5px rgba(0, 123, 255, 0.5);
    }

    .bold {
        color:dimgrey;
    }

    /* Estilo para o botão "Buscar Autor" */
    button.busca {
        background-color: #ffcc00; /* Cor de fundo amarela escuro*/
        color: black; /* Cor do texto */
        border: none; /* Remove bordas */
        padding: 10px 20px; /* Espaçamento interno */
        font-size: 16px; /* Tamanho da fonte */
        border-radius: 5px; /* Bordas arredondadas */
        cursor: pointer; /* Cursor de ponteiro ao passar o mouse */
        transition: background-color 0.3s ease, transform 0.2s ease; /* Transições suaves */
    }

    button.busca:hover {
        background-color: #0056b3; /* Cor de fundo mais escura ao passar o mouse */
        transform: scale(1.05); /* Leve aumento no tamanho */
    }

    button.busca:active {
        background-color: #003f7f; /* Cor de fundo ainda mais escura ao clicar */
        transform: scale(0.95); /* Leve redução no tamanho ao clicar */
    }

    /* Estilo para o contêiner do input e botão */
    .input-group {
        display: flex; /* Usa Flexbox para alinhar os itens */
        align-items: center; /* Alinha verticalmente o input e o botão */
        gap: 10px; /* Espaçamento entre o input e o botão */
        justify-content: center; /* Centraliza o grupo horizontalmente */
        margin-top: 10px; /* Espaçamento acima do grupo */
    }

    .input-group .input-box {
        flex: 1; /* Faz o input ocupar o espaço restante */
    }

    .input-group .busca {
        flex-shrink: 0; /* Impede que o botão encolha */
    }
</style>
