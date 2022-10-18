<script lang="ts">
    import Rack from './Rack.svelte';
    import type { Rack as TRack} from "../types";

    let rack: TRack = {
        id: 0,
        name: "Rack 1",
        width: 4,
        miners: [
            [
                {ip: "1", make: "Antminer", model: "S9", hashrate: 0},
                {ip: "2", make: "Antminer", model: "S9", hashrate: 0},
                {ip: "3", make: "Antminer", model: "S9", hashrate: 0},
                {ip: "4", make: "Antminer", model: "S9", hashrate: 45},
            ],
            [
                {ip: "5"},
                {ip: "6"},
                {ip: "7", make: "Antminer", model: "S9", hashrate: 45},
                {ip: "8", make: "Antminer", model: "S9", hashrate: 45},
            ],
            [
                {ip: "9", make: "Antminer", model: "S9", hashrate: 45},
                {ip: "10", make: "Antminer", model: "S9", hashrate: 45},
                {ip: "11", make: "Antminer", model: "S9", hashrate: 0},
                {ip: "12"},
            ]
        ]
    };

    let racks = Array.from({length: 11}, i => rack);
</script>

<div class="grid" style="--nracks: {racks.length}">
    {#each racks as rack}
        <Rack {rack} />
    {/each}
</div>

<style>
    .grid {
        display: grid;
        grid-template-columns: repeat(var(--nracks), 1fr);
        grid-gap: 10px;
        background-color: #2196F3;
        padding: 10px;
    }
</style>