<script lang="ts">
    import IsochroneForm from "./lib/IsochroneForm.svelte";
    import Map from "./lib/Map.svelte";
    import type { IsochroneResponse } from "./lib/types";

    let isochrones: IsochroneResponse[] = [];

    function addIsochrone(iso: IsochroneResponse) {
        isochrones = [...isochrones, iso];
    }
</script>

<main class="wrapper">
    <div class="header">
        <h1>Fastreach</h1>
        <IsochroneForm useIsochrone={addIsochrone} />
    </div>
</main>
<div class="map">
    <Map geometries={isochrones.map((val) => val.geometry)} />
</div>

<style>
    .wrapper {
        position: relative;
        z-index: 1;
    }

    .header {
        background-color: white;
        border-radius: 20px;
        margin: 5px;
        width: fit-content;
    }

    .map {
        position: absolute;
        left: 0px;
        top: 0px;
        z-index: 0;
        height: 100%;
        width: 100%;
    }
</style>
