<script lang="ts">
  export let rows = 3;
  export let columns = 3;
  export let inarow = 3;
  export let style = "";
  export let started = false;

  let board;
  let displayBoard = new Array(columns).fill("").map(() => new Array(rows).fill(""));

  $: if (board === undefined) {
    displayBoard = new Array(columns).fill("").map(() => new Array(rows).fill(""));
  }

  function place(idx: number) {
    // console.log(idx)
    // board[idx] = xTurn ? "close" : "circle"
    // xTurn = !xTurn
  }
</script>

<div id="boardContainer">
  <div id="board" style="--rows: {rows}; --columns: {columns}; {style}">
    {#each displayBoard as row, i}
      {#each row as col, i}
        <div
          class="tile centred-container"
          on:click={() => {
            if (started) place(i);
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
    background: white;
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
    content: "";
    display: block;
    height: 0;
    width: 0;
    padding-bottom: calc(100%);
  }
</style>
