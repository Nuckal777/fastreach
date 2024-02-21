<script lang="ts">
    import "leaflet/dist/leaflet.css";
    import type { GeoJsonObject } from "geojson";

    import L, { control } from "leaflet";
    import { mapLocation } from "./store";

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
        map = L.map(container, {
            zoomControl: false,
            attributionControl: false,
        });
        map.addControl(
            control.attribution({ position: "bottomright", prefix: false })
        );
        map.addControl(control.zoom({ position: "bottomleft" }));
        map.setView(
            { lat: $mapLocation.lat, lng: $mapLocation.lng },
            $mapLocation.zoom
        );
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

    $: map?.setView(
        { lat: $mapLocation.lat, lng: $mapLocation.lng },
        $mapLocation.zoom
    );
</script>

<div style="height:100%; width:100%;" use:initialize>
    {#if map}
        <slot />
    {/if}
</div>
