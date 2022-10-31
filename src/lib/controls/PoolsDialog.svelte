<script lang="ts">
  import { getContext } from "svelte";
  import _ from "lodash";
  import { pools } from "../../stores";
  import { invoke } from "@tauri-apps/api/tauri";

  const { close } = getContext<{ close: any }>('simple-modal');

  let selected = null;
  let editing = {
    name: "",
    url1: "",
    url2: "",
    url3: "",
    worker: "",
  };

  function onDone() {
    close();
  }

  function update() {
    return invoke("save_pools", { pools: $pools });
  }

  function onAdd() {
    // No duplicate names
    if ($pools.find((p) => p.name === editing.name)) {
      return;
    }

    $pools.push({ ...editing });
    $pools = $pools;
    editing = {
      name: "",
      url1: "",
      url2: "",
      url3: "",
      worker: "",
    };
    update();
  }

  function onEdit() {
    // No duplicate names
    if (pools[selected].name !== editing.name && $pools.find((p) => p.name === editing.name)) {
      return;
    }

    $pools[selected] = {...editing};
    editing = {
      name: "",
      url1: "",
      url2: "",
      url3: "",
      worker: "",
    };
    selected = null;
    update();
  }

  function onRemove() {
    $pools.splice(selected, 1);
    editing = {
      name: "",
      url1: "",
      url2: "",
      url3: "",
      worker: "",
    };
    selected = null;
    update();
  }

  function onSelect(i) {
    selected = i;
    editing = {...$pools[selected]};
  }

  function onCancel() {
    editing = {
      name: "",
      url1: "",
      url2: "",
      url3: "",
      worker: "",
    };
    selected = null;
  }

</script>

<h2>Pools</h2>
<hr />
<div class="grid">
  <div>
    <table>
      <thead>
        <th>Name</th>
      </thead>
      <tbody>
        {#each $pools as pool, i}
        <tr>
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <td
            class={selected === i ? "selected" : ""}
            on:click={() => onSelect(i)}
          >
            {pool.name}
          </td>
        </tr>
        {/each}
      </tbody>
    </table>
  </div>
  <div class="col">
    <div class="row">Name:   <input type="text" bind:value={editing.name}/></div>
    <div class="row">Pool 1: <input type="text" bind:value={editing.url1}/></div>
    <div class="row">Pool 2: <input type="text" bind:value={editing.url2}/></div>
    <div class="row">Pool 3: <input type="text" bind:value={editing.url3}/></div>
    <div class="row">Worker: <input type="text" bind:value={editing.worker}/></div>
    <div>
      {#if selected === null}
        <button
          disabled={!editing.name || !editing.url1 || !editing.worker}
          on:click={onAdd}
        >Add</button>
      {/if}
      {#if selected !== null}
        <button on:click={onCancel}>Cancel</button>
        <button
          disabled={_.isEqual(pools[selected], editing)}
          on:click={onEdit}
        >Save</button>
        <button on:click={onRemove}>Remove</button>
      {/if}
    </div>
    <p>
      Worker names may contain placeholders to be filled in with miner details:
    </p>
      <ul>
        <li><code>{`{can}`}</code> - Currently selected can name</li>
        <li><code>{`{model}`}</code> - Miner's model</li>
        <li><code>{`{ip}`}</code> - Last 2 octets of the miner's IP address (e.g. 4x126)</li>
      </ul>
  </div>
</div>
<div class="buttons">
  <button on:click={onDone}> Close </button>
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: 1fr 2fr;
    grid-gap: 1rem;
  }

  table {
    width: 100%;
  }

  table,
  th,
  td {
    border: 1px solid black;
    border-collapse: collapse;
  }

  th,
  td {
    padding: 5px;
    text-align: left;
  }

  
  tbody > tr:nth-child(odd) {
    background-color: #c2c2c2;
  }
  
  tr:hover td {
    background-color: #a8a8a8;
  }
  
  td.selected {
    background-color: rgb(146, 146, 146);
  }

  .buttons {
    display: flex;
    justify-content: flex-end;
  }

  .buttons button {
    margin-left: 1rem;
  }

  .col {
    display: flex;
    flex-direction: column;
  }

  .row {
    display: grid;
    grid-template-columns: 0.5fr 2.5fr;
    align-items: center;
  }

  .col > div {
    margin-bottom: 1rem;
  }

  .col > div > input {
    margin-left: 1rem;
  }

  .col > p {
    margin-top: 1rem;
  }

  /* .col {
    display: flex;
    flex-direction: column;
  } */

  /* .buttons {
    display: flex;
    justify-content: space-between;
    margin-top: 10px;
  } */
</style>
