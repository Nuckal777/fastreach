<script lang="ts">
    import IsochroneForm from "./IsochroneForm.svelte";
    import StationSelect from "./StationSelect.svelte";
    import type { NodeResponse, Node, IsochroneCallHandler, IsochroneConfiguration, IsochroneResponse } from "./types";

    export let useIsochrone: IsochroneCallHandler = () => {};

    let nodes: Node[] = [];
    let ambiguousConfig: IsochroneConfiguration | undefined;

    let nodesPromise = fetchNodes();
    async function fetchNodes() {
        const res = await fetch("/nodes.json.br");
        nodes = (await res.json()) as NodeResponse;
    }

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
            name: target.name
        });
    }
</script>

{#await nodesPromise}
    <p>Loading...</p>
{:then}
    {#if ambiguousConfig === undefined}
        <IsochroneForm nodes={nodes} useNodes={useNodes}></IsochroneForm>
    {:else}
        <StationSelect config={ambiguousConfig} useConfiguration={doPost}></StationSelect>
    {/if}
{/await}
