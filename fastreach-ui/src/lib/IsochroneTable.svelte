<script lang="ts">
    import IsochroneRow from "./IsochroneRow.svelte";
    import type { IsochroneCall } from "./types";

    interface Props {
        isochrones?: IsochroneCall[];
        onRemove?: (index: number) => void;
    }

    let { isochrones = [], onRemove = () => {} }: Props = $props();
</script>

<table class="pure-table pure-table-bordered">
    <thead>
        <tr>
            <th>From</th>
            <th>Date</th>
            <th>Duration</th>
            <th>Area</th>
            <th>Diameter</th>
            <th></th>
        </tr>
    </thead>
    <tbody>
        {#each isochrones as iso, i}
            <IsochroneRow isochroneCall={iso} index={i} onRemove={onRemove} />
        {/each}
    </tbody>
</table>

<style>
    table {
        height: fit-content;
        max-height: 100%;
        width: fit-content;
        max-width: 100%;
    }

    th {
        background-color: #0078e7;
        color: white;
    }
</style>
