<script lang="ts">
    import type { IsochroneCall } from "./types";

    export let isochroneCall: IsochroneCall;
</script>

<div>
    <p>
        From {isochroneCall.request.name} at {isochroneCall.request.start.toLocaleDateString()}
        {isochroneCall.request.start.toLocaleTimeString()}
        for {isochroneCall.request.minutes} minutes:
    </p>
    <p>
        Area: {isochroneCall.response.area} km²
    </p>
    <p>
        Diameter: {isochroneCall.response.diameter} km
    </p>
    <hr />
</div>
