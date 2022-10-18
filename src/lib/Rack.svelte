<script lang="ts">
  import type { Rack } from "../types";
  import Miner from "./Miner.svelte";
  export let rack: Rack = undefined;
  let ncols = rack.width;
</script>

<div>
    {rack.name}
    <div class="grid" style="--ncols: {ncols}">
        {#each rack.miners as row}
            {#each row as miner}
                <Miner {miner} />
            {/each}
        {/each}
    </div>
</div>

<style>
    .grid {
        display: grid;
        grid-template-columns: repeat(var(--ncols), 1fr);
        grid-gap: 10px;
        background-color: #2196F3;
        padding: 10px;
    }
</style>