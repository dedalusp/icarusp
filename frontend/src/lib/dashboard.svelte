<script lang="ts">
// filepath: /home/miki/IME/Intro a DS/icarusp/frontend/src/lib/dashboard.svelte

const BASE = "http://127.0.0.1:8080";

// Inserção de autor
let nomeAutor = "";
let anoNascimento: number | null = null;
let pais = "";

// Inserção de publicação
let titulo = "";
let anoPublicacao: number | null = null;
let resumo = "";
let autorPub = "";

// Buscas
let resumoBusca = "";
let tituloBusca = "";
let nomeBusca = "";

let resultado: any = null;
let mensagem = "";

let selectedTab: 'insercao' | 'consulta' = 'insercao';
let selectedInsercaoTab: 'autor' | 'publicacao' = 'autor';
let selectedConsultaTab: 'vetorial' | 'titulo' | 'publicacoesAutor' | 'autor' = 'vetorial';

// Funções para interagir com o backend
async function inserirAutor() {
    mensagem = "";
    const res = await fetch(`${BASE}/inserirAutor`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            nome: nomeAutor,
            ano_nascimento: anoNascimento,
            pais
        })
    });
    const data = await res.json();
    mensagem = data.id > 0 ? "Autor inserido com sucesso!" : "Erro ao inserir autor.";
}

async function inserirPublicacao() {
    mensagem = "";
    const res = await fetch(`${BASE}/inserirPublicacao`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            titulo,
            ano_publicacao: anoPublicacao,
            resumo,
            autor: autorPub
        })
    });
    const data = await res.json();
    mensagem = data.publicacao_id > 0 ? "Publicação inserida com sucesso!" : "Erro ao inserir publicação.";
}

async function buscaVetorial() {
    mensagem = "";
    const res = await fetch(`${BASE}/buscaVetorial?resumo=${encodeURIComponent(resumoBusca)}`);
    resultado = await res.json();
}

async function buscaPorPublicacoes() {
    mensagem = "";
    const res = await fetch(`${BASE}/buscaPorPublicacoes?titulo=${encodeURIComponent(tituloBusca)}`);
    resultado = await res.json();
}

async function buscaPorPublicacoesDoAutor() {
    mensagem = "";
    const res = await fetch(`${BASE}/buscaPorPublicacoesDoAutor?nome=${encodeURIComponent(nomeBusca)}`);
    resultado = await res.json();
}

