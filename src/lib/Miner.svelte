<script lang="ts">
  import type { Miner } from "../types";
  export let miner: Miner = undefined;

  let color = "gray";
  if (miner.make) {
    color = miner.hashrate > 0 ? "green" : "red";
  }
</script>

<div class="miner" style="--bgcolor: {color}">
</div>

<style>
    .miner {
        background-color: var(--bgcolor);
        padding: 10px;
        font-size: 1.2rem;
        text-align: center;
        border: 1px solid black;
    }
</style>