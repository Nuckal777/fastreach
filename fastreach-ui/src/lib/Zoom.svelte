<script lang="ts">
    import { mapLocation } from "./store";

    function zoom(change: number) {
        mapLocation.update((loc) => ({
            lat: loc.lat,
            lng: loc.lng,
            zoom: loc.zoom + change,
        }));
    }
</script>

<div class="zoom-container">
    <button class="zoom-btn" on:click={() => zoom(1)}>+</button>
    <hr />
    <button class="zoom-btn" on:click={() => zoom(-1)}>−</button>
</div>

<style>
    .zoom-container {
        padding: 5px;
    }

    .zoom-btn {
        width: 30px;
        height: 30px;
        line-height: 30px;
        background-color: transparent;
        text-align: center;
        font-family: monospace;
        font-size: 22px;
        font-weight: bold;
        border: none;
    }

    .zoom-btn:hover {
        background-color: #f4f4f4;
    }
</style>
