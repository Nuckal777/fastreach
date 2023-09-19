<script lang="ts">
    import type { GeoJsonObject } from "geojson";

    import L, { control } from "leaflet";

    let map: L.Map | null = null;
    let geo: L.GeoJSON = L.geoJSON();
    export let geometries: GeoJsonObject[] = [];
    $: updateGeoLayer(geometries);

    function updateGeoLayer(geometries: GeoJsonObject[]) {
        if (map === null) {
            return;
        }
        map.removeLayer(geo);
        geo = L.geoJson(geometries);
        geo.addTo(map);
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
        geo.addTo(map);
        console.log("hurra");
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
