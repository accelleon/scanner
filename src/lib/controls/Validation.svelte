<script lang="ts">
  import { getContext } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { Circle } from 'svelte-loading-spinners';
  import type { Rack, Miner } from "../../types";

  export let racks: Rack[];

  const { close } = getContext<{ close: any }>("simple-modal");
  let working = false;
  let column = -1;
  let step = "";
  let progress;

  onMount(() => {
    advance();
  })

  async function setLocate(ips: string[], locate: boolean) {
    if (!working) {
      working = true;
      await invoke("run_job", { job: { job: "Locate", ips, locate }}).finally(() => {
        working = false;
      });
    }
  }

  function getAll() {
    return racks
      .map((rack) => rack.miners)
      .flat(2)
      .filter((miner) => miner.make)
      .map((miner) => miner.ip);
  }

  function getColumn(i: number) {
    // Return i column of ips from each rack
    return racks.map((rack) => 
      rack.miners.map((row) => row[i])
      .filter((m) => m.make)
      .map((m) => m.ip)
    ).flat();
  }

  async function advance() {
    if (column < racks[0].width-1) {
      if (column < 0) {
        step = "Disabling locate on all miners";
        await setLocate(getAll(), false);
      } else {
        step = `Disabling locate on column ${column + 1}`;
        await setLocate(getColumn(column), false);
      }
      column++;
      step = `Enabling locate on column ${column + 1}`;
      await setLocate(getColumn(column), true);
    } else {
      step = "Disabling locate on all miners";
      await setLocate(getAll(), false);
      step = "Done";
      close();
    }
    step = `Please ensure only the miners on column ${column+1} of each rack are locating`;
  }
</script>

<div class="grid">
  <h3>{step}</h3>
  {#if working}
    <Circle/>
  {/if}
  <button on:click={advance} disabled={working}>Advance</button>
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: 1fr;
    grid-gap: 1rem;
    align-items: center;
    justify-items: center;
  }
</style>
