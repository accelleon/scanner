<script lang="ts">
  import SortAscending from 'svelte-material-icons/SortAscending.svelte';
  import SortDescending from 'svelte-material-icons/SortDescending.svelte';

  type Column = {
    name: string;
    key: string;
    sortFn?: (a: any, b: any) => number;
    dispFn?: (a: any) => string;
    sortable?: boolean;
  };

  export let selection: any[] = undefined;
  export let columns: Column[] = undefined;
  export let data = [];
  export let unique = undefined;

  let sortBy = { key: undefined, order: undefined };
  let sorted = false;
  let display = data;

  function sortClick(key: string) {
    if (!columns.find((col) => col.key === key).sortable) return;
    if (sortBy.key === key) {
      sortBy.order = sortBy.order === "asc" ? "desc" : "asc";
    } else {
      sortBy.key = key;
      sortBy.order = "asc";
    }
    sorted = true;
  }

  $: {
    if (sorted) {
      if (!columns.find((col) => col.key === sortBy.key).sortFn) {
        display = data.sort((a, b) => {
          if (a[sortBy.key] < b[sortBy.key]) {
            return sortBy.order === "desc" ? -1 : 1;
          }
          if (a[sortBy.key] > b[sortBy.key]) {
            return sortBy.order === "desc" ? 1 : -1;
          }
          return 0;
        });
      } else {
        display = data.sort((a, b) => {
          if (
            columns
              .find((col) => col.key === sortBy.key)
              .sortFn(a[sortBy.key], b[sortBy.key]) > 0
          ) {
            return sortBy.order === "desc" ? -1 : 1;
          } else {
            return sortBy.order === "desc" ? 1 : -1;
          }
        });
      }
    } else {
      display = data;
    }
  }
</script>

<div class="container">
  <table>
    <thead>
      <tr>
        {#each columns as column}
          <th on:click={() => sortClick(column.key)}>
            {column.name}
            {#if sortBy.key === column.key}
              {#if sortBy.order === "asc"}
                <SortAscending />
              {:else}
                <SortDescending />
              {/if}
            {/if}
          </th>
        {/each}
      </tr>
    </thead>
    <tbody>
      {#if display}
        {#each display as row}
          <tr
            class={selection.findIndex((e) => e[unique] === row[unique]) !== -1
              ? "selected"
              : ""}
          >
            {#each columns as column}
              <td>
                {
                  @html row[column.key]
                  ? (column.dispFn
                    ? column.dispFn(row[column.key])
                    : row[column.key])
                  : ""
                }
              </td>
            {/each}
          </tr>
        {/each}
      {/if}
    </tbody>
  </table>
</div>

<style>
  table {
    border-collapse: collapse;
    width: 100%;
    border: 3px solid black;
  }
  th,
  td {
    text-align: left;
    padding: 1px;
    border: 1px solid #ddd;
  }
  th:hover {
    cursor: pointer;
    background-color: darkgrey;
  }
  tr {
    padding: 0;
  }
  tbody > tr:hover {
    background-color: darkgrey;
  }
  .selected {
    background-color: darkgrey;
  }
</style>
