<script lang="ts">
  import { open, type DialogFilter } from "@tauri-apps/api/dialog";

  export let filters: DialogFilter[] = undefined;
  export let multiple: boolean = undefined;

  export let value: String | String[] = undefined;

  async function _open() {
    const path = await open({
      filters: filters,
      multiple: multiple,
    });
    value = path;
  }
</script>

<div class="container">
  <div class="col">
    <button on:click={_open}>Open</button>
    <p class="selection">{value ? value : "No selection"}</p>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: left;
    border: 1px solid black;
    border-radius: 5px;
    padding: 0px;
  }

  .col {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: left;
  }

  .selection {
    margin: 0;
    margin-left: 5px;
  }
</style>
