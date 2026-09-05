<script lang="ts">
    interface Props {
        onBack?: () => void;
        children?: import('svelte').Snippet;
    }

    let { onBack = () => {} }: Props = $props();
</script>

<input
    class="pure-button font-small"
    type="button"
    value="Back"
    onclick={() => onBack()}
/>

<style>
    .font-small {
        font-size: 85%;
    }
</style>
