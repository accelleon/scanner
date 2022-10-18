<script lang="ts">
    import Dropdown from "./controls/Dropdown.svelte";
    import { Tabs, TabList, TabPanel, Tab } from "./controls/tabs/tabs.js";

    let options = [
        {label: "Can 1", value: "option1"},
    ]
    let selected;

    $: console.log(selected);
</script>

<div class="container">
    <div class="col">
        <Dropdown bind:selected={selected} options={options} selObject={true} class="dropdown"/>
        <button>Scan</button>
    </div>
    <div class="divider"/>
    <Tabs>
        <TabList>
            <Tab>Pools</Tab>
            <Tab>Controls</Tab>
            <Tab>Tab 3</Tab>
        </TabList>
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
            <h2>Tab 2</h2>
            <p>Tab 2 content</p>
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
        border-left: 2px solid black;
        height: 95%;
        margin: 0 20px;
    }

    .container {
        display: flex;
        flex-direction: row;
        align-items: left;
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