async function buscaPorAutor() {
    mensagem = "";
    const res = await fetch(`${BASE}/buscaPorAutor?nome=${encodeURIComponent(nomeBusca)}`);
    resultado = await res.json();
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
            <div class="panel">
                <h2>Inserção de Dados</h2>
                <div class="tabs" style="justify-content: flex-start;">
                    <button class="tab" on:click={() => selectedInsercaoTab = 'autor'} class:selected={selectedInsercaoTab === 'autor'}>
                        Novo Autor
                    </button>
                    <button class="tab" on:click={() => selectedInsercaoTab = 'publicacao'} class:selected={selectedInsercaoTab === 'publicacao'}>
                        Nova Publicação
                    </button>
                </div>
                {#if selectedInsercaoTab === 'autor'}
                    <p class="bold">Insira os dados do novo autor:</p>
                    <div class="boxes-group">
                        <input type="text" placeholder="Nome" class="input-box" bind:value={nomeAutor} />
                        <input type="number" placeholder="Ano de Nascimento" class="input-box" bind:value={anoNascimento} />
                        <input type="text" placeholder="País de Nascimento" class="input-box" bind:value={pais} />
                        <button on:click={inserirAutor} class="busca">Inserir Autor</button>
                    </div>
                {:else}
                    <p class="bold">Insira os dados da nova publicação:</p>
                    <div class="boxes-group">
                        <input type="text" placeholder="Título" class="input-box" bind:value={titulo} />
                        <input type="number" placeholder="Ano de Publicação" class="input-box" bind:value={anoPublicacao} />
                        <input type="text" placeholder="Autor" class="input-box" bind:value={autorPub} />
                        <button on:click={inserirPublicacao} class="busca">Inserir Publicação</button>
                    </div>
                    <div class="boxes-group">
                        <textarea placeholder="Resumo de Conteúdo" class="input-box content-box" bind:value={resumo}></textarea>
                    </div>
                {/if}
            </div>
        {:else}
            <div class="panel">
                <h2>Consultas</h2>
                <div class="tabs" style="justify-content: flex-start;">
                    <button class="tab" on:click={() => selectedConsultaTab = 'vetorial'} class:selected={selectedConsultaTab === 'vetorial'}>
                        Por Resumo
                    </button>
                    <button class="tab" on:click={() => selectedConsultaTab = 'titulo'} class:selected={selectedConsultaTab === 'titulo'}>
                        Por Título
                    </button>
                    <button class="tab" on:click={() => selectedConsultaTab = 'publicacoesAutor'} class:selected={selectedConsultaTab === 'publicacoesAutor'}>
                        Publicações do Autor
                    </button>
                    <button class="tab" on:click={() => selectedConsultaTab = 'autor'} class:selected={selectedConsultaTab === 'autor'}>
                        Por Autor
                    </button>
                </div>
                {#if selectedConsultaTab === 'vetorial'}
                    <textarea
                        placeholder="Resumo de Conteúdo"
                        class="input-box consulta-content-box"
                        bind:value={resumo}
                    ></textarea>
                    <button on:click={buscaVetorial} class="busca">Buscar</button>       
                {:else if selectedConsultaTab === 'titulo'}
                    <div class="boxes-group">
                        <input type="text" placeholder="Título" class="input-box" bind:value={tituloBusca} />
                        <button on:click={buscaPorPublicacoes} class="busca">Buscar</button>
                    </div>
                {:else if selectedConsultaTab === 'publicacoesAutor'}
                    <div class="boxes-group">
                        <input type="text" placeholder="Nome do Autor" class="input-box" bind:value={nomeBusca} />
                        <button on:click={buscaPorPublicacoesDoAutor} class="busca">Buscar</button>
                    </div>
                {:else if selectedConsultaTab === 'autor'}
                    <div class="boxes-group">
                        <input type="text" placeholder="Nome do Autor" class="input-box" bind:value={nomeBusca} />
                        <button on:click={buscaPorAutor} class="busca">Buscar</button>
                    </div>
                {/if}
            </div>
        {/if}
        {#if mensagem}
            <div class="mensagem">{mensagem}</div>
        {/if}
        {#if resultado}
            <div class="panel resultado">
                <h3>Resultado</h3>
                <pre>{JSON.stringify(resultado, null, 2)}</pre>
            </div>
        {/if}
    </section>
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        align-items: flex-start; /* Alinha tudo à esquerda */
        padding: 20px;
    }

    .tabs-bar {
        width: 100%;
        background: #e0e0e0;
        text-align: left; /* Alinha tabs-bar à esquerda */
    }

    /* Style for tabs inside the <nav> */
    .tabs {
        display: flex;
        justify-content: flex-start; /* Alinha tabs à esquerda */
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
        background: #0056b3;
        color: white;
        font-weight: bold;
    }

    .panel {
        padding: 40px 0;
        width: 100%;
        max-width: 1200px;
        text-align: left; /* Alinha texto à esquerda */
        margin: 20px auto;
        border: none;
        background: none;
        box-shadow: none;
        border-radius: 0;
    }

    /* Make the panel bigger for Inserção */
    /*.panel.insercao {
        max-width: 1000px;
        padding: 39px;
    }*/

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
    .boxes-group {
        display: flex; /* Usa Flexbox para alinhar os itens */
        align-items: center; /* Alinha verticalmente o input e o botão */
        gap: 10px; /* Espaçamento entre o input e o botão */
        justify-content: center; /* Centraliza o grupo horizontalmente */
        margin-top: 10px; /* Espaçamento acima do grupo */
    }

    .boxes-group .input-box {
        flex: 1; /* Faz o input ocupar o espaço restante */
    }

    .boxes-group .busca {
        flex-shrink: 0; /* Impede que o botão encolha */
    }

    .mensagem {
        margin-top: 20px;
        padding: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
        background: #f9f9f9;
        color: #333;
        font-weight: bold;
    }

    .resultado {
        text-align: left;
        white-space: pre-wrap; /* Mantém quebras de linha */
        word-wrap: break-word; /* Quebra palavras longas */
    }

    .content-box {
        min-height: 400px;   /* Altura mínima maior */
        width: 100%;         /* Ocupa toda a largura disponível */
        max-width: 1200px;    /* Limite opcional de largura */
        resize: vertical;    /* Permite redimensionar verticalmente */
        margin-top: 10px;
        font-size: 16px;
    }

    .consulta-content-box {
        min-height: 50px;
        width: 100%;
        max-width: 1200px;
        resize: vertical;
        margin-top: 10px;
        font-size: 18px;
    }
</style>
