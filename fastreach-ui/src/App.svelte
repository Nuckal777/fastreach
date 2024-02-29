<script lang="ts">
    import "purecss/build/pure-min.css";
    import "purecss/build/grids-responsive-min.css";
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
    <div class="overlay pure-g">
        <div class="pure-u-1">
            <div class="wrapper pure-g">
                <div class="pure-u-1 pure-u-md-1-2 pure-u-xl-1-3">
                    <div class="map-overlay">
                        <Toggle>
                            <h2>Fastreach</h2>
                            <button
                                class="small-btn border float-right"
                                on:click={() => (infoOpen = true)}>i</button
                            >
                            <IsochroneConfig useIsochrone={addIsochrone} />
                        </Toggle>
                    </div>
                </div>
                <div class="pure-u-1 pure-u-md-1-2 pure-u-xl-2-3">
                    <div class="map-overlay float-right">
                        <Toggle right>
                            <h2>Isochrones</h2>
                            <IsochroneTable
                                {isochrones}
                                onRemove={removeIsochrone}
                            />
                        </Toggle>
                    </div>
                </div>
            </div>
        </div>
        <div class="pure-u-1">
            <div class="map-overlay">
                <Zoom></Zoom>
            </div>
        </div>
    </div>
    <div class="map">
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

        flex-direction: column;
        justify-content: space-between;
    }

    .map-overlay {
        background-color: rgba(255, 255, 255, 0.85);
        backdrop-filter: blur(3px);
        border-radius: 5px;
        margin: 10px;
        padding: 5px;
        min-width: calc(1em + 8px);
        width: fit-content;
        max-width: calc(100% - 34px);
        border: 2px solid rgba(0, 0, 0, 0.2);
        pointer-events: auto;
        max-height: 70vh;
        overflow: auto;
    }

    @media (width <= 768px) {
        .map-overlay {
            max-height: 38vh;
        }
    }

    @media (height <= 512px) and (width > 768px) {
        .map-overlay {
            max-height: 60vh;
        }
    }

    .map {
        pointer-events: initial;
        position: absolute;
        left: 0px;
        top: 0px;
        z-index: 0;
        height: 100%;
        width: 100%;
    }

    .wrapper {
        flex-direction: row;
        justify-content: space-between;
        align-content: space-between;
        width: 100%;
    }
</style>
