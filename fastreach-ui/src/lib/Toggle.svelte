<script lang="ts">
    function toggle() {
        open = !open;
    }

    let open = true;
    $: text = open ? "<" : ">";
</script>

{#if open}
    <button class="small-btn border" on:click={toggle}>{text}</button>
    <slot />
{:else}
    <button class="small-btn no-border" on:click={toggle}>{text}</button>
{/if}

<style>
    .small-btn {
        width: 1.5em;
        height: 1.5em;
        max-height: 1.5em;
        line-height: 1.5em;
        background-color: transparent;
        text-align: center;
        display: inline-block;
        vertical-align: middle;
    }
    .small-btn:hover {
        background-color: #f4f4f4;
    }
    .border {
        border: 2px solid rgba(0, 0, 0, 0.2);
        border-radius: 5px;
    }
    .no-border {
        border: 2px;
    }
</style>
