<script lang="ts">
	import Paper from '@smui/paper'
	import Button, { Label } from '@smui/button'

	import Board from "./Board.svelte";
	import BoardSettings from './BoardSettings.svelte'

	import { invoke } from "@tauri-apps/api/tauri";

	let connect4 = false;
	let boardWidth = 3;
	let boardHeight = 3;
	let boardRow = 3;

	let gameStarted = false;
</script>

<main>
	<Paper style="height: 60vh; position: relative">
		<Board started={gameStarted} columns={boardWidth} rows={boardHeight} inarow={boardRow} style="margin: 0 auto;" />
		<div style="position: absolute; bottom: 24px; left: 0px; right: 0px"> <!-- Why can't you just act like a normal browser safari -->
			<Button style="margin: 8px auto 0; display: block; " variant="raised" on:click={() => gameStarted = !gameStarted}>
				<Label>{gameStarted ? "Stop Game" : "Start Game"}</Label>
			</Button>
		</div>
	</Paper>

	{#if !gameStarted}
		<BoardSettings bind:connect4 bind:boardWidth bind:boardHeight bind:boardRow style="margin-top: 16px;"/>
	{/if}

	<Button on:click={() => {invoke("sanity_check").catch(error => {debugger;})}}><Label>Press me 😏</Label></Button>
</main>

<style>
	main {
		margin: auto;
		max-width: 60%;
	}

</style>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://unpkg.com/svelte-material-ui/bare.css"
	/>
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