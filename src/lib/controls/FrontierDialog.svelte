<script lang="ts">
  import { getContext } from "svelte";
  import FileInput from "./FileInput.svelte";
  export let onCancel = () => {};
  export let onOkay = (layout: String, sitemap: String) => {};

  const { close } = getContext<{ close: any }>("simple-modal");

  let layout;
  let sitemap;

  function _onCancel() {
    onCancel();
    close();
  }

  function _onOkay() {
    onOkay(layout, sitemap);
    close();
  }
</script>

<h2>Import Layout and Sitemap</h2>
<p class="warning">This will wipe and replace the existing sitemap!</p>
<label>
  Layout
  <!-- <input type="file" bind:value={layout} /> -->
  <FileInput
    bind:value={layout}
    filters={[{ name: "csv", extensions: ["csv"] }]}
    multiple={false}
  />
</label>
<label>
  Sitemap
  <!-- <input type="file" bind:value={sitemap} /> -->
  <FileInput
    bind:value={sitemap}
    filters={[{ name: "csv", extensions: ["csv"] }]}
    multiple={false}
  />
</label>

<div class="buttons">
  <button on:click={_onCancel}> Cancel </button>
  <button on:click={_onOkay}> Okay </button>
</div>

<style>
  h2 {
    font-size: 2rem;
    text-align: center;
  }

  input {
    width: 100%;
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
