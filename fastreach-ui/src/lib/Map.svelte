<script lang="ts">
    import type { GeoJsonObject } from "geojson";

    import L, { control } from "leaflet";

    const isoOpts: L.GeoJSONOptions[] = [
        "#0078e7ff",
        "#f98948ff",
        "#2a7f62ff",
        "#802392ff",
        "#a31621ff",
    ].map((c) => {
        return { style: { color: c } };
    });
    let map: L.Map | null = null;
    let geos: L.GeoJSON[] = [];
    export let geometries: GeoJsonObject[] = [];
    $: updateGeoLayer(geometries);

    function updateGeoLayer(geometries: GeoJsonObject[]) {
        if (map === null) {
            return;
        }
        for (let g of geos) {
            map.removeLayer(g);
        }
        geos = [];
        for (const [i, g] of geometries.entries()) {
            const layer = L.geoJSON(g, isoOpts[i % isoOpts.length]);
            layer.addTo(map);
            geos.push(layer);
        }
    }

    function initialize(container: string | HTMLElement) {
        map = L.map(container, { zoomControl: false });
        map.addControl(control.zoom({ position: "bottomleft" }));
        map.setView({ lat: 50.97, lng: 11.035 }, 11);
        L.tileLayer("https://tile.openstreetmap.org/{z}/{x}/{y}.png", {
            maxZoom: 15,
            attribution:
                '&copy; <a href="https://openstreetmap.org/copyright">OpenStreetMap contributors</a>',
        }).addTo(map);
        return {
            destroy: () => {
                if (map !== null) {
                    map.remove();
                }
            },
        };
    }
</script>

<div style="height:100%; width:100%;" use:initialize>
    {#if map}
        <slot />
    {/if}
</div>

<link
    rel="stylesheet"
    href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
    integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
    crossorigin=""
/>
