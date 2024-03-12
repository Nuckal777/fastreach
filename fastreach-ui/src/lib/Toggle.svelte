<script lang="ts">
    import list from "css.gg/icons/svg/list.svg";
    import search from "css.gg/icons/svg/search.svg";

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
    {#if open}
        <input
            type="button"
            class="small-btn border"
            on:click={toggle}
            value={text}
        />
    {:else}
        {#if icon === "config"}
            <input
                type="image"
                class="no-border"
                src={search}
                alt="isochrone configuration"
                on:click={toggle}
            />
        {/if}
        {#if icon === "table"}
            <input
                type="image"
                class="no-border"
                src={list}
                alt="isochrone data"
                on:click={toggle}
            />
        {/if}
    {/if}
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

    .toggle-center {
        justify-content: center;
        align-content: center;
    }

    .end {
        justify-content: end;
    }

    @media (width <= 640px) {
        .end {
            justify-content: start;
        }
    }

    .content {
        overflow: auto;
        padding: 5px;
    }
</style>
