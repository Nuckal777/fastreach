<script lang="ts">
    import "css.gg/icons/css/list.css";
    import "css.gg/icons/css/search.css";

    type Icon = "config" | "table" | "none";

    export let right = false;
    export let icon: Icon = "none";

    function toggle() {
        open = !open;
    }

    let open = true;
    $: text = open ? "-" : icon === "none" ? "+" : "";
</script>

<div class="toggle-container" class:end={right} class:toggle-center={!open}>
    <button
        class="small-btn"
        class:no-border={!open}
        class:border={open}
        class:gg-search={!open && icon === "config"}
        class:gg-list={!open && icon === "table"}
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
        min-width: 22px;
        min-height: 22px;
    }

    .gg-search {
        position: absolute;
        top: 10px;
    }

    .toggle-center {
        justify-content: center;
        align-content: center;
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
