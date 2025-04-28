<script lang="ts">
    let selectedTab: 'insercao' | 'consulta' = 'insercao';
    let selectedInsercaoTab: 'autor' | 'livro' | 'artigo' = 'autor';
    let selectedConsultaTab: 'autor' | 'titulo' | 'conteudo' | 'ano' | 'nacionalidade' = 'autor';
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
                    <input type="text" placeholder="Nome" class="input-box" />
                    <input type="number" placeholder="Ano de Nascimento" class="input-box" />
                    <input type="text" placeholder="País de Nascimento" class="input-box" />
                {:else if selectedInsercaoTab === 'livro'}
                    <p class="bold">Insira os dados do novo livro:</p>
                    <input type="text" placeholder="ISBN" class="input-box" />
                    <input type="text" placeholder="Título" class="input-box" />
                    <input type="text" placeholder="Autor" class="input-box" />
                    <input type="number" placeholder="Ano de Publicação" class="input-box" />
                    <textarea placeholder="Resumo de Conteúdo" class="input-box content-box"></textarea>
                    <input type="number" placeholder="Número de Edições" class="input-box" />
                {:else if selectedInsercaoTab === 'artigo'}
                    <p class="bold">Insira os dados do novo artigo:</p>
                    <input type="text" placeholder="DOI" class="input-box" />
                    <input type="text" placeholder="Título" class="input-box" />
                    <input type="text" placeholder="Autor" class="input-box" />
                    <input type="number" placeholder="Ano de Publicação" class="input-box" />
                    <textarea placeholder="Abstract" class="input-box content-box"></textarea>
                    <textarea placeholder="Bibliografia" class="input-box content-box"></textarea>
                {/if}
            </div>
        {:else}
            <div class="panel">
                <h2>Consulta de Dados</h2>
                <p>Escolha a categoria para visualizar os registros existentes.</p>
                <div class="tabs">
                    <button class="tab" on:click={() => selectedConsultaTab = 'autor'} class:selected={selectedConsultaTab === 'autor'}>
                        Autor
                    </button>
                    <button class="tab" on:click={() => selectedConsultaTab = 'titulo'} class:selected={selectedConsultaTab === 'titulo'}>
                        Título
                    </button>
                    <button class="tab" on:click={() => selectedConsultaTab = 'conteudo'} class:selected={selectedConsultaTab === 'conteudo'}>
                        Conteúdo
                    </button>
                    <button class="tab" on:click={() => selectedConsultaTab = 'ano'} class:selected={selectedConsultaTab === 'ano'}>
                        Ano de publicação
                    </button>
                    <button class="tab" on:click={() => selectedConsultaTab = 'nacionalidade'} class:selected={selectedConsultaTab === 'nacionalidade'}>
                        Nacionalidade do autor
                    </button>
                </div>
                {#if selectedConsultaTab === 'autor'}
                    <input type="text" placeholder="Digite o nome do autor" class="input-box" />
                {:else if selectedConsultaTab === 'titulo'}
                    <input type="text" placeholder="Digite o título" class="input-box" />
                {:else if selectedConsultaTab === 'conteudo'}
                    <input type="text" placeholder="Digite o conteúdo" class="input-box" />
                {:else if selectedConsultaTab === 'ano'}
                    <input type="number" placeholder="Digite o ano de publicação" class="input-box" />
                {:else if selectedConsultaTab === 'nacionalidade'}
                    <input type="text" placeholder="Digite o país" class="input-box" />
                {/if}
            </div>
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
        padding: 20px;
        border: 1px solid #ccc;
        border-radius: 10px;
        width: 100%;
        max-width: 600px;
        text-align: center;
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
</style>
