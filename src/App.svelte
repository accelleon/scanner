<script lang="ts">
  import Controls from "./lib/Controls.svelte";
  import type { Miner } from "./types";
  import Can from "./lib/Can.svelte";
  import { modal } from "./stores.js";
  import Modal from "./lib/controls/Modal.svelte";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import Table from "./lib/controls/Table.svelte";
  import { round } from "./util";

  let selected_miners: Miner[] = [];
  let miners: any = [];
  let tab_miners: any = [];

  let columns = [
    {
      name: "IP",
      key: "ip",
      sortable: true,
      sortFn: (a: any, b: any) => {
        const num1 = Number(
          a
            .split(".")
            .map((num) => `000${num}`.slice(-3))
            .join("")
        );
        const num2 = Number(
          b
            .split(".")
            .map((num) => `000${num}`.slice(-3))
            .join("")
        );
        return num1 - num2;
      },
    },
    {
      name: "Make",
      key: "make",
      sortable: true,
    },
    {
      name: "Model",
      key: "model",
      sortable: true,
    },
    {
      name: "MAC Address",
      key: "mac",
    },
    {
      name: "Hashrate",
      key: "hashrate",
      sortable: true,
      dispFn: (hashrate: number) => `${round(hashrate, 2)}`,
    },
    {
      name: "Temp",
      key: "temp",
      sortable: true,
      dispFn: (temp: number) => `${round(temp, 2)}`,
    },
    {
      name: "Fan",
      key: "fan",
    },
    {
      name: "Pools",
      key: "pools",
      dispFn: (pools: any) => {
        return pools
          .map((pool: any) => pool.url + " " + pool.user)
          .join("<br>");
      },
    },
  ];

  onMount(() => {
    let unlisten = listen("miner", (e: any) => {
      let miner = e.payload;
      miners[miner.rack].miners[miner.row][miner.index] = miner.miner;
    });
  });

  $: {
    let rack_miners = miners.map((rack: any) => {
      return rack.miners.flat();
    });
    tab_miners = rack_miners.flat().filter((miner: Miner) => {
      return miner.make;
    });
  }
</script>

<main class="container">
  <Modal show={$modal} />
  <Controls bind:miners />
  <Can {miners} bind:selection={selected_miners} />
  <Table
    {columns}
    data={tab_miners}
    unique="ip"
    bind:selection={selected_miners}
  />
</main>

<style>
  .container {
    display: grid;
    grid-template-rows: 0.2fr 0.25fr auto auto;
    grid-template-columns: 1fr;
    align-items: baseline;
    margin: 0;
    padding: 0;
    text-align: center;
  }
</style>
