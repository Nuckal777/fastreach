<script lang="ts">
    export let onBack: () => void = () => {};
</script>

<input
    class="pure-button font-small"
    type="button"
    value="Back"
    on:click={() => onBack()}
/>
<slot />

<style>
    .font-small {
        font-size: 85%;
    }
</style>
