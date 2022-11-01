<script lang="ts">
  import { getContext } from "svelte";
  import { settings } from "../../stores";
  import { invoke } from "@tauri-apps/api/tauri";
  import FileInput from "./FileInput.svelte";
  import { Circle } from 'svelte-loading-spinners';
  import { onMount } from "svelte";

  const { close } = getContext<{ close: any }>("simple-modal");
  let values = $settings;
  let layout;
  let sitemap;
  let working = false;
  let miner_auth = [];

  onMount(() => {
    invoke("get_miner_auth").then((res: any[]) => {
      console.log(res);
      miner_auth = res;
    });
  });

  function onCancel() {
    close();
  }

  async function onOkay() {
    working = true;
    settings.set(values);
    await invoke("save_settings", { settings: values });
    // Remove empty make from miner_auth
    miner_auth = miner_auth.filter((m) => m.make);
    await invoke("save_miner_auth", { auths: miner_auth });
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

<div class="dialog">
  <h2>Settings</h2>
  <hr />
  {#if !working}
  <div class="settings">
    <div class="settings__col">
      <div class="settings__row">
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
        <h3>Import Layout and Sitemap</h3>
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
      </div>
    </div>
    <div class="settings__col">
      <div class="settings__row">
        <h3>Miner Authentication</h3>
        <table>
          <thead>
            <tr>
              <th>Make</th>
              <th>Username</th>
              <th>Password</th>
              <th><button on:click={() => {miner_auth.push({ make: "", username: "", password: "" }); miner_auth = miner_auth;}}>+</button></th>
            </tr>
          </thead>
          <tbody>
            {#each miner_auth as auth, i}
            <tr>
              <td contenteditable="true" bind:innerHTML={miner_auth[i].make} />
              <td contenteditable="true" bind:innerHTML={miner_auth[i].username} />
              <td contenteditable="true" bind:innerHTML={miner_auth[i].password} />
              <td>
                <button on:click={() => {miner_auth.splice(i, 1); miner_auth = miner_auth;}}>-</button>
              </td>
            </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
    <div class="buttons">
      <button on:click={onCancel}> Cancel </button>
      <button on:click={onOkay}> Save </button>
    </div>
  {:else}
  <div class="loading">
    <Circle />
  </div>
  {/if}
</div>

<style>
  h2 {
    font-size: 2rem;
    text-align: center;
  }

  h3 {
    font-size: 1.5rem;
  }

  .settings {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-gap: 1rem;
  }

  .settings__col {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  .settings__row {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  table,
  th,
  td {
    border: 1px solid black;
    border-collapse: collapse;
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

  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
  }
</style>
