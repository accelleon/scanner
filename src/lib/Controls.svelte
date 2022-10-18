<script lang="ts">
    import Dropdown from "./controls/Dropdown.svelte";
    import { Tabs, TabList, TabPanel, Tab } from "./controls/tabs/tabs.js";
    import { invoke } from '@tauri-apps/api/tauri';


    export let miners: any = undefined;

    async function scan() {
        return await invoke("scan_miners", {location: 'C20-1'});
    }

    async function scanMiners() {
        miners = await scan();
        console.log(miners);
        miners = miners;
    }

    let options = [
        {label: "Can 1", value: "option1"},
    ]
    let selected;

    $: console.log(selected);
</script>

<div class="container">
    <div class="col">
        <Dropdown bind:selected={selected} options={options} selObject={true} class="dropdown"/>
        <button on:click="{scanMiners}">Scan</button>
    </div>
    <div class="divider"/>
    <Tabs>
        <TabList>
            <Tab>Controls</Tab>
            <Tab>Pools</Tab>
            <Tab>Tab 3</Tab>
        </TabList>
        <TabPanel>
            <button>Start Locating</button>
            <button>Locate Selected</button>
        </TabPanel>
        <TabPanel>
            <div class="pool">
                <label>
                    Pool 1:
                    <input type="text" value="tcp+stratum://btcfoundry.us:3333"/>
                </label>
                <label>
                    Worker:
                    <input type="text" value="pc{'{'}model{'}'}."/>
                </label>
                <label>
                    Password:
                    <input type="text" value=""/>
                </label>
                <label>
                    IP Suffix:
                    <input type="checkbox"/>
                </label>
                <label>
                    Pool 2:
                    <input type="text" value="tcp+stratum://btcfoundry.us:3333"/>
                </label>
                <label>
                    Worker:
                    <input type="text" value="pc{'{'}model{'}'}."/>
                </label>
                <label>
                    Password:
                    <input type="text" value=""/>
                </label>
                <div/>
                <label>
                    Pool 3:
                    <input type="text" value="tcp+stratum://btcfoundry.us:3333"/>
                </label>
                <label>
                    Worker:
                    <input type="text" value="pc{'{'}model{'}'}."/>
                </label>
                <label>
                    Password:
                    <input type="text" value=""/>
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
    }

    .divider {
        border-left: 2px solid grey;
        height: 200px;
        margin: 0 20px;
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

    .pool {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr 1fr;
    }
</style>