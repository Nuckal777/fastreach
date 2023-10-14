<script lang="ts">
    import IsochroneForm from "./IsochroneForm.svelte";
    import StationSelect from "./StationSelect.svelte";
    import { nodes } from "./store";
    import type {
        IsochroneCallHandler,
        IsochroneConfiguration,
        IsochroneResponse,
    } from "./types";

    export let useIsochrone: IsochroneCallHandler = () => {};

    let ambiguousConfig: IsochroneConfiguration | undefined;

    let station = "Erfurt Hbf";
    let minutes = 30;
    let start = "2023-09-26T10:15:00";

    async function useNodes(config: IsochroneConfiguration) {
        if (config.nodes.length === 0) {
            ambiguousConfig = undefined;
            return;
        }
        if (config.nodes.length === 1) {
            ambiguousConfig = undefined;
            await doPost(config);
            return;
        }
        if (config.nodes.length > 200) {
            config.nodes = config.nodes.slice(0, 200);
        }
        ambiguousConfig = config;
    }

    async function doPost(config: IsochroneConfiguration) {
        ambiguousConfig = undefined;
        if (config.nodes.length !== 1) {
            return;
        }
        const target = config.nodes[0];
        const res = await fetch("/api/v1/isochrone", {
            method: "POST",
            body: JSON.stringify({
                id: target.id,
                minutes: config.minutes,
                start: config.start.getTime(),
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
                minutes: config.minutes,
                id: target.id,
                start: config.start,
            },
            response: result,
            name: target.name,
        });
    }
</script>

{#if $nodes.length === 0}
    <p>Loading...</p>
{:else if ambiguousConfig === undefined}
    <IsochroneForm
        nodes={$nodes}
        {useNodes}
        bind:station
        bind:minutes
        bind:start
    />
{:else}
    <StationSelect config={ambiguousConfig} useConfiguration={doPost} />
{/if}
