<script lang="ts">

const BASE = "http://127.0.0.1:8080";

// Inserção de autor
let nomeAutor = "";
let anoNascimento: number | null = null;
let pais = "";

// Inserção de publicação
let titulo = "";
let anoPublicacao: number | null = null;
let resumo = "";
let autorPub: number | null = null;
let autoresPub = "";

// Buscas
let resumoBusca = "";
let tituloBusca = "";
let nomeBusca = "";

// Variáveis de mensagem específicas para cada consulta
let mensagemVetorial = "";
let mensagemTitulo = "";
let mensagemPublicacoesAutor = "";
let mensagemAutor = "";

// Variáveis de resultado específicas para cada consulta
let resultadoVetorial: any = null;
let resultadoTitulo: any = null;
let resultadoPublicacoesAutor: any = null;
let resultadoAutor: any = null;

let mensagem = "";

let selectedTab: 'insercao' | 'consulta' = 'insercao';
let selectedInsercaoTab: 'autor' | 'publicacao' = 'autor';
let selectedConsultaTab: 'vetorial' | 'titulo' | 'publicacoesAutor' | 'autor' = 'vetorial';

// Funções para interagir com o backend
async function inserirAutor() {
    mensagem = "";
    if (!nomeAutor || !anoNascimento || !pais) {
        mensagem = "Preencha todos os campos para inserir o autor.";
        setTimeout(() => { mensagem = ""; }, 1500);
        return;
    }
    const res = await fetch(`${BASE}/insert/author`, { // Atualizado para o novo endpoint
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            name: nomeAutor,
            birth_year: anoNascimento,
            country: pais
        })
    });
    const data = await res.json();
    mensagem = data.success ? "Autor inserido com sucesso!" : `Erro ao inserir autor.`;
    setTimeout(() => { mensagem = ""; }, 1500);
}

async function inserirPublicacao() {
    mensagem = "";
    if (!titulo || !anoPublicacao || !resumo || !autoresPub) {
        mensagem = "Preencha todos os campos para inserir a publicação.";
        setTimeout(() => { mensagem = ""; }, 1500);
        return;
    }

    // Converter os IDs dos autores em uma lista de números
    const authorsIds = autoresPub.split(",").map(id => parseInt(id.trim())).filter(id => !isNaN(id));

    if (authorsIds.length === 0) {
        mensagem = "Insira pelo menos um ID de autor válido.";
        setTimeout(() => { mensagem = ""; }, 1500);
        return;
    }

    // Inserir publicação
    const res = await fetch(`${BASE}/insert/book`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
            title: titulo,
            publication_year: anoPublicacao,
            abstract_text: resumo
        })
    });

    const data = await res.json();
    if (data.success) {
        mensagem = "Publicação inserida com sucesso!";
        const bookId = data.data.id; // ID da publicação retornada pelo backend

        // Fazer o link entre publicação e autores
        const linkRes = await fetch(`${BASE}/insert/book-author-link`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                book_id: bookId,
                authors_ids: authorsIds // Lista de IDs dos autores como números
            })
        });

        const linkData = await linkRes.json();
        if (linkData.success) {
            mensagem += " Autores vinculados à publicação com sucesso!";
        } else {
            mensagem += ` Erro ao vincular autor à publicação.`;
        }
    } else {
        mensagem = `Erro ao inserir publicação.`;
    }

    setTimeout(() => { mensagem = ""; }, 1500);
}

async function buscaVetorial() {
    mensagem = "";
    mensagemVetorial = "";
    resultadoVetorial = null;
    try {
        const res = await fetch(`${BASE}/search/book/embedding`, { // Atualizado para o novo endpoint
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                query: resumoBusca,
                limit: 10
            })
        });
        const data = await res.json();
        resultadoVetorial = data.data;
        mensagemVetorial = (Array.isArray(data.data) && data.data.length === 0) ? "Nenhum resultado encontrado." : "";
        if (mensagemVetorial) setTimeout(() => { mensagemVetorial = ""; }, 1500);
    } catch (e) {
        mensagem = "Erro ao buscar.";
        setTimeout(() => { mensagem = ""; }, 1500);
    }
}

