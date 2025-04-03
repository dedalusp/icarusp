<script lang="ts">
    let selectedTab: 'insercao' | 'consulta' = 'insercao';
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
            <div class="panel">
                <h2>Inserção de Dados</h2>
                <p>Escolha entre adicionar um novo Autor, Livro ou Artigo.</p>
                <div class="options">
                    <button class="option">Novo Autor</button>
                    <button class="option">Novo Livro</button>
                    <button class="option">Novo Artigo</button>
                </div>
            </div>
        {:else}
            <div class="panel">
                <h2>Consulta de Dados</h2>
                <p>Escolha a categoria para visualizar os registros existentes.</p>
                <div class="options">
                    <button class="option">Autores</button>
                    <button class="option">Livros</button>
                    <button class="option">Artigos</button>
                </div>
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
    .tabs {
        display: flex;
        gap: 10px;
        margin-bottom: 20px;
    }
    .tab {
        padding: 10px 20px;
        border: none;
        background: lightgray;
        cursor: pointer;
        font-size: 16px;
        border-radius: 5px;
    }
    .tab.selected {
        background: darkgray;
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
    .options {
        display: flex;
        justify-content: center;
        gap: 10px;
        margin-top: 10px;
    }
    .option {
        padding: 10px;
        border: none;
        background: #007bff;
        color: white;
        border-radius: 5px;
        cursor: pointer;
    }
</style>
