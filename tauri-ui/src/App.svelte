<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  import Paper from '@smui/paper';
  import Button, { Label } from '@smui/button';

  import Board from './Board.svelte';
  import BoardSettings from './BoardSettings.svelte';

  let board: Board;
  let boardString: string;

  let gameStarted: boolean;
  let playingAsX = true;
  let boardWidth = 3;
  let boardHeight = 3;
  let boardRow = 3;
  let waiting = false;

  async function toggleGameState() {
    gameStarted = !gameStarted;
    if (gameStarted) {
      await invoke('reset', {
        width: boardWidth,
        height: boardHeight,
        row: boardRow,
        kind: 'XOBoard',
      });
      boardString = await invoke('board');
      if (!playingAsX) {
        await cpuPlay();
        boardString = await invoke('board');
      }
    }
  }

  async function place(event: CustomEvent<{ row: number; col: number }>) {
    if (await invoke('can_play', event.detail)) {
      await invoke('place', event.detail);
      boardString = await invoke('board');
      if (await invoke('over')) {
        toggleGameState();
        return;
      }
      await cpuPlay();
      if (await invoke('over')) toggleGameState();
    }
  }

  async function cpuPlay() {
    waiting = true;
    await invoke('place_best_move');
    boardString = await invoke('board');
    waiting = false;
  }
</script>

<main class="m-auto max-w-2xl">
  <Paper
    style="background-color: {waiting ? '#aaa' : '#fff'};"
    class="relative transition-colors"
  >
    <Board
      bind:this={board}
      bind:started={gameStarted}
      {boardString}
      columns={boardWidth}
      rows={boardHeight}
      {waiting}
      on:place={place}
    />
    <div class="absolute bottom-6 left-0 right-0">
      <!-- Why can't you just act like a normal browser safari -->
      <Button
        class="mx-auto mt-2 mb-0 block"
        variant="raised"
        on:click={toggleGameState}
      >
        <Label>{gameStarted ? 'Stop Game' : 'Start Game'}</Label>
      </Button>
    </div>
  </Paper>

  {#if !gameStarted}
    <BoardSettings
      bind:playingAsX
      bind:boardWidth
      bind:boardHeight
      bind:boardRow
    />
  {/if}

  <!-- <Button on:click={() => {console.log(board)}}><Label>Press me 😏</Label></Button> -->
</main>

<svelte:head>
  <link rel="stylesheet" href="https://unpkg.com/svelte-material-ui/bare.css" />
  <!-- Roboto -->
  <link
    rel="stylesheet"
    href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,600,700"
  />
  <!-- Roboto Mono -->
  <link
    rel="stylesheet"
    href="https://fonts.googleapis.com/css?family=Roboto+Mono"
  />
  <!-- Material Icons -->
  <link
    rel="stylesheet"
    href="https://fonts.googleapis.com/icon?family=Material+Icons+Outlined"
  />
</svelte:head>

<style global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
