<script lang="ts">
  export let options: Array<string | object> = undefined;
  export let selObject: Boolean = false;
  export let selected: string | object = undefined;
  export let labelfn: Function = (e: any) => e?.label || e;

  export { className as class };
  let className = "";
</script>

<select bind:value={selected} class={className}>
  {#each options as option, i}
    <option value={selObject ? option : labelfn(option)}>
      {labelfn(option)}
    </option>
  {/each}
</select>