async function buscaPorPublicacoes() {
    mensagemTitulo = "";
    mensagem = "";
    resultadoTitulo = null;
    try {
        const res = await fetch(`${BASE}/search/books`, { // Atualizado para o novo endpoint
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                title: tituloBusca
            })
        });
        const data = await res.json();
        resultadoTitulo = data.data;
        mensagemTitulo = (Array.isArray(data.data) && data.data.length === 0) ? "Nenhum resultado encontrado." : "";
        if (mensagemTitulo) setTimeout(() => { mensagemTitulo = ""; }, 1500);
    } catch (e) {
        mensagem = "Erro ao buscar.";
        setTimeout(() => { mensagem = ""; }, 1500);
    }
}

async function buscaPorPublicacoesDoAutor() {
    mensagemPublicacoesAutor = "";
    mensagem = "";
    resultadoPublicacoesAutor = null;
    try {
        const res = await fetch(`${BASE}/search/author/books`, { // Atualizado para o novo endpoint
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                author_name: nomeBusca
            })
        });
        const data = await res.json();
        resultadoPublicacoesAutor = data.data;
        mensagemPublicacoesAutor = (Array.isArray(data.data) && data.data.length === 0) ? "Nenhum resultado encontrado." : "";
        if (mensagemPublicacoesAutor) setTimeout(() => { mensagemPublicacoesAutor = ""; }, 1500);
    } catch (e) {
        mensagem = "Erro ao buscar.";
        setTimeout(() => { mensagem = ""; }, 1500);
    }
}

