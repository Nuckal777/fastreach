<script lang="ts">
    import "purecss/build/pure-min.css";
    import { onMount } from "svelte";
    import IsochroneConfig from "./lib/IsochroneConfig.svelte";
    import IsochroneTable from "./lib/IsochroneTable.svelte";
    import Map from "./lib/Map.svelte";
    import Toggle from "./lib/Toggle.svelte";
    import { nodes } from "./lib/store";
    import type { IsochroneCall, NodeResponse } from "./lib/types";
    import Info from "./lib/Info.svelte";
    import Zoom from "./lib/Zoom.svelte";

    let infoOpen = true;
    let isochrones: IsochroneCall[] = [];

    function addIsochrone(iso: IsochroneCall) {
        isochrones = [...isochrones, iso];
    }

    function removeIsochrone(index: number) {
        isochrones = isochrones.toSpliced(index, 1);
    }

    async function fetchNodes() {
        const res = await fetch("/nodes-v1.json");
        if (!res.ok) {
            nodes.set({
                response: [],
                error: `HTTP request failed, status code: ${res.status}`,
            });
            return;
        }
        const nodeRes = (await res.json()) as NodeResponse;
        nodes.set({ response: nodeRes, error: "" });
    }

    onMount(fetchNodes);
</script>

<main>
    {#if infoOpen}
        <Info onClose={() => (infoOpen = false)}></Info>
    {/if}
    <div class="overlay">
        <div class="config-container">
            <div class="config-item">
                <div class="map-overlay border">
                    <Toggle icon="config">
                        <h2>Fastreach</h2>
                        <button
                            class="small-btn border"
                            on:click={() => (infoOpen = true)}>i</button
                        >
                        <IsochroneConfig useIsochrone={addIsochrone} />
                    </Toggle>
                </div>
            </div>
            <div class="config-item flex-container justify-end">
                <div class="map-overlay border">
                    <Toggle right icon="table">
                        <h2>Isochrones</h2>
                        <IsochroneTable
                            {isochrones}
                            onRemove={removeIsochrone}
                        />
                    </Toggle>
                </div>
            </div>
        </div>
        <div style="height: fit-content;">
            <div class="map-overlay border">
                <Zoom></Zoom>
            </div>
        </div>
    </div>
    <div class="map fill">
        <Map geometries={isochrones.map((val) => val.response.geometry)} />
    </div>
</main>

<style>
    main {
        position: absolute;
        pointer-events: none;
        left: 0px;
        right: 0px;
        height: 100%;
    }

    .overlay {
        position: absolute;
        top: 0px;
        left: 0px;
        right: 0px;
        pointer-events: none;
        height: 100%;
        z-index: 1;

        display: flex;
        flex-direction: column;
        justify-content: space-between;
        overflow: hidden;
        row-gap: 10px;
    }

    .map-overlay {
        min-height: 0;
        background-color: rgba(255, 255, 255, 0.85);
        backdrop-filter: blur(3px);
        margin: 10px;
        min-width: calc(1em + 8px);
        width: fit-content;
        height: fit-content;
        max-width: calc(100% - 34px);
        pointer-events: auto;
        max-height: 100%;
        overflow: auto;
    }

    .map {
        pointer-events: initial;
        position: absolute;
        left: 0px;
        top: 0px;
        z-index: 0;
    }

    .config-container {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        width: 100%;

        min-height: 0;
        flex-basis: content;
    }

    .config-item {
        flex-basis: auto;
        width: 50%;
        height: 100%;
    }

    .flex-container {
        display: flex;
    }

    .justify-end {
        justify-content: end;
    }

    @media (width <= 768px) {
        .config-container {
            flex-direction: column;
            justify-content: start;
            row-gap: 10px;
        }

        .config-item {
            width: 100%;
            flex-basis: content;
            min-height: 2.5em;
        }

        .justify-end {
            justify-content: start;
        }
    }
</style>
