<script lang="ts">
  import { onMount } from "svelte";
  import Dropdown from "./controls/Dropdown.svelte";
  import { Tabs, TabList, TabPanel, Tab } from "./controls/tabs/tabs.js";
  import { invoke } from "@tauri-apps/api/tauri";
  import SettingsDialog from "./controls/SettingsDialog.svelte";
  import { modal } from "../stores.js";
  import { bind } from "./controls/Modal.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { settings } from "../stores.js";

  export let miners: any = undefined;
  export let selection: any[];
  let cans = [];
  let selected;
  let scanned;
  let working = false;
  let progress;
  let monitor;

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
      await invoke("run_job", { job: { job: job, ips: selection.map((m: any) => m.ip), ...args }}).finally(() => {
        working = false;
      });
    } else {
      invoke("cancel_job");
    }
  }

  async function settingsDialog() {
    modal.set(bind(SettingsDialog));
  }

  $: if (selected != scanned) {
    scanned = false;
    invoke("gen_empty_can", { can: selected.id }).then((resp: any) => {
      miners = resp.racks;
    });
  }

  $: control_disabled = monitor || working || selection.length == 0;
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
      labelfn={(e) => e.name}
      class="dropdown"
      disabled = {working || monitor}
    />
    <button on:click={scanMiners} disabled={monitor || !selected}> {working ? "Cancel" : "Scan" }</button>
    <button on:click={monitorMiners} disabled={working || !selected}>{monitor ? "Stop Monitoring" : "Monitor"}</button>
    <button on:click={settingsDialog} disabled={working || monitor}>Settings</button>
  </div>
  <div class="divider" />
  <Tabs>
    <TabList>
      <Tab>Controls</Tab>
      <Tab>Pools</Tab>
      <Tab>Tab 3</Tab>
    </TabList>
    <TabPanel>
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
    </TabPanel>
    <TabPanel>
      <div class="pool">
        <label>
          Pool 1:
          <input type="text" value="tcp+stratum://btcfoundry.us:3333" />
        </label>
        <label>
          Worker:
          <input type="text" value="pc{'{'}model{'}'}." />
        </label>
        <label>
          Password:
          <input type="text" value="" />
        </label>
        <label>
          IP Suffix:
          <input type="checkbox" />
        </label>
        <label>
          Pool 2:
          <input type="text" value="tcp+stratum://btcfoundry.us:3333" />
        </label>
        <label>
          Worker:
          <input type="text" value="pc{'{'}model{'}'}." />
        </label>
        <label>
          Password:
          <input type="text" value="" />
        </label>
        <div />
        <label>
          Pool 3:
          <input type="text" value="tcp+stratum://btcfoundry.us:3333" />
        </label>
        <label>
          Worker:
          <input type="text" value="pc{'{'}model{'}'}." />
        </label>
        <label>
          Password:
          <input type="text" value="" />
        </label>
        <div />
        <button>Set All</button>
        <button>Set Selected</button>
      </div>
    </TabPanel>
    <TabPanel>
      <h2>Tab 3</h2>
      <p>Tab 3 content</p>
    </TabPanel>
  </Tabs>
</div>

<style>
  * :global(.dropdown) {
    width: 200px;
    height: 3em;
    margin-bottom: 5px;
  }

  .divider {
    border-left: 2px solid grey;
    height: 200px;
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
  }

  .col > * {
    margin-bottom: 5px;
  }

  .pool {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
  }

  button:disabled:active,
  button:disabled,
  button:disabled:hover {
    background-color: grey;
    color: darkgrey;
    cursor: not-allowed !important;
    pointer-events: none;
  }

  .control-grid {
    display: grid;
    grid-template-rows: auto auto auto;
  }

  .control-group {
    display: grid;
    margin-top: 5px;
    grid-template-columns: auto auto;
  }

  .control-group > * {
    margin-right: 5px;
  }
</style>
