<script lang="ts">
    export let right = false;

    function toggle() {
        open = !open;
    }

    let open = true;
    $: text = open ? "-" : "+";
</script>

<button
    class="small-btn"
    class:no-border={!open}
    class:border={open}
    class:float-right={right}
    on:click={toggle}>{text}</button
>
<div class:hidden={!open}>
    <slot />
</div>

<style>
    .small-btn {
        width: 1.5em;
        height: 1.5em;
        max-height: 1.5em;
        line-height: 1.5em;
        background-color: transparent;
        text-align: center;
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
    .float-right {
        float: right;
    }
</style>
