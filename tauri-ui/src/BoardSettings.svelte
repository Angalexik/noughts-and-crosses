<script lang="ts">
	import Checkbox from '@smui/checkbox'
	import FormField from '@smui/form-field'
	import Paper from '@smui/paper'

	import SliderWithCentredLabel from './SliderWithCentredLabel.svelte';

	export let connect4 = false;
	export let boardWidth = 3;
	export let boardHeight = 3;
	export let boardRow = 3;

	export let style = '';

	let sliderProps = {
		min: 2,
		max: 10,
		discrete: true,
	}
</script>

<Paper class="centred-container" {style}>
    <FormField align="start">
        <Checkbox bind:checked={connect4} />
        <span slot="label">Connect 4 Mode</span>
    </FormField>
    <SliderWithCentredLabel bind:value={boardHeight} {...sliderProps}>
        <span>Board Height</span>
    </SliderWithCentredLabel>
    <SliderWithCentredLabel bind:value={boardWidth} {...sliderProps}>
        <span>Board Width</span>
    </SliderWithCentredLabel>
    <SliderWithCentredLabel bind:value={boardRow} min={2} max={Math.min(boardHeight, boardWidth)} discrete>
        <span>How Many In A Row</span>
    </SliderWithCentredLabel>
</Paper>

<style>
	:global(.centred-container) {
		display: flex;

		justify-content: center;
		align-items: center;
	}
</style>