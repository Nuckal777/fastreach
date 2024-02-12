<script lang="ts">
    import "leaflet/dist/leaflet.css";
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
    export let lng = 0;
    export let lat = 0;
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
            control.attribution({ position: "bottomright", prefix: false }),
        );
        map.addControl(control.zoom({ position: "bottomleft" }));
        map.setView({ lat, lng }, 11);
        map.on("moveend", () => {
            if (map === null) {
                return;
            }
            lat = map.getCenter().lat;
            lng = map.getCenter().lng;
        });
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

    $: setView(lat, lng);

    function setView(localLat: number, localLng: number) {
        if (map === null) {
            return;
        }
        map.setView({ lat: localLat, lng: localLng });
    }
</script>

<div style="height:100%; width:100%;" use:initialize>
    {#if map}
        <slot />
    {/if}
</div>
