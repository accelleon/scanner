<script lang="ts">
  import type { Rack } from "../types";
  import Miner from "./Miner.svelte";
  import _ from 'lodash';

  export let rack: Rack = undefined;
  export let selection: any[] = undefined;
  let ncols = rack.width;

  function selectAll(e: any) {
    let new_selection = rack.miners.flat(2).filter((miner) => miner.make);
    // if ctrl is held we want to add/remove
    // otherwise toggle it
    if (e.ctrlKey) {
      // check if the rack is already selected
      if (_.intersection(selection, new_selection).length === new_selection.length) {
        // remove it
        selection = _.difference(selection, new_selection);
      } else {
        // add it
        selection = _.union(selection, new_selection);
      }
    } else {
      // toggle it on/off
      if (_.isEqual(selection, new_selection)) {
        selection = [];
      } else {
        selection = new_selection;
      }
    }
  }
</script>

<div class="container" on:dblclick={selectAll}>
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
    background-color: #ddd;
    text-align: center;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(var(--ncols), 1fr);
    grid-gap: 10px;
    padding: 10px;
  }
</style>
