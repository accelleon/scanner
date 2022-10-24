<script lang="ts">
  import { getContext } from "svelte";
  import { TABS } from "./Tabs.svelte";

  const panel = {};
  const { registerPanel, selectedPanel } = getContext<{
    registerPanel: any;
    selectedPanel: any;
  }>(TABS);

  registerPanel(panel);
</script>

{#if $selectedPanel === panel}
  <slot />
{/if}
