<script lang="ts">
  import Switch from "@smui/switch";
  import FormField from "@smui/form-field";
  import Paper from "@smui/paper";

  import SliderWithCentredLabel from "./SliderWithCentredLabel.svelte";

  export let connect4 = false;
  export let boardWidth = 3;
  export let boardHeight = 3;
  export let boardRow = 3;

  export let style = "";

  let sliderProps = {
    min: 2,
    max: 6,
    discrete: true,
  };
</script>

<Paper class="centred-container" {style}>
  <FormField align="start" style="width: 210px">
    <Switch bind:checked={connect4} />
    <span slot="label"
      >{connect4 ? "Connect 4 Mode" : "Noughts and Crosses"}</span
    >
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
