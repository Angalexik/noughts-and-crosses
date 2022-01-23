<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  export let rows = 3;
  export let columns = 3;
  export let inarow = 3;
  export let style = "";

  export let started = false;

  let board: string[][] | undefined;

  async function place(row: number, col: number) {
    if (await invoke("can_play", { row: row, col: col })) {
      invoke("place", {
        row: row,
        col: col,
      });
      getBoard(columns, rows).then((x) => (board = x));
    }
    // console.log(idx)
    // board[idx] = xTurn ? "close" : "circle"
    // xTurn = !xTurn
  }

  export async function toggleGameState() {
    // TODO: shorter name
    invoke("reset", {
      width: columns,
      height: rows,
      row: inarow,
      kind: "XOBoard",
    });
    started = !started;
  }

  async function getBoard(width: number, height: number): Promise<string[][]> {
    if (started) {
      const boardString: string = await invoke("board");
      return boardString.split("\n").map((line) => line.split(""));
    } else {
      return new Array(width).fill("").map(() => new Array(height).fill("."));
    }
  }

  $: getBoard(columns, rows).then((x) => (board = x));
</script>

<div id="boardContainer">
  <div id="board" style="--rows: {rows}; --columns: {columns}; {style}">
    {#if board !== undefined}
      {#each board as row, i}
        {#each row as col, j}
          <div
            class="tile centred-container"
            on:click={() => {
              if (started) place(i, j);
            }}
          >
            <img class="icon" src="./{col}.svg" alt=" " />
          </div>
        {/each}
      {/each}
    {/if}
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
