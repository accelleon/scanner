<script lang="ts">
  import Rack from "./Rack.svelte";
  import type { Rack as TRack } from "../types";
  import { round } from "../util";

  export let miners: TRack[] = undefined;
  export let selection: any[] = undefined;

  let detected;
  let hashing;
  let hashrate;

  function selectNotHashing() {
    selection = miners
      .map((rack) => rack.miners)
      .flat(2)
      .filter((miner) => !miner.hashrate && miner.make)
      .map((miner) => miner.ip);
  }

  function selectSleeping() {
    selection = miners
      .map((rack) => rack.miners)
      .flat(2)
      .filter((miner) => miner.sleep && miner.make)
      .map((miner) => miner.ip);
  }

  function selectAll() {
    selection = miners
      .map((rack) => rack.miners)
      .flat(2)
      .filter((miner) => miner.make)
      .map((miner) => miner.ip);
  }

  function format_hashrate(ths) {
    if (ths < 1000) {
      return `${round(ths, 2)} TH/s`;
    } else if (ths < 1000000) {
      return `${round((ths / 1000), 2)} PH/s`;
    }
  }

  $: {
    if (miners) {
      detected = 0;
      hashing = 0;
      hashrate = 0;
      miners.forEach((rack: TRack) => {
        rack.miners.forEach((row: any) => {
          row.forEach((miner: any) => {
            if (miner.make) {
              detected++;
            }
            if (miner.hashrate) {
              hashing++;
              hashrate += miner.hashrate;
            }
          });
        });
      });
    }
  }
</script>

<div style="margin: 0; margin-top: 10px;">
  <p style="margin: 0;">
    {hashing} Hashing / {detected} Detected @ {format_hashrate(hashrate)}
  </p>
  <div class="grid" style="--nracks: {miners.length}">
    {#each miners as rack}
      <Rack bind:selection {rack} />
    {/each}
  </div>
  <div class="footer">
    <div>
      <button on:click={selectNotHashing}>Select Not Hashing</button>
      <button on:click={selectSleeping}>Select Sleeping</button>
      <button on:click={selectAll}>Select All</button>
    </div>

    <div class="legend">
      <div class="legend-item">
        <div class="miner" style="--bgcolor: #ddd;"></div>
        <div class="legend-text">Not Detected</div>
      </div>
      <div class="legend-item">
        <div class="miner" style="--bgcolor: green;"></div>
        <div class="legend-text">Hashing</div>
      </div>
      <div class="legend-item">
        <div class="miner" style="--bgcolor: red;"></div>
        <div class="legend-text">Not Hashing</div>
      </div>
      <div class="legend-item">
        <div class="miner" style="--bgcolor: black;"></div>
        <div class="legend-text">Sleeping</div>
      </div>
      <div class="legend-item">
        <div class="miner-blink" style="--bgcolor: red;"></div>
        <div class="legend-text">Locating</div>
      </div>
    </div>
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

  .miner {
    background-color: var(--bgcolor);
    border: 1px solid black;
    width: 1.5em;
    height: 1.5em;
  }

  .miner-blink {
    border: 1px solid black;
    width: 1.5em;
    height: 1.5em;
    animation: blink 1s infinite;
  }

  .footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .legend {
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
    align-items: center;
    margin: 0;
    padding: 0;
  }

  .legend-item {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    margin: 5px;
    padding: 0;
  }

  .legend-text {
    margin: 0;
    padding: 0;
    margin-left: 5px;
  }
</style>
