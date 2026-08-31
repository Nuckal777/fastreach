<script lang="ts">
    import { onMount } from "svelte";
    import Back from "./Back.svelte";
    import IsochroneForm from "./IsochroneForm.svelte";
    import StationSelect from "./StationSelect.svelte";
    import { mapLocation } from "./store";
    import type {
    IndexedDataset,
        IsochroneCallHandler,
        IsochroneConfiguration,
        IsochroneResponse,
        Node,
        NodeResponse,
    } from "./types";

    interface Props {
        dataset: IndexedDataset,
        useIsochrone?: IsochroneCallHandler;
    }

    let {
        dataset = {edges: 0, from: new Date(), index: -1, name: "", nodes: 0, to: new Date()},
        useIsochrone = () => {},
    }: Props = $props();

    let ambiguousConfig: IsochroneConfiguration | undefined = $state();
    let error = $state("");

    let station = $state("Erfurt Hbf");
    let minutes = $state(30);
    const pad = (n: number) => String(n).padStart(2, '0');
    // svelte-ignore state_referenced_locally
    let initialStart = `${dataset.from.getFullYear()}-${pad(dataset.from.getMonth() + 1)}-${pad(dataset.from.getDate())}`
    console.log(initialStart);
    let start = $state(`${initialStart}T10:15:00`);
    let jump = $state(true);
    let nodes: Node[] = $state.raw([]);

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
        let matchIdx = config.nodes.findIndex((n) => n.name === station);
        if (matchIdx !== -1) {
            let match = config.nodes[matchIdx];
            config.nodes[matchIdx] = config.nodes[0];
            config.nodes[0] = match;
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
                dataset: dataset.index,
            }),
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (res.status !== 200) {
            if (res.status === 429) {
                error = "Please try again later.";
            } else {
                error = `HTTP request failed, status code: ${res.status}.`;
            }
            return;
        }
        const result = (await res.json()) as IsochroneResponse;
        if (jump) {
            mapLocation.update((loc) => ({
                lng: target.coords[0],
                lat: target.coords[1],
                zoom: loc.zoom,
            }));
        }
        useIsochrone({
            request: {
                minutes: config.minutes,
                id: target.id,
                start: config.start,
            },
            response: result,
            name: target.name,
            lng: target.coords[0],
            lat: target.coords[1],
        });
    }

    async function fetchNodes() {
        const res = await fetch(`/api/v1/nodes/${dataset.index}`);
        if (res.status !== 200) {
            if (res.status === 429) {
                error = "Please try again later.";
            } else {
                error = `HTTP request failed, status code: ${res.status}.`;
            }
            return;
        }
        const nodeRes = (await res.json()) as NodeResponse;
        nodes = nodeRes;
    }

    onMount(fetchNodes);
</script>

{#if nodes.length === 0}
    <p>Loading...</p>
{:else if error !== ""}
    <Back onBack={() => (error = "")}>
        <p>{error}</p>
    </Back>
{:else if ambiguousConfig === undefined}
    <IsochroneForm
        nodes={nodes}
        {useNodes}
        bind:station
        bind:minutes
        bind:start
        bind:jump
    />
{:else}
    <Back onBack={() => (ambiguousConfig = undefined)}>
        <StationSelect config={ambiguousConfig} useConfiguration={doPost} />
    </Back>
{/if}
