<script lang="ts">
  import { onMount } from "svelte";
  import Dropdown from "./controls/Dropdown.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import SettingsDialog from "./controls/SettingsDialog.svelte";
  import { modal } from "../stores.js";
  import { bind } from "./controls/Modal.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { settings } from "../stores.js";
  import { open, save } from "@tauri-apps/api/dialog";
  import type { Miner, Rack, Profile } from '../types';
  import { writeTextFile } from '@tauri-apps/api/fs';
  import PoolsDialog from "./controls/PoolsDialog.svelte";
  import { pools } from "../stores.js";
  import Validation from "./controls/Validation.svelte";
  import { pretty_profile } from "../util";
  import RangeSlider from "./controls/RangeSlider.svelte";

  export let miners: any = undefined;
  export let selection: any[];
  export let pool: any = undefined;
  export let selected;

  let cans = [];
  let scanned;
  let working = false;
  let progress;
  let monitor;
  let prof_err;
  let profiles: Profile[] = [];
  let profile: Profile;

  var setIntervalSynchronous = function (func, delay) {
    var intervalFunction, timeoutId, clear, cancel;
    // Call to clear the interval.
    clear = function () {
      // Since we run the function immediately without a timeout
      // we need to cancel the timeout that would have been created
      cancel = true;
      clearTimeout(timeoutId);
    };
    intervalFunction = function () {
      func().finally(() => {
        if (!cancel) {
          timeoutId = setTimeout(intervalFunction, delay);
        }
      });
    }
    intervalFunction();
    // You should capture the returned function for clearing.
    return clear;
  };

  onMount(async () => {
    cans = await invoke("get_cans");
    if (cans.length > 0) {
      selected = cans[0];
    }
    let unlisten = listen("progress", (e: any) => {
      progress = e.payload;
    });
  });

  function scan() {
    return invoke("run_job", { job: { job: "Scan", can: selected.id }});
  }

  async function scanMiners() {
      if (!working) {
        invoke("gen_empty_can", { can: selected.id }).then((resp: any) => {
          miners = resp.racks;
          working = true;
          scanned = selected;
          scan().finally(() => {
            working = false;
          });
        });
      } else {
        invoke("cancel_job");
      }
  }

  async function monitorMiners() {
    if (!monitor) {
      monitor = setIntervalSynchronous(scan, $settings.refreshRate * 1000);
    } else {
      invoke("cancel_job").then(() => {
        monitor();
        monitor = undefined;
      });
    }
  }

  async function runJob(job: string, args?: any) {
    console.log("runJob", job, args);
    if (!working) {
      working = true;
      await invoke("run_job", { job: { job: job, ips: selection.map((m) => m.ip), ...args }}).finally(() => {
        working = false;
      });
    } else {
      invoke("cancel_job");
    }
  }

  async function settingsDialog() {
    modal.set(bind(SettingsDialog));
  }

  async function poolsDialog() {
    modal.set(bind(PoolsDialog));
  }

  async function validationDialog() {
    modal.set(bind(Validation, { racks: miners }));
  }

  async function exportMiners() {
    save({
      filters: [{name: "csv", extensions: ["csv"]}],
    }).then((path) => {
      const headers = ["IP", "Rack", "Shelf", "Slot", "Make", "Model", "Hashrate", "Errors", "CF_MAC Address", "Worker", "Pool1", "Pool2", "Pool3"].join(",");
      var contents = miners.flatMap((r: Rack, i: number) => {
        return r.miners.flatMap((row: Miner[], y: number) => {
          return row.map((m: Miner, x: number) => {
            return m.make ? 
              `${m.ip},${i+1},${y+1},${(y*4)+x+1},${m.make},${m.model},${m.hashrate},${m.nameplate},${m.errors ? m.errors.join("; ") : ""},${m.mac ? m.mac.toLowerCase() : ""},${m.pools[0].user},${m.pools[0].url},${m.pools[1].url},${m.pools[2].url}`
              : `${m.ip},${i+1},${y+1},${(y*4)+x+1},,,,,,,`;
          });
        });
      });
      contents = contents.filter((c) => c != "")
      const outfile = [headers, ...contents].join("\n");
      writeTextFile({
        contents: outfile,
        path: path,
      });
    });
  }

  async function exportMiners2() {
    save({
      filters: [{name: "csv", extensions: ["csv"]}],
    }).then((path) => {
      const headers = ["IP", "MAC", "Make", "Model", "Hashrate", "Errors"].join(",");
      const contents = miners.flatMap((r: Rack, i: number) => {
        return r.miners.flatMap((row: Miner[], y: number) => {
          return row.map((m: Miner, x: number) => {
            if (m.make) {
              return `${m.ip},${m.mac ? m.mac.toLowerCase() : ""},${m.make},${m.model},${m.hashrate},${m.errors ? m.errors.join("; ") : ""}`;
            }
          });
        });
      });
      const outfile = [headers, ...contents].join("\n");
      writeTextFile({
        contents: outfile,
        path: path,
      });
    });
  }

  async function exportLogs() {
    open({
      directory: true,
    }).then((path) => {
      runJob("Log", { path: path });
    });
  }

  $: if (selected != scanned) {
    scanned = false;
    profiles = [];
    profile = undefined;
    selection = [];
    invoke("gen_empty_can", { can: selected.id }).then((resp: any) => {
      miners = resp.racks;
    });
  }

  $: control_disabled = monitor || working || selection.length == 0;

  $: {
    // prof_err = undefined;
    // if selection.length && profiles.length == 0 not supported
    // if miners have mixed models not supported

    if (selection.length > 0) {
      profiles = selection[0].profiles || [];
      if (profiles.length == 0) {
        prof_err = "No profiles available";
      } else if (selection.find((m: Miner) => m.model != selection[0].model)) {
        prof_err = "Miners have mixed models";
      } else {
        prof_err = undefined;
      }
    } else {
      profiles = [];
      profile = undefined;
      prof_err = undefined;
    }

  }

  $: {
    console.log(profile);
  }
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
      labelfn={(e) => `${e.name} (${e.num})`}
      class="dropdown"
      disabled = {working || monitor}
    />
    <button on:click={scanMiners} disabled={monitor || !selected}> {working ? "Cancel" : "Scan" }</button>
    <button on:click={monitorMiners} disabled={working || !selected}>{monitor ? "Stop Monitoring" : "Monitor"}</button>
    <button on:click={settingsDialog} disabled={working || monitor}>Settings</button>
  </div>
  <div class="divider" />
  <div class="control">
    <div>
      <h3>Controls</h3>
      <hr />
      <div class="control-grid">
        <div class="control-group">
          <button disabled={control_disabled} on:click={() => runJob("Locate", {"locate": true})}>Locate On</button>
          <button disabled={control_disabled} on:click={() => runJob("Locate", {"locate": false})}>Locate Off</button>
        </div>
        <div class="control-group">
          <button disabled={control_disabled} on:click={() => runJob("Sleep", {"sleep": true})}>Sleep</button>
          <button disabled={control_disabled} on:click={() => runJob("Sleep", {"sleep": false})}>Wake</button>
        </div>
        <div class="control-group">
          <button disabled={control_disabled} on:click={() => runJob("Reboot")}>Reboot</button>
        </div>
      </div>
    </div>
    <div>
      <h3>Pools</h3>
      <hr />
      <div class="pool">
        <Dropdown bind:selected={pool} options={$pools} selObject={true} labelfn={(e) => e.name} class="dropdown" />
        <button disabled={control_disabled || !pool} on:click={() => runJob("Pool", {"pool": pool})}>Update Selected</button>
        <button on:click={() => poolsDialog()}>Edit Pools</button>
      </div>
    </div>
    <div>
      <h3>Tools</h3>
      <hr />
      <div class="col">
        <button disabled={monitor || working || !scanned} on:click={() => validationDialog()}>Location Validation</button>
        <button disabled={working || !scanned} on:click={() => exportMiners2()}>Export Miners</button>
        <button disabled={control_disabled} on:click={() => exportLogs()}>Export Logs</button>
      </div>
    </div>
    <div>
      <h3>Profiles</h3>
      <hr />
      <div class="control-group">
        <div class="col">
          <Dropdown bind:selected={profile} options={profiles} selObject={true} labelfn={(e) => pretty_profile(e)} class="dropdown" />
          <button disabled={control_disabled || !profile || prof_err} on:click={() => runJob("Profile", {"profile": profile})}>Update Selected</button>
          {#if prof_err}
            <p style="color: red;">{prof_err}</p>
          {/if}
        </div>
        <div class="col">
          Voltage
          <RangeSlider values={[profile?.volt || 0]} min={profile?.min_volt || 0} max={profile?.max_volt || 0} step={5} formatter={(v,_i,_p) => v/100} all='label' float={true} on:change={(e) => profile.volt = e.detail.value} disabled={!profile || profile.type!="Manual"}/>
          Frequency
          <RangeSlider values={[profile?.freq || 0]} min={profile?.min_freq || 0} max={profile?.max_freq || 0} step={5} all='label' float={true} on:change={(e) => profile.freq = e.detail.value} disabled={!profile || profile.type!="Manual"}/>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  * :global(.dropdown) {
    width: 100%;
    height: 3em;
    margin-bottom: 5px;
  }

  .divider {
    border-left: 2px solid grey;
    height: 100%;
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
    min-width: 200px;
  }

  .col > * {
    margin-bottom: 5px;
  }

  h3 {
    margin: 0;
    margin-bottom: 5px;
  }

  .control {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 2fr;
    height: fit-content;
  }

  .control > * {
    margin: 0 10px;
    padding: 10px;
    border: 1px solid grey;
  }

  .control-grid {
    display: grid;
    grid-template-rows: auto auto auto;
  }

  .control-group {
    display: grid;
    margin-top: 5px;
    grid-template-columns: 1fr 1fr;
  }

  .control-group > * {
    margin-right: 5px;
  }

  .pool {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  .pool > * {
    margin-bottom: 5px;
  }
</style>
