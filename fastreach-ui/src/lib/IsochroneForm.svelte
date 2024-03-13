<script lang="ts">
    import type { Node, IsochroneConfiguration } from "./types";

    enum FilterState {
        Empty,
        Match,
        Ambiguous,
    }

    const minMinutes = 5;
    const maxMinutes = 120;

    export let useNodes: (nodes: IsochroneConfiguration) => void;
    export let station = "";
    export let minutes = 0;
    export let start = "";
    export let jump = true;

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

    const minutesExceeded = (m: number) => m < minMinutes || m > maxMinutes;

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
                min={minMinutes}
                max={maxMinutes}
                step="1"
                bind:value={minutes}
            />
        </div>
        {#if minutesExceeded(minutes)}
            <p>Minutes must be between {minMinutes} and {maxMinutes}</p>
        {/if}
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
        <div class="pure-control-group">
            <label for="jump">Jump to station:</label>
            <input type="checkbox" name="jump" id="jump" bind:checked={jump} />
        </div>
        <div class="pure-controls">
            <input
                type="button"
                class="pure-button pure-button-primary"
                value={filterState === FilterState.Ambiguous
                    ? "Search stations"
                    : "Calculate isochrone"}
                disabled={filterState === FilterState.Empty ||
                    minutesExceeded(minutes)}
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
