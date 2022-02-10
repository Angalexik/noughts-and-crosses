<script lang="ts">
  import Switch from '@smui/switch';
  import FormField from '@smui/form-field';
  import Paper from '@smui/paper';

  import SliderWithCentredLabel from './SliderWithCentredLabel.svelte';

  export let playingAsX = true;
  export let boardWidth = 3;
  export let boardHeight = 3;
  export let boardRow = 3;

  let sliderProps = {
    min: 2,
    max: 6,
    discrete: true,
  };
</script>

<Paper class="centred-container absolute bottom-4 max-w-2xl">
  <FormField align="start" style="width: 210px">
    <Switch bind:checked={playingAsX} />
    <span slot="label">{playingAsX ? 'Playing as X' : 'Playing as O'}</span>
  </FormField>
  <SliderWithCentredLabel bind:value={boardHeight} {...sliderProps}>
    <span>Board Height</span>
  </SliderWithCentredLabel>
  <SliderWithCentredLabel bind:value={boardWidth} {...sliderProps}>
    <span>Board Width</span>
  </SliderWithCentredLabel>
  <SliderWithCentredLabel
    bind:value={boardRow}
    min={2}
    max={Math.min(boardHeight, boardWidth)}
    discrete
  >
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
