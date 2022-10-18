<script lang="ts">
    import Rack from './Rack.svelte';
    import type { Rack as TRack} from "../types";
    export let miners: TRack[] = undefined;
    export let selection: any[] = undefined;
</script>

<div class="grid" style="--nracks: {miners.length}">
    {#each miners as rack}
        <Rack bind:selection = {selection} {rack} />
    {/each}
</div>

<style>
    .grid {
        display: grid;
        grid-template-columns: repeat(var(--nracks), 1fr);
        grid-gap: 10px;
        padding: 10px;
        margin: 0;
        height: 10vh;
    }
</style>