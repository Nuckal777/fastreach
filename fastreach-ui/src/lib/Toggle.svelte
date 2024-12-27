<script lang="ts">
    import list from "css.gg/icons/svg/list.svg";
    import search from "css.gg/icons/svg/search.svg";

    type Icon = "config" | "table" | "none";

    interface Props {
        right?: boolean;
        icon?: Icon;
        children?: import('svelte').Snippet;
    }

    let { right = false, icon = "none", children }: Props = $props();

    function toggle() {
        open = !open;
    }

    let open = $state(true);
    let text = $derived(open ? "-" : icon === "none" ? "+" : "");
</script>

<div class="toggle-container" class:end={right} class:toggle-center={!open}>
    {#if open}
        <input
            type="button"
            class="small-btn border"
            onclick={toggle}
            value={text}
        />
    {:else}
        {#if icon === "config"}
            <input
                type="image"
                class="no-border"
                src={search}
                alt="isochrone configuration"
                onclick={toggle}
            />
        {/if}
        {#if icon === "table"}
            <input
                type="image"
                class="no-border"
                src={list}
                alt="isochrone data"
                onclick={toggle}
            />
        {/if}
    {/if}
</div>
<div class:hidden={!open} class="content">
    {@render children?.()}
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
