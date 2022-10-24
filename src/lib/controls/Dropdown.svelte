<script lang="ts">
  export let options: Array<string | object> = undefined;
  export let selObject: boolean = false;
  export let selected: string | object = undefined;
  export let labelfn: Function = (e: any) => e?.label || e;
  export let disabled: boolean = false;

  export { className as class };
  let className = "";
</script>

<select bind:value={selected} class={className} {disabled}>
  {#each options as option, i}
    <option value={selObject ? option : labelfn(option)}>
      {labelfn(option)}
    </option>
  {/each}
</select>

<style>
  select {
    width: 100%;
    height: 2.5rem;
    border: 1px solid #ccc;
    border-radius: 0.25rem;
    padding: 0.25rem;
  }

  select:focus {
    outline: none;
    border-color: #007bff;
    box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
  }

  select:disabled {
    background-color: #e9ecef;
    color: #6c757d;
  }

  select:disabled:focus {
    box-shadow: none;
  }

  select:disabled:hover {
    cursor: not-allowed;
  }

  select:hover {
    cursor: pointer;
  }
</style>
