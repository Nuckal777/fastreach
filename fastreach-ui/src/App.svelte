<script lang="ts">
    import IsochroneConfig from "./lib/IsochroneConfig.svelte";
    import IsochroneTable from "./lib/IsochroneTable.svelte";
    import Map from "./lib/Map.svelte";
    import Toggle from "./lib/Toggle.svelte";
    import type { IsochroneCall } from "./lib/types";

    let isochrones: IsochroneCall[] = [];

    function addIsochrone(iso: IsochroneCall) {
        isochrones = [...isochrones, iso];
    }
</script>

<main class="main">
    <div class="overlay wrapper">
        <div class="map-overlay">
            <Toggle>
                <h2>Fastreach</h2>
                <IsochroneConfig useIsochrone={addIsochrone} />
            </Toggle>
        </div>
        <div class="map-overlay">
            <Toggle>
                <h2>Isochrones</h2>
                <IsochroneTable {isochrones} />
            </Toggle>
        </div>
    </div>
</main>
<div class="map">
    <Map geometries={isochrones.map((val) => val.response.geometry)} />
</div>

<style>
    @import "https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/pure-min.css";

    .main {
        position: absolute;
        left: 0px;
        right: 0px;
        z-index: 1;
    }

    .overlay {
        position: absolute;
        left: 0px;
        right: 0px;
        pointer-events: none;
    }

    .map-overlay {
        background-color: rgba(255, 255, 255, 0.85);
        backdrop-filter: blur(3px);
        border-radius: 5px;
        margin: 10px;
        padding: 5px;
        width: fit-content;
        height: fit-content;
        border: 2px solid rgba(0, 0, 0, 0.2);
        pointer-events: auto;
        max-height: calc(80vh - 105px);
        overflow: scroll;
    }

    .map {
        position: absolute;
        left: 0px;
        top: 0px;
        z-index: 0;
        height: 100%;
        width: 100%;
    }

    .wrapper {
        display: flex;
        justify-content: space-between;
    }
</style>
