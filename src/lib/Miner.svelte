<script lang="ts">
  import type { Miner } from "../types";
  import Check from 'svelte-material-icons/Check.svelte';

  export let miner: Miner = undefined;
  export let disabled: Boolean = false;
  export let group: any[] = undefined;

  let isHovered = false;
  let x = undefined;
  let y = undefined;
  let selected = false;

  let color = "gray";
  if (miner.make) {
    color = miner.hashrate > 0 ? "green" : "red";
  }

  function mouseOver(event) {
    isHovered = true;
    x = event.pageX + 5;
    y = event.pageY + 5;
  }

  function mouseMove(event) {
    x = event.pageX + 5;
    y = event.pageY + 5;
  }

  function mouseLeave(event) {
    isHovered = false;
  }

  function updateCheckbox(group, value) {
    selected = group.indexOf(value) > -1;
  }

  function updateGroup(checked, value) {
    const index = group.indexOf(value);
    if (checked) {
      if (index === -1) {
        group.push(value);
        group = group;
      }
    } else {
      if (index !== -1) {
        group.splice(index, 1);
        group = group;
      }
    }
  }

  $: group && updateCheckbox(group, miner);
  $: group && updateGroup(selected, miner);
</script>

<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
  class="miner"
  style="--bgcolor: {color}"
  on:click={() => (selected = !selected)}
  on:mouseover={mouseOver}
  on:mouseleave={mouseLeave}
  on:mousemove={mouseMove}
>
  {#if selected}
    <Check
      color=white
      size="100%"
    />
  {/if}
</div>

{#if isHovered}
  {#if miner.make}
    <div class="tooltip" style="left: {x}px; top: {y}px">
      <div class="tooltip-header">
        <div class="tooltip-title">{miner.make}</div>
        <div class="tooltip-subtitle">{miner.model}</div>
      </div>
      <div class="tooltip-body">
        <div class="tooltip-row">
          <div class="tooltip-label">Hashrate</div>
          <div class="tooltip-value">{miner.hashrate} TH/s</div>
        </div>
        <div class="tooltip-row">
          <div class="tooltip-label">Temp</div>
          <div class="tooltip-value">{miner.temp} °C</div>
        </div>
        <div class="tooltip-row">
          <div class="tooltip-label">Fan</div>
          <div class="tooltip-value">{miner.fan} RPM</div>
        </div>
        <div class="tooltip-row">
          <div class="tooltip-label">Power</div>
          <div class="tooltip-value">{miner.power} W</div>
        </div>
      </div>
    </div>
  {:else}
    <div class="tooltip" style="left: {x}px; top: {y}px">
      <div class="tooltip-header">
        <div class="tooltip-title">No Miner</div>
      </div>
    </div>
  {/if}
{/if}

<style>
  .miner {
    background-color: var(--bgcolor);
    border: 1px solid black;
    width: 1.5em;
    height: 1.5em;
    justify-content: center;
    align-items: center;
    display: flex;
  }

  .tooltip {
    position: absolute;
    background-color: white;
    border: 1px solid black;
    padding: 10px;
    font-size: 1rem;
  }
  .tooltip-header {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .tooltip-title {
    font-size: 1.2rem;
    font-weight: bold;
  }

  .tooltip-subtitle {
    font-size: 1rem;
  }

  .tooltip-body {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .tooltip-row {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
  }

  .tooltip-label {
    font-size: 1rem;
    font-weight: bold;
    margin-right: 10px;
  }

  .tooltip-value {
    font-size: 1rem;
  }
</style>