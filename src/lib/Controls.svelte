<script lang="ts">
  import { onMount } from "svelte";
  import Dropdown from "./controls/Dropdown.svelte";
  import { Tabs, TabList, TabPanel, Tab } from "./controls/tabs/tabs.js";
  import { invoke } from "@tauri-apps/api/tauri";
  import FrontierDialog from "./controls/FrontierDialog.svelte";
  import { modal } from "../stores.js";
  import { bind } from "./controls/Modal.svelte";
  import { listen } from "@tauri-apps/api/event";

  export let miners: any = undefined;
  let cans = [];
  let selected;
  let scanned;
  let scanning = false;
  let disabled = false;
  let progress;

  onMount(async () => {
    cans = await invoke("get_cans");
    let unlisten = listen("progress", (e: any) => {
      progress = e.payload;
    });
  });

  async function scan() {
    let res = await invoke("scan_miners_async", { can: selected.id });
    return res;
  }

  async function scanMiners() {
    scanning = true;
    scanned = selected;
    await invoke("gen_empty_can", { can: selected.id }).then((resp: any) => {
      miners = resp.racks;
      scan().then(() => {
        scanning = false;
      });
    });
  }

  const onOkay = (layout: String, sitemap: String) => {
    invoke("import_frontier_locations", { layout: layout, sitemap: sitemap });
  };

  async function importMap() {
    modal.set(bind(FrontierDialog, { onOkay }));
  }

  $: if (selected != scanned) {
    scanned = false;
    invoke("gen_empty_can", { can: selected.id }).then((resp: any) => {
      miners = resp.racks;
    });
  }
  $: disabled = scanning || !selected;
</script>

{#if progress}
  <progress class="progress" value={progress.value}>
    {progress.job}... {progress.done} / {progress.max}
  </progress>
{/if}
<div class="container">
  <div class="col">
    <Dropdown
      bind:selected
      options={cans}
      selObject={true}
      labelfn={(e) => e.name}
      class="dropdown"
    />
    <button on:click={scanMiners} {disabled}>Scan</button>
    <button on:click={importMap}>Import Map</button>
  </div>
  <div class="divider" />
  <Tabs>
    <TabList>
      <Tab>Controls</Tab>
      <Tab>Pools</Tab>
      <Tab>Tab 3</Tab>
    </TabList>
    <TabPanel>
      <button>Start Locating</button>
      <button>Locate Selected</button>
    </TabPanel>
    <TabPanel>
      <div class="pool">
        <label>
          Pool 1:
          <input type="text" value="tcp+stratum://btcfoundry.us:3333" />
        </label>
        <label>
          Worker:
          <input type="text" value="pc{'{'}model{'}'}." />
        </label>
        <label>
          Password:
          <input type="text" value="" />
        </label>
        <label>
          IP Suffix:
          <input type="checkbox" />
        </label>
        <label>
          Pool 2:
          <input type="text" value="tcp+stratum://btcfoundry.us:3333" />
        </label>
        <label>
          Worker:
          <input type="text" value="pc{'{'}model{'}'}." />
        </label>
        <label>
          Password:
          <input type="text" value="" />
        </label>
        <div />
        <label>
          Pool 3:
          <input type="text" value="tcp+stratum://btcfoundry.us:3333" />
        </label>
        <label>
          Worker:
          <input type="text" value="pc{'{'}model{'}'}." />
        </label>
        <label>
          Password:
          <input type="text" value="" />
        </label>
        <div />
        <button>Set All</button>
        <button>Set Selected</button>
      </div>
    </TabPanel>
    <TabPanel>
      <h2>Tab 3</h2>
      <p>Tab 3 content</p>
    </TabPanel>
  </Tabs>
</div>

<style>
  * :global(.dropdown) {
    width: 200px;
    height: 3em;
  }

  .divider {
    border-left: 2px solid grey;
    height: 200px;
    margin: 0 20px;
  }

  .progress {
    width: 100%;
  }

  .container {
    display: grid;
    grid-template-columns: 0.05fr 0.05fr auto;
    grid-template-rows: auto;
    margin-top: 0;
    margin-left: 10px;
    margin-right: 10px;
    margin-bottom: 0;
    padding: 0;
    padding-bottom: 10px;
    border-bottom: 2px solid grey;
    justify-content: left;
  }

  .col {
    display: flex;
    flex-direction: column;
  }

  .pool {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
  }

  button[disabled]:active,
  button[disabled],
  button[disabled]:hover {
    background-color: grey;
    color: darkgrey;
    cursor: not-allowed !important;
    pointer-events: none;
  }
</style>
