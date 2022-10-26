<script lang="ts">
  import { getContext } from "svelte";
  import { settings } from "../../stores";
  import { invoke } from "@tauri-apps/api/tauri";
  import FileInput from "./FileInput.svelte";
  import { Circle } from 'svelte-loading-spinners';

  const { close } = getContext<{ close: any }>("simple-modal");
  let values = $settings;
  let layout;
  let sitemap;
  let working = false;

  function onCancel() {
    close();
  }

  async function onOkay() {
    working = true;
    settings.set(values);
    await invoke("save_settings", { settings: values });
    working = false;
    close();
  }

  async function importMap() {
    working = true;
    await invoke("import_frontier_locations", { layout: layout, sitemap: sitemap });
    working = false;
    close();
  };
</script>

<h2>Settings</h2>
<hr />
{#if !working}
  <div class="col">
    <div class="row">
      Monitor Refresh Rate (seconds):
      <input type="number" bind:value={values.refreshRate} />
    </div>
    <div>
      Adjusting settings below may result in poor detection:
    </div>
    <div class="row">
      Connection Timeout (seconds):
      <input type="number" bind:value={values.connectionTimeout} />
    </div>
    <div class="row">
      Read Timeout (seconds):
      <input type="number" bind:value={values.readTimeout} />
    </div>
    <div class="row">
      Max Connections:
      <input type="number" bind:value={values.maxConnections} />
    </div>
  </div>
  <hr />
  <h2>Import Layout and Sitemap</h2>
  <p class="warning">This will wipe and replace the existing sitemap!</p>
  Layout
  <FileInput
    bind:value={layout}
    filters={[{ name: "csv", extensions: ["csv"] }]}
    multiple={false}
  />
  Sitemap
  <FileInput
    bind:value={sitemap}
    filters={[{ name: "csv", extensions: ["csv"] }]}
    multiple={false}
  />
  <button on:click={importMap}> Import Map </button>
  <div class="buttons">
    <button on:click={onCancel}> Cancel </button>
    <button on:click={onOkay}> Save </button>
  </div>
{:else}
  <Circle />
{/if}

<style>
  h2 {
    font-size: 2rem;
    text-align: center;
  }

  .col {
    display: grid;
    grid-template-columns: auto;
  }

  .col > * {
    margin-bottom: 5px;
  }

  .row {
    display: grid;
    grid-template-columns: auto auto;
    justify-content: space-between;
    align-items: center;
  }

  .buttons {
    display: flex;
    justify-content: space-between;
    margin-top: 10px;
  }

  .warning {
    color: red;
    font-weight: bold;
  }
</style>