async function buscaPorAutor() {
    mensagemAutor = "";
    mensagem = "";
    resultadoAutor = null;
    try {
        const res = await fetch(`${BASE}/search/authors`, { // Atualizado para o novo endpoint
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                name: nomeBusca
            })
        });
        const data = await res.json();
        resultadoAutor = data.data;
        mensagemAutor = (Array.isArray(data.data) && data.data.length === 0) ? "Nenhum resultado encontrado." : "";
        if (mensagemAutor) setTimeout(() => { mensagemAutor = ""; }, 1500);
    } catch (e) {
        mensagem = "Erro ao buscar.";
        setTimeout(() => { mensagem = ""; }, 1500);
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
                        <input
                            type="text"
                            placeholder="IDs dos Autores (use ',')"
                            class="input-box"
                            bind:value={autoresPub}
                        />
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
                        bind:value={resumoBusca}
                    ></textarea>
                    <button on:click={buscaVetorial} class="busca">Buscar</button>
                    {#if mensagemVetorial}
                        <div class="mensagem">{mensagemVetorial}</div>
                    {/if}
                    {#if resultadoVetorial}
                        <div class="panel resultado">
                            <h3>Resultado</h3>
                            <table class="resultado-tabela">
                                <thead>
                                    <tr>
                                        <th>ID</th>
                                        <th>Título</th>
                                        <th>Ano de Publicação</th>
                                        <th>Resumo</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {#if Array.isArray(resultadoVetorial)}
                                        {#each resultadoVetorial as item, _}
                                            <tr>
                                                <td>{item.id}</td>
                                                <td>{item.title}</td>
                                                <td>{item.publication_year}</td>
                                                <td>{item.abstract_text}</td>
                                            </tr>
                                        {/each}
                                    {:else}
                                        <tr>
                                            <td>{resultadoVetorial.id}</td>
                                            <td>{resultadoVetorial.title}</td>
                                            <td>{resultadoVetorial.publication_year}</td>
                                            <td>{resultadoVetorial.abstract_text}</td>
                                        </tr>
                                    {/if}
                                </tbody>
                            </table>
                        </div>
                    {/if}    
                {:else if selectedConsultaTab === 'titulo'}
                    <div class="boxes-group">
                        <input type="text" placeholder="Título" class="input-box" bind:value={tituloBusca} />
                        <button on:click={buscaPorPublicacoes} class="busca">Buscar</button>
                    </div>
                    {#if mensagemTitulo}
                        <div class="mensagem">{mensagemTitulo}</div>
                    {/if}
                    {#if resultadoTitulo}
                        <div class="panel resultado">
                            <h3>Resultado</h3>
                            <table class="resultado-tabela">
                                <thead>
                                    <tr>
                                        <th>ID</th>
                                        <th>Título</th>
                                        <th>Ano de Publicação</th>
                                        <th>Resumo</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {#if Array.isArray(resultadoTitulo)}
                                        {#each resultadoTitulo as item, _}
                                            <tr>
                                                <td>{item.id}</td>
                                                <td>{item.title}</td>
                                                <td>{item.publication_year}</td>
                                                <td>{item.abstract_text}</td>
                                            </tr>
                                        {/each}
                                    {:else}
                                        <tr>
                                            <td>{resultadoTitulo.id}</td>
                                            <td>{resultadoTitulo.title}</td>
                                            <td>{resultadoTitulo.publication_year}</td>
                                            <td>{resultadoTitulo.abstract_text}</td>
                                        </tr>
                                    {/if}
                                </tbody>
                            </table>
                        </div>
                    {/if}   
                {:else if selectedConsultaTab === 'publicacoesAutor'}
                    <div class="boxes-group">
                        <input type="text" placeholder="Nome do Autor" class="input-box" bind:value={nomeBusca} />
                        <button on:click={buscaPorPublicacoesDoAutor} class="busca">Buscar</button>
                    </div>
                    {#if mensagemPublicacoesAutor}
                        <div class="mensagem">{mensagemPublicacoesAutor}</div>
                    {/if}
                    {#if resultadoPublicacoesAutor}
                        <div class="panel resultado">
                            <h3>Resultado</h3>
                            <table class="resultado-tabela">
                                <thead>
                                    <tr>
                                        <th>ID</th>
                                        <th>Título</th>
                                        <th>Ano de Publicação</th>
                                        <th>Resumo</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {#if Array.isArray(resultadoPublicacoesAutor)}
                                        {#each resultadoPublicacoesAutor as item, _}
                                            <tr>
                                                <td>{item.id}</td>
                                                <td>{item.title}</td>
                                                <td>{item.publication_year}</td>
                                                <td>{item.abstract_text}</td>
                                            </tr>
                                        {/each}
                                    {:else}
                                        <tr>
                                            <td>{resultadoPublicacoesAutor.id}</td>
                                            <td>{resultadoPublicacoesAutor.title}</td>
                                            <td>{resultadoPublicacoesAutor.publication_year}</td>
                                            <td>{resultadoPublicacoesAutor.abstract_text}</td>
                                        </tr>
                                    {/if}
                                </tbody>
                            </table>
                        </div>
                    {/if}   
                {:else if selectedConsultaTab === 'autor'}
                    <div class="boxes-group">
                        <input type="text" placeholder="Nome do Autor" class="input-box" bind:value={nomeBusca} />
                        <button on:click={buscaPorAutor} class="busca">Buscar</button>
                    </div>
                    {#if mensagemAutor}
                        <div class="mensagem">{mensagemAutor}</div>
                    {/if}
                    {#if resultadoAutor}
                        <div class="panel resultado">
                            <h3>Resultado</h3>
                            <table class="resultado-tabela">
                                <thead>
                                    <tr>
                                        <th>ID</th>
                                        <th>Nome</th>
                                        <th>Ano de Nascimento</th>
                                        <th>País</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {#if Array.isArray(resultadoAutor)}
                                        {#each resultadoAutor as item, _}
                                            <tr>
                                                <td>{item.id}</td>
                                                <td>{item.name}</td>
                                                <td>{item.birth_year}</td>
                                                <td>{item.country}</td>
                                            </tr>
                                        {/each}
                                    {:else}
                                        <tr>
                                            <td>{resultadoAutor.id}</td>
                                            <td>{resultadoAutor.name}</td>
                                            <td>{resultadoAutor.birth_year}</td>
                                            <td>{resultadoAutor.country}</td>
                                        </tr>
                                    {/if}
                                </tbody>
                            </table>
                        </div>
                    {/if}   
                {/if}
            </div>
        {/if}
        {#if mensagem}
            <div class="mensagem">{mensagem}</div>
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
        width: 100%;
        max-width: 1200px;
        text-align: left; /* Alinha texto à esquerda */
        margin: 20px auto;
        border: none;
        background: none;
        box-shadow: none;
        border-radius: 0;
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

    .resultado-tabela {
        width: 100%;
        border-collapse: collapse; /* Remove espaços entre as bordas */
        margin-top: 10px;
        font-size: 14px;
        text-align: left;
    }

    .resultado-tabela th, .resultado-tabela td {
        border: 1px solid #ddd; /* Bordas suaves */
        padding: 8px; /* Espaçamento interno */
    }

    .resultado-tabela th {
        background-color: #f4f4f4; /* Fundo para cabeçalhos */
        font-weight: bold; /* Texto em negrito */
    }

    .resultado-tabela tr:nth-child(even) {
        background-color: #f9f9f9; /* Fundo alternado para linhas pares */
    }

    .resultado-tabela tr:hover {
        background-color: #f1f1f1; /* Fundo ao passar o mouse */
    }
</style>
