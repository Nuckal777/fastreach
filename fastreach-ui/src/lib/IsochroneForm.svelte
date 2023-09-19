<script lang="ts">
    import type { IsochroneResponse, IsochroneResponseHandler } from "./types";

    export let useIsochrone: IsochroneResponseHandler = () => {};
    let station = "Erfurt Hbf";
    let minutes = 30;
    let start = "2022-07-23T10:15:00";

    async function doPost() {
        const res = await fetch("/api/v1/isochrone", {
            method: "POST",
            body: JSON.stringify({
                name: station,
                minutes: minutes,
                start: new Date(start).getTime(),
            }),
            headers: {
                "Content-Type": "application/json",
            },
        });

        const result = (await res.json()) as IsochroneResponse;
        useIsochrone(result);
    }
</script>

<form>
    <label for="station">Station: </label>
    <input type="text" name="station" id="station" bind:value={station} />
    <label for="minutes">Minutes of travel: </label>
    <input
        type="range"
        name="minutes"
        id="minutes"
        min="1"
        max="120"
        bind:value={minutes}
    />
    {minutes} Minuten
    <label for="start">Start date: </label>
    <input type="datetime-local" name="start" id="start" bind:value={start} />
    <input type="button" value="Do request" on:click={doPost} />
</form>
