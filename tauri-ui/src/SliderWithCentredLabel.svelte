<script lang="ts">
    import FormField from "@smui/form-field";
    import Slider from "@smui/slider";

    export let value;
    export let min: number;
    export let max: number;
    export let discrete: boolean;

    $: {
        value = Math.max(Math.min(max, value), min)
    }
</script>

<FormField style="display: inline-block; text-align: center;">
    <slot></slot>
    <Slider
        bind:value
        {min}
        {max}
        {discrete}
        style="width: 10vw;"
    />
</FormField>