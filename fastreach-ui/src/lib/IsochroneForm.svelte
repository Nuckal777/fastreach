<script lang="ts">
    import type { Node, IsochroneConfiguration } from "./types";

    enum FilterState {
        Empty,
        Match,
        Ambiguous,
    }

    export let useNodes: (nodes: IsochroneConfiguration) => void;
    export let station = "";
    export let minutes = 0;
    export let start = "";

    export let nodes: Node[];
    let matchingNodes: Node[] = [];
    let filterState = FilterState.Empty;
    let filterText: string = "";

    function filterNodes(name: string) {
        matchingNodes = nodes.filter((node) =>
            node.name.toLowerCase().includes(name.toLowerCase()),
        );
        if (matchingNodes.length === 0) {
            filterState = FilterState.Empty;
            filterText = `No station name contains ${name}`;
            return;
        }
        if (matchingNodes.length === 1) {
            filterState = FilterState.Match;
            const precision = 4;
            const matching = matchingNodes[0];
            filterText = `${matching.name}\nLon: ${matching.coords[0].toFixed(
                precision,
            )} Lat: ${matching.coords[1].toFixed(precision)}`;
            return;
        }
        filterState = FilterState.Ambiguous;
        filterText = `Ambiguous: ${matchingNodes.length} stations match`;
    }

    function createConfiguration() {
        useNodes({
            minutes: minutes,
            nodes: matchingNodes,
            start: new Date(start),
        });
    }

    $: filterNodes(station);
</script>

<form class="pure-form pure-form-aligned">
    <legend>Isochrone configuration</legend>
    <fieldset>
        <div class="pure-control-group">
            <label for="station">Station: </label>
            <input
                type="text"
                class="fill-width"
                name="station"
                id="station"
                bind:value={station}
            />
        </div>
        <p>{filterText}</p>
        <div class="pure-control-group">
            <label for="minutes">Minutes of travel:</label>
            <input
                type="number"
                class="fill-width"
                name="minutes"
                id="minutes"
                min="5"
                max="120"
                step="5"
                bind:value={minutes}
            />
        </div>
        <div class="pure-control-group">
            <label for="start">Start date: </label>
            <input
                type="datetime-local"
                class="fill-width"
                name="start"
                id="start"
                bind:value={start}
            />
        </div>
        <div class="pure-controls">
            <input
                type="button"
                class="pure-button pure-button-primary"
                value={filterState === FilterState.Ambiguous
                    ? "Specify station"
                    : "Calculate isochrone"}
                disabled={filterState === FilterState.Empty}
                on:click={createConfiguration}
            />
        </div>
    </fieldset>
</form>

<style>
    .fill-width {
        width: 13em;
    }
</style>
