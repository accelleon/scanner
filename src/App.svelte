<script lang="ts">
  import Controls from './lib/Controls.svelte';
  import type { Miner } from './types';
  import Can from './lib/Can.svelte';
  import { modal } from './stores.js';
  import Modal from './lib/controls/Modal.svelte';
  import { onMount } from 'svelte';
  import { listen } from '@tauri-apps/api/event';

  let selected_miners: Miner[] = [];
  let miners: any = [];

  onMount (() => {
    let unlisten = listen('miner', (e: any) => {
      let miner = e.payload;
      console.log(miners, miner.rack, miner.row, miner.index);
      miners[miner.rack].miners[miner.row][miner.index] = miner.miner;
    });
  });
</script>

<main class="container">
  <Modal show={$modal} />
  <Controls bind:miners = {miners}/>
  <Can miners = {miners} bind:selection = {selected_miners}/>
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