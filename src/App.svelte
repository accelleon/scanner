<script lang="ts">
  import Controls from './lib/Controls.svelte';
  import type { Miner } from './types';
  import Can from './lib/Can.svelte'

  let selected_miners: Miner[] = [];
  let miners: any = [];
</script>

<main class="container">
  <Controls bind:miners = {miners}/>
  <Can miners = {miners} bind:selection = {selected_miners}/>

  <div>
    {#each selected_miners as miner}
      {JSON.stringify(miner)}
    {/each}
  </div>
</main>

<style>
  .container {
    display: grid;
    grid-template-rows: 0.2fr 0.25fr auto auto;
    grid-template-columns: 1fr;
    align-items: baseline;
    margin: 0;
    padding: 0;
  }
</style>