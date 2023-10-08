<script lang="ts">
    import type { IsochroneCall } from "./types";

    export let isochroneCall: IsochroneCall;
</script>

<tr>
    <td>{isochroneCall.name}</td>
    <td
        >{isochroneCall.request.start.toLocaleDateString()}
        {isochroneCall.request.start.toLocaleTimeString()}</td
    >
    <td>{isochroneCall.request.minutes} min</td>
    <td>{isochroneCall.response.area.toFixed(3)} km²</td>
    <td>{isochroneCall.response.diameter.toFixed(3)} km</td>
</tr>

<style>
    tr:nth-child(5n + 1) {
        color: #0078e7ff;
    }
    tr:nth-child(5n + 2) {
        color: #f98948ff;
    }
    tr:nth-child(5n + 3) {
        color: #2a7f62ff;
    }
    tr:nth-child(5n + 4) {
        color: #802392ff;
    }
    tr:nth-child(5n) {
        color: #a31621ff;
    }
</style>
