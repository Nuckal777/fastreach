<script lang="ts">
    import IsochroneRow from "./IsochroneRow.svelte";
    import type { IsochroneCall } from "./types";

    export let isochrones: IsochroneCall[] = [];
</script>

<table class="pure-table pure-table-bordered">
    <thead>
        <tr>
            <th>From</th>
            <th>Date</th>
            <th>Duration</th>
            <th>Area</th>
            <th>Diameter</th>
        </tr>
    </thead>
    <tbody>
        {#each isochrones as iso}
            <IsochroneRow isochroneCall={iso} />
        {/each}
    </tbody>
</table>

<style>
    th {
        background-color: #0078e7;
        color: white;
    }
</style>
