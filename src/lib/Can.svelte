<script lang="ts">
  import Rack from "./Rack.svelte";
  import type { Rack as TRack } from "../types";
  import { round } from "../util";

  export let miners: TRack[] = undefined;
  export let selection: any[] = undefined;
  export let pool: any = undefined;
  export let can: any = undefined;

  let detected;
  let hashing;
  let hashrate;
  let sel_model = false;
  let sel_profile = false;

  function sameModel() {
    if (selection.length) {
      let model = selection[0].model;
      return selection.every((miner) => miner.model == model);
    }
    return false;
  }

  function sameProfile() {
    if (selection.length && sameModel()) {
      let profile = selection[0].profile;
      return selection.every((miner) => miner.profile == profile);
    }
    return false;
  }

  function selectNotHashing() {
    selection = miners
      .map((rack) => rack.miners)
      .flat(2)
      .filter((miner) => !miner.hashrate && miner.make);
  }

  function selectSleeping() {
    selection = miners
      .map((rack) => rack.miners)
      .flat(2)
      .filter((miner) => miner.sleep && miner.make);
  }

  function selectAll() {
    selection = miners
      .map((rack) => rack.miners)
      .flat(2)
      .filter((miner) => miner.make);
  }

  function selectModel() {
    if (sameModel()) {
      let model = selection[0].model;
      selection = miners
        .map((rack) => rack.miners)
        .flat(2)
        .filter((miner) => miner.model == model && miner.make);
    }
  }

  function selectProfile() {
    if (sameProfile()) {
      let model = selection[0].model;
      let profile = selection[0].profile;
      selection = miners
        .map((rack) => rack.miners)
        .flat(2)
        .filter((miner) => miner.model == model && miner.make && miner.profile.name == profile.name);
    }
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

  $: {
    if (selection.length) {
      sel_model = sameModel();
      sel_profile = sameProfile();
    } else {
      sel_model = false;
      sel_profile = false;
    }
  }
</script>

<div style="margin: 0; margin-top: 10px;">
  <p style="margin: 0;">
    {hashing} Hashing / {detected} Detected @ {format_hashrate(hashrate)}
  </p>
  <div class="grid" style="--nracks: {miners.length}">
    {#each miners as rack}
      <Rack bind:selection {rack} {pool} {can}/>
    {/each}
  </div>
  <div class="footer">
    <div>
      <button disabled={!sel_model} on:click={selectModel}>Select Model</button>
      <button disabled={!sel_profile} on:click={selectProfile}>Select Profile</button>
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
        <div class="miner" style="--bgcolor: orange;"></div>
        <div class="legend-text">Hashing with Error</div>
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
