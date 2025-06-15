<script lang="ts">
  import Dashboard from './lib/dashboard.svelte';
  import IcarusLogo from './assets/Icarus_logo.png';
</script>

<main>
  <div class="header-bar">
    <img src={IcarusLogo} alt="Icarus Logo" class="logo-img" />
    <div class="header-title">Banco de Dados Bibliográficos</div>
  </div>
  <div>
    <Dashboard />
  </div>
</main>

<style>
  main {
    text-align: center;
    background: #fff;
    font-family: sans-serif, Helvetica;
  }

  .header-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: #268fa9;
    border-radius: 0 0 8px 8px;
    padding: 0.5em 1.5em;
    margin-bottom: 1.5em;
    min-width: 320px;
    color: #fff;
    height: 66px;
    box-shadow: 0 2px 8px #0001;
  }

  .header-title {
    font-size: 1.4em;
    font-weight: bold;
    color: #fff;
    text-align: right;
    flex: 1;
    letter-spacing: 0.5px;
  }

  .logo-img {
    height: 55px;
    width: auto;
    margin-left: 1.5em;
    object-fit: contain;
    background: white;
    border-radius: 4px;
    padding: 2px 8px;
    box-shadow: 0 1px 4px #0002;
  }
</style>
