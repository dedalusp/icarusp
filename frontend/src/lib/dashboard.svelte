<script lang="ts">
    let selectedTab: 'insercao' | 'consulta' = 'insercao';
    let selectedInsercaoTab: 'autor' | 'publicacao' = 'autor';
    let valorParaBusca: string = ""; // Valor digitado pelo usuário
    let campoDeBusca: 'autor' | 'titulo' = 'autor'; // Campo selecionado
    let resultado: { titulo: string; nome_autor: string; ano_publicacao: number; resumo: string; nacionalidade: string } | null = null;

    let autor = "";
    let anoNascimento: number | null = null;
    let pais = "";
    let titulo = "";
    let anoPublicacao: number | null = null;
    let resumo = "";
    let prompt = "";

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
            autor,
            ano_nascimento: anoNascimento,
            pais,
            titulo,
            ano_publicacao: anoPublicacao,
            resumo,
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
    <div class="tabs-bar">
        <nav class="tabs">
            <button class="tab" on:click={() => selectedTab = 'insercao'} class:selected={selectedTab === 'insercao'}>
                Inserção
            </button>
            <button class="tab" on:click={() => selectedTab = 'consulta'} class:selected={selectedTab === 'consulta'}>
                Consulta
            </button>
        </nav>
    </div>
    <section>
        {#if selectedTab === 'insercao'}
            <div class="panel insercao">
                <h2>Inserção de Dados</h2>
                <p>Escolha entre adicionar um novo Autor ou Publicacao.</p>
                <div class="tabs">
                    <button class="tab" on:click={() => selectedInsercaoTab = 'autor'} class:selected={selectedInsercaoTab === 'autor'}>
                        Novo Autor
                    </button>
                    <button class="tab" on:click={() => selectedInsercaoTab = 'publicacao'} class:selected={selectedInsercaoTab === 'publicacao'}>
                        Novo Publicacao
                    </button>
                </div>
                {#if selectedInsercaoTab === 'autor'}
                    <p class="bold">Insira os dados do novo autor:</p>
                    <input type="text" placeholder="Nome" class="input-box" bind:value={autor} />
                    <input type="number" placeholder="Ano de Nascimento" class="input-box" bind:value={anoNascimento} />
                    <input type="text" placeholder="País de Nascimento" class="input-box" bind:value={pais} />
                    <button on:click={buscaAutor} class="busca">Buscar Autor</button>
                {:else if selectedInsercaoTab === 'publicacao'}
                    <p class="bold">Insira os dados do novo livro:</p>
                    <input type="text" placeholder="Título" class="input-box" bind:value={titulo} />
                    <input type="text" placeholder="Autor" class="input-box" bind:value={autor} />
                    <input type="number" placeholder="Ano de Publicação" class="input-box" bind:value={anoPublicacao} />
                    <textarea placeholder="Resumo de Conteúdo" class="input-box content-box" bind:value={resumo}></textarea>
                    <button on:click={buscaPublicacao} class="busca">Buscar Publicacao</button>
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

    .tabs-bar {
        width: 100%;
        background: #e0e0e0;
    }

    /* Style for tabs inside the <nav> */
    .tabs {
        display: flex;
    }
    .tabs > .tab {
        padding: 10px 20px;
        border: none;
        background: transparent;
        cursor: pointer;
        font-size: 16px;
        color: #222;
        transition: background 0.2s, color 0.2s;
    }

    .tabs > .tab:focus {
        outline: none;
        box-shadow: none;
    }

    .tabs > .tab:hover {
        color:rgb(255, 123, 0);
    }

    /* Style for tabs inside the panels */
    .panel .tabs {
        display: flex;
        gap: 10px;
        margin-top: 10px;
        justify-content: center; /* Centraliza as tabs horizontalmente */
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
        background: #fff;
        color: #222;
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
