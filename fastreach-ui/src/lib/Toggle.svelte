<script lang="ts">
    export let right = false;

    function toggle() {
        open = !open;
    }

    let open = true;
    $: text = open ? "-" : "+";
</script>

<div class="toggle-container" class:end={right}>
    <button
        class="small-btn"
        class:no-border={!open}
        class:border={open}
        on:click={toggle}>{text}</button
    >
</div>
<div class:hidden={!open} class="content">
    <slot />
</div>

<style>
    .toggle-container {
        display: flex;
        padding: 5px;
    }

    .end {
        justify-content: end;
    }

    @media (width <= 768px) {
        .end {
            justify-content: start;
        }
    }

    .content {
        overflow: auto;
        padding: 5px;
    }
</style>
