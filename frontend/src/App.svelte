<script lang="ts">
  import Dashboard from './lib/dashboard.svelte';
</script>

<main>
  <h1>Icarus</h1>
  <div>
    <Dashboard />
  </div>
</main>
<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: black;
    font-size: calc(10px + 2vmin);
    margin-bottom: 0.5em;
    text-transform: uppercase;
  }

  @media (prefers-color-scheme: dark) {
    main {
      color: white;
    }
  }
  @media (prefers-color-scheme: light) {
    main {
      color: black;
    }
  }
  @media (prefers-color-scheme: no-preference) {
    main {
      color: black;
    }
  }
  @media (prefers-color-scheme: dark) {
    h1 {
      color: white;
    }
  }
  @media (prefers-color-scheme: light) {
    h1 {
      color: black;
    }
  }
  @media (prefers-color-scheme: no-preference) {
    h1 {
      color: black;
    }
  }
</style>
