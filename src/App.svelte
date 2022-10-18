<script lang="ts">
  import Controls from './lib/Controls.svelte';
  import Can from './lib/Can.svelte'
</script>

<main class="container">
  <Controls />
  <Can/>
</main>

<style>
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
  }
</style>