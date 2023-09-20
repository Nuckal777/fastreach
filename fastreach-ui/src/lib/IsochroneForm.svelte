<script lang="ts">
    import type { IsochroneResponse, IsochroneCallHandler } from "./types";

    export let useIsochrone: IsochroneCallHandler = () => {};
    let station = "Erfurt Hbf";
    let minutes = 30;
    let start = "2022-07-23T10:15:00";

    async function doPost() {
        const startDate = new Date(start);
        const res = await fetch("/api/v1/isochrone", {
            method: "POST",
            body: JSON.stringify({
                name: station,
                minutes: minutes,
                start: startDate.getTime(),
            }),
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (res.status !== 200) {
            console.log(`POST isochrone failed: ${res.status}`);
            return;
        }
        const result = (await res.json()) as IsochroneResponse;
        useIsochrone({
            request: {
                minutes: minutes,
                name: station,
                start: startDate,
            },
            response: result,
        });
    }
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
                value="Calculate isochrone"
                on:click={doPost}
            />
        </div>
    </fieldset>
</form>

<style>
    .fill-width {
        width: 13em;
    }
</style>
