<script lang="ts">
    import type { Node, IsochroneConfiguration } from "./types";

    export let config: IsochroneConfiguration;
    export let useConfiguration: (config: IsochroneConfiguration) => void;
    let stations: Node[] = [];
    $: stations = config.nodes;

    function clickStation(node: Node) {
        useConfiguration({
            minutes: config.minutes,
            nodes: [node],
            start: config.start,
        });
    }
</script>

<p>Click a station to calculate isochrone.</p>
<hr />
{#each stations as station}
    <input
        type="button"
        value={station.name}
        class="pure-button"
        on:click={() => clickStation(station)}
    />
    <p>
        Lon: {station.coords[0].toFixed(4)} Lat: {station.coords[1].toFixed(4)}
    </p>
    <hr />
{/each}
