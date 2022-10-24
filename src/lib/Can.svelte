<script lang="ts">
  import Rack from "./Rack.svelte";
  import type { Rack as TRack } from "../types";
  export let miners: TRack[] = undefined;
  export let selection: any[] = undefined;

  let detected;
  let hashing;

  $: {
    if (miners) {
      detected = 0;
      hashing = 0;
      miners.forEach((rack: TRack) => {
        rack.miners.forEach((row: any) => {
          row.forEach((miner: any) => {
            if (miner.make) {
              detected++;
            }
            if (miner.hashrate) {
              hashing++;
            }
          });
        });
      });
    }
  }
</script>

<div style="margin: 0; margin-top: 10px;">
  <p style="margin: 0;">
    {hashing} Hashing / {detected} Detected
  </p>
  <div class="grid" style="--nracks: {miners.length}">
    {#each miners as rack}
      <Rack bind:selection {rack} />
    {/each}
  </div>
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(var(--nracks), 1fr);
    grid-gap: 10px;
    padding: 10px;
    margin: 0;
  }
</style>
