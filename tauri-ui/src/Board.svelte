<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import CircularProgress from '@smui/circular-progress';

  const dispatch =
    createEventDispatcher<{ place: { row: number; col: number } }>();

  export let rows = 3;
  export let columns = 3;
  export let boardString: string;
  export let waiting: boolean;

  export let started = false;

  let board: string[][] = getBoard(columns, rows, boardString);

  function getBoard(
    width: number,
    height: number,
    boardString: string
  ): string[][] {
    if (started) {
      return boardString.split('\n').map((line) => line.split(''));
    } else {
      return new Array(width).fill('').map(() => new Array(height).fill('.'));
    }
  }

  $: board = getBoard(columns, rows, boardString);
</script>

<div id="boardContainer">
  {#if waiting}
    <CircularProgress
      style="width: 160px; height: 160px; position: absolute; inset: 0; margin: auto;"
      indeterminate
    />
  {/if}
  <div
    id="board"
    style="--rows: {rows}; --columns: {columns};"
    class="mt-0 mb-16 mx-auto"
  >
    {#each board as row, i}
      {#each row as col, j}
        <div
          class="tile centred-container transition-colors"
          style="background-color: {waiting ? '#aaa' : '#fff'};"
          on:click={() => {
            if (started && !waiting) {
              dispatch('place', {
                row: i,
                col: j,
              });
            }
          }}
        >
          <img class="icon" src="./{col}.svg" alt=" " />
        </div>
      {/each}
    {/each}
  </div>
</div>

<style>
  #board {
    display: grid;
    grid-template-columns: repeat(var(--columns), minmax(0, 1fr));
    grid-template-rows: repeat(var(--rows), minmax(0, 1fr));
    gap: 4px;
    background: black;
    aspect-ratio: calc(var(--columns) / var(--rows));
    max-width: 50vmin;
    max-height: 50vmin;
  }

  .tile {
    min-height: 0;
    /* font-size: calc(50vmin / (var(--rows))); */
    text-align: center;
    aspect-ratio: 1/1;
    max-height: 100%;
    max-width: 100%;
    /* width: calc(50vmin / min(var(--rows), var(--columns)));
        height: calc(50vmin / min(var(--rows), var(--columns))); */
  }

  .icon {
    width: 70%;
    height: 70%;
  }

  .icon:before {
    content: '';
    display: block;
    height: 0;
    width: 0;
    padding-bottom: calc(100%);
  }
</style>
