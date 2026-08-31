<script lang="ts">
    import { onMount } from "svelte";
    import type { DatasetResponse, DatasetResponseState, IndexedDataset, IndexedDatasetResponseState } from "./types";

    let datasets: IndexedDatasetResponseState = $state({datasets: [], error: ""})

    interface Props {
        useDataset: (ds: IndexedDataset) => void;
    }

    let { useDataset = () => {} }: Props = $props();

    function clickSelect(idx: number) {
        useDataset(datasets.datasets[idx])
    }

    async function fetchDatasets() {
        const res = await fetch("/api/v1/datasets");
        if (!res.ok) {
            datasets = {
                datasets: [],
                error: `HTTP request failed, status code: ${res.status}`,
            };
            return;
        }
        const datasetsRes = (await res.json()) as DatasetResponse;
        let indexed: IndexedDataset[] = datasetsRes.datasets.map((ds, idx) => ({
            edges: ds.edges,
            from: new Date(ds.from),
            index: idx,
            name: ds.name,
            nodes: ds.nodes,
            to: new Date(ds.to),
        }));
        datasets = { datasets: indexed, error: "" };
    }

    onMount(fetchDatasets);
</script>

<p>Select a dataset</p>
{#if datasets.error !== ""}
    <p>{datasets.error}</p>
{:else if datasets.datasets.length === 0}
    <p>Loading...</p>
{:else}
    {#each datasets.datasets as ds, idx}
       <hr />
       <h3>{ds.name} </h3> 
       Stations: {ds.nodes} Edges: {ds.edges} Valid: {ds.from.toDateString()} - {ds.to.toDateString()}
       <div>
       <input type="button" class="pure-button pure-button-primary" value="Select" onclick={() => clickSelect(idx)} />
       </div>
    {/each}
{/if}
