<script lang="ts">
  import type { Rack } from "../types";
  import Miner from "./Miner.svelte";
  export let rack: Rack = undefined;
  export let selection: any[] = undefined;
  let ncols = rack.miners[0].length;
</script>

<div class="container">
  {rack.name}
  <div class="grid" style="--ncols: {ncols}">
    {#each rack.miners as row}
      {#each row as miner}
        <Miner bind:group={selection} {miner} />
      {/each}
    {/each}
  </div>
</div>

<style>
  .container {
    padding: 0;
    border: 1px solid black;
    max-width: min-content;
    text-align: center;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(var(--ncols), 1fr);
    grid-gap: 10px;
    background-color: #212ff3;
    padding: 10px;
  }
</style>
