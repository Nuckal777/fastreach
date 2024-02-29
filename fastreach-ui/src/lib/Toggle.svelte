<script lang="ts">
    export let right = false;

    function toggle() {
        open = !open;
    }

    let open = true;
    $: text = open ? "-" : "+";
</script>

<div>
    <button
        class="small-btn"
        class:no-border={!open}
        class:border={open}
        class:float-right={right}
        on:click={toggle}>{text}</button
    >
</div>
<div class:hidden={!open} class="content">
    <slot />
</div>

<style>
    .content {
        overflow: auto;
        width: 100%;
    }
</style>
