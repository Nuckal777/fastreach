<script lang="ts">
    interface Props {
        onBack?: () => void;
        children?: import('svelte').Snippet;
    }

    let { onBack = () => {}, children }: Props = $props();
</script>

<input
    class="pure-button font-small"
    type="button"
    value="Back"
    onclick={() => onBack()}
/>
{@render children?.()}

<style>
    .font-small {
        font-size: 85%;
    }
</style>
