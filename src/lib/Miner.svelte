<script lang="ts">
  import type { Miner } from "../types";
  import Check from "svelte-material-icons/Check.svelte";
  import { open } from "@tauri-apps/api/shell";
  import { round } from "../util";

  export let miner: Miner = undefined;
  export let disabled: Boolean = false;
  export let group: any[] = undefined;

  let isHovered = false;
  let x = undefined;
  let y = undefined;
  let selected = false;

  let color = "gray";

  function locate(ex: number, ey: number) {
    var tooltip = document.querySelectorAll(".tooltip")[0];
    if (tooltip) {
      var rect = tooltip.getBoundingClientRect();
      // Align the tooltip left of the mouse if it would go off the right side of the screen
      if (ex + 5 + rect.width > window.innerWidth) {
        x = ex - rect.width - 5;
      } else {
        x = ex + 5;
      }
      y = ey + 5;
    }
  }

  function mouseOver(event) {
    isHovered = true;
    locate(event.pageX, event.pageY);
  }

  function mouseMove(event) {
    locate(event.pageX, event.pageY);
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

  function onClick(event) {
    if (disabled) {
      return;
    }
    if (event.ctrlKey) {
      event.preventDefault();
      event.stopPropagation();
      if (group) {
        updateGroup(!selected, miner.ip);
      }
    } else {
      group = [miner.ip];
    }
  }

  function onDblClick(event) {
    if (disabled) {
      return;
    }
    open("http://" + miner.ip);
  }

  $: group && updateCheckbox(group, miner.ip);
  $: color = miner.make ? (miner.sleep ? "black" : (miner.hashrate > 0 ? "green" : "red")) : "#ddd";
  $: disabled = !miner.make;
</script>

<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
  class={miner.make && miner.locate ? "miner-blink" : "miner"}
  style="--bgcolor: {color}"
  on:click|preventDefault|stopPropagation={onClick}
  on:dblclick|preventDefault|stopPropagation={onDblClick}
  on:mouseover={mouseOver}
  on:mouseleave={mouseLeave}
  on:mousemove={mouseMove}
>
  {#if selected}
    <Check color="white" size="100%" />
  {/if}
</div>

{#if isHovered}
  <div class="tooltip" id="minertip" style="left: {x}px; top: {y}px">
    {#if miner.make}
      <div class="tooltip-header">
        <div class="tooltip-title">{miner.ip}</div>
        <div class="tooltip-subtitle">{miner.make + " " + miner.model}</div>
      </div>
      <div class="tooltip-body">
        <div class="tooltip-row">
          <div class="tooltip-label">MAC</div>
          <div class="tooltip-value">{miner.mac}</div>
        </div>
        <div class="tooltip-row">
          <div class="tooltip-label">Hashrate</div>
          <div class="tooltip-value">{round(miner.hashrate, 2)} TH/s</div>
        </div>
        <div class="tooltip-row">
          <div class="tooltip-label">Power</div>
          <div class="tooltip-value">{round(miner.power, 2)} W</div>
        </div>
        <div class="tooltip-row">
          <div class="tooltip-label">Temp</div>
          <div class="tooltip-value">{round(miner.temp, 2)} °C</div>
        </div>
        <div class="tooltip-row">
          <div class="tooltip-label">Fans</div>
          <div class="tooltip-value">
            {miner.fan ? miner.fan.join(",") : "Unknown"}
          </div>
        </div>
      </div>
      {#if miner.errors}
        <div class="tooltip-footer">
          {#each miner.errors as err}
            <div class="tooltip-error">{err}</div>
          {/each}
        </div>
      {/if}
      {#if miner.sleep}
        <div class="tooltip-footer">
          <div class="tooltip-error">Sleeping</div>
        </div>
      {/if}
    {:else}
      <div class="tooltip-header">
        <div class="tooltip-title">No Miner</div>
      </div>
    {/if}
  </div>
{/if}

<style>
  .miner {
    background-color: var(--bgcolor);
    border: 1px solid black;
    width: 1.5em;
    height: 1.5em;
  }

  .miner-blink {
    border: 1px solid black;
    width: 1.5em;
    height: 1.5em;
    animation: blink 1s infinite;
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

  .tooltip-error {
    font-size: 1rem;
    color: red;
    font-weight: bold;
  }
</style>
