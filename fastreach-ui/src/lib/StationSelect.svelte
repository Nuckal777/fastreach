<script lang="ts">
    import type { Node, IsochroneConfiguration } from "./types";

    interface Props {
        config: IsochroneConfiguration;
        useConfiguration: (config: IsochroneConfiguration) => void;
    }

    let { config, useConfiguration }: Props = $props();
    let stations: Node[] = $state([]);
    $effect(() => {
        stations = config.nodes;
    });

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
        onclick={() => clickStation(station)}
    />
    <p>
        Lon: {station.coords[0].toFixed(4)} Lat: {station.coords[1].toFixed(4)}
    </p>
    <hr />
{/each}